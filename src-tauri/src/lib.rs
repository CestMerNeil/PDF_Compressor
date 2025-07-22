use serde::{Deserialize, Serialize};
use lopdf::Document;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::Emitter;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CompressionResult {
    success: bool,
    message: String,
}

// 全局状态，用于跟踪 Ghostscript 的下载和安装状态
lazy_static::lazy_static! {
    static ref GHOSTSCRIPT_STATE: Arc<Mutex<GhostscriptState>> = Arc::new(Mutex::new(GhostscriptState {
        is_installed: false,
        is_downloading: false,
        download_progress: 0.0,
        executable_path: None,
    }));
}

#[derive(Debug, Clone)]
struct GhostscriptState {
    is_installed: bool,
    is_downloading: bool,
    download_progress: f32,
    executable_path: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GhostscriptStatus {
    is_installed: bool,
    is_downloading: bool,
    download_progress: f32,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn select_input_file(app_handle: tauri::AppHandle) -> Result<String, String> {
    use tauri_plugin_dialog::DialogExt;
    use tokio::sync::oneshot;
    
    let (tx, rx) = oneshot::channel();
    
    app_handle.dialog()
        .file()
        .add_filter("PDF 文件", &["pdf"])
        .pick_file(move |file_path| {
            let _ = tx.send(file_path);
        });
    
    // 等待用户选择文件
    match tokio::time::timeout(std::time::Duration::from_secs(30), rx).await {
        Ok(Ok(Some(path))) => Ok(path.to_string()),
        Ok(Ok(None)) => Err("未选择文件".to_string()),
        Ok(Err(_)) => Err("内部错误".to_string()),
        Err(_) => Err("选择文件超时".to_string()),
    }
}

#[tauri::command]
async fn select_output_path(app_handle: tauri::AppHandle) -> Result<String, String> {
    use tauri_plugin_dialog::DialogExt;
    use tokio::sync::oneshot;
    
    let (tx, rx) = oneshot::channel();
    
    app_handle.dialog()
        .file()
        .add_filter("PDF 文件", &["pdf"])
        .save_file(move |file_path| {
            let _ = tx.send(file_path);
        });
    
    // 等待用户选择保存位置
    match tokio::time::timeout(std::time::Duration::from_secs(30), rx).await {
        Ok(Ok(Some(path))) => Ok(path.to_string()),
        Ok(Ok(None)) => Err("未选择保存位置".to_string()),
        Ok(Err(_)) => Err("内部错误".to_string()),
        Err(_) => Err("选择保存位置超时".to_string()),
    }
}

#[tauri::command]
async fn compress_pdf(input_path: String, output_path: String, compression_level: String) -> Result<CompressionResult, String> {
    
    // 检查输入文件是否存在
    if !std::path::Path::new(&input_path).exists() {
        return Err("输入文件不存在".to_string());
    }

    // 检查输出目录是否存在，如果不存在则创建
    if let Some(parent) = std::path::Path::new(&output_path).parent() {
        if !parent.exists() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                return Err(format!("无法创建输出目录: {}", e));
            }
        }
    }

    // 首先检查 Ghostscript 是否可用
    if is_ghostscript_available() {
        // 使用 Ghostscript 进行高效压缩
        compress_with_ghostscript(&input_path, &output_path, &compression_level).await
    } else {
        // 回退到 lopdf + 增强优化
        compress_with_enhanced_lopdf(&input_path, &output_path, &compression_level).await
    }
}

fn is_ghostscript_available() -> bool {
    // 首先检查系统是否已安装 Ghostscript
    let commands = ["gs", "gswin64c", "gswin32c"]; // 支持不同平台的 gs 命令
    
    for cmd in &commands {
        if let Ok(output) = std::process::Command::new(cmd)
            .arg("--version")
            .output()
        {
            if output.status.success() {
                return true;
            }
        }
    }
    
    // 检查是否有捆绑的 Ghostscript
    if let Ok(bundled_path) = get_bundled_ghostscript_path() {
        if std::path::Path::new(&bundled_path).exists() {
            return true;
        }
    }
    
    false
}

fn get_bundled_ghostscript_path() -> Result<String, String> {
    let app_data_dir = get_app_data_dir()?;
    let gs_dir = app_data_dir.join("ghostscript");
    
    let gs_executable = if cfg!(target_os = "windows") {
        "gs.exe"
    } else {
        "gs"
    };
    
    let gs_path = gs_dir.join(gs_executable);
    Ok(gs_path.to_string_lossy().to_string())
}

async fn compress_with_ghostscript(input_path: &str, output_path: &str, compression_level: &str) -> Result<CompressionResult, String> {
    use std::fs;
    
    // 获取原始文件大小
    let original_size = fs::metadata(input_path)
        .map_err(|e| format!("无法读取原始文件信息: {}", e))?
        .len();

    // 根据压缩等级设置 Ghostscript 参数
    let (pdf_settings, additional_args) = match compression_level {
        "/screen" => ("/screen", vec![
            "-dColorImageResolution=72",
            "-dGrayImageResolution=72", 
            "-dMonoImageResolution=300",
            "-dColorImageDownsampleType=/Bicubic",
            "-dGrayImageDownsampleType=/Bicubic",
            "-dColorImageDownsampleThreshold=1.5",
            "-dGrayImageDownsampleThreshold=1.5",
            "-dEncodeColorImages=true",
            "-dEncodeGrayImages=true",
            "-dColorImageFilter=/DCTEncode",
            "-dGrayImageFilter=/DCTEncode",
            "-dJPEGQ=30",
        ]),
        "/ebook" => ("/ebook", vec![
            "-dColorImageResolution=150",
            "-dGrayImageResolution=150",
            "-dMonoImageResolution=300",
            "-dColorImageDownsampleType=/Bicubic",
            "-dGrayImageDownsampleType=/Bicubic",
            "-dJPEGQ=50",
        ]),
        "/printer" => ("/printer", vec![
            "-dColorImageResolution=300",
            "-dGrayImageResolution=300",
            "-dMonoImageResolution=1200",
            "-dJPEGQ=80",
        ]),
        "/prepress" => ("/prepress", vec![
            "-dColorImageResolution=300",
            "-dGrayImageResolution=300", 
            "-dMonoImageResolution=1200",
            "-dJPEGQ=90",
            "-dPreserveAnnots=true",
            "-dPreserveMarkedContent=true",
        ]),
        _ => ("/ebook", vec!["-dJPEGQ=50"]),
    };

    // 构建 Ghostscript 命令
    let gs_command = find_ghostscript_command();
    let mut cmd = Command::new(&gs_command);
    
    cmd.args(&[
        "-sDEVICE=pdfwrite",
        "-dCompatibilityLevel=1.4",
        &format!("-dPDFSETTINGS={}", pdf_settings),
        "-dNOPAUSE",
        "-dQUIET", 
        "-dBATCH",
        "-dSAFER",
        "-dAutoRotatePages=/None",
        "-dColorConversionStrategy=/LeaveColorUnchanged",
        "-dDownsampleColorImages=true",
        "-dDownsampleGrayImages=true",
        "-dDownsampleMonoImages=true",
        "-dOptimize=true",
        "-dEmbedAllFonts=true",
        "-dSubsetFonts=true",
        "-dCompressFonts=true",
        "-dNOPLATFONTS",
    ]);
    
    // 添加额外参数
    cmd.args(&additional_args);
    
    // 添加输出和输入文件
    cmd.args(&[
        &format!("-sOutputFile={}", output_path),
        input_path,
    ]);

    // 执行压缩
    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                // 计算压缩比
                if let Ok(compressed_size) = fs::metadata(output_path).map(|m| m.len()) {
                    let compression_ratio = ((original_size as f64 - compressed_size as f64) / original_size as f64) * 100.0;
                    let size_reduction = format_file_size(original_size - compressed_size);
                    
                    Ok(CompressionResult {
                        success: true,
                        message: format!(
                            "PDF 压缩成功！压缩率: {:.1}% (节省 {})", 
                            compression_ratio, 
                            size_reduction
                        ),
                    })
                } else {
                    Ok(CompressionResult {
                        success: true,
                        message: "PDF 压缩成功！".to_string(),
                    })
                }
            } else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(format!("Ghostscript 压缩失败: {}", error_msg))
            }
        }
        Err(e) => Err(format!("执行 Ghostscript 失败: {}", e)),
    }
}

async fn compress_with_enhanced_lopdf(input_path: &str, output_path: &str, compression_level: &str) -> Result<CompressionResult, String> {
    use std::fs;
    
    // 获取原始文件大小
    let original_size = fs::metadata(input_path)
        .map_err(|e| format!("无法读取原始文件信息: {}", e))?
        .len();

    // 使用增强的 lopdf 压缩
    match Document::load(input_path) {
        Ok(mut document) => {
            // 根据压缩等级设置不同的压缩参数
            let optimization_level = match compression_level {
                "/screen" => "aggressive",
                "/ebook" => "balanced", 
                "/printer" => "quality",
                "/prepress" => "maximum",
                _ => "balanced",
            };

            // 执行增强的压缩优化
            enhanced_pdf_optimization(&mut document, optimization_level);

            // 保存压缩后的PDF
            match document.save(output_path) {
                Ok(_) => {
                    // 计算压缩比
                    if let Ok(compressed_size) = fs::metadata(output_path).map(|m| m.len()) {
                        let compression_ratio = ((original_size as f64 - compressed_size as f64) / original_size as f64) * 100.0;
                        let size_reduction = format_file_size(original_size - compressed_size);
                        
                        Ok(CompressionResult {
                            success: true,
                            message: format!(
                                "PDF 压缩完成！压缩率: {:.1}% (节省 {}) - 注意：安装 Ghostscript 可获得更好的压缩效果", 
                                compression_ratio, 
                                size_reduction
                            ),
                        })
                    } else {
                        Ok(CompressionResult {
                            success: true,
                            message: "PDF 压缩完成！建议安装 Ghostscript 以获得更好的压缩效果".to_string(),
                        })
                    }
                }
                Err(e) => Err(format!("保存压缩后的PDF失败: {}", e)),
            }
        }
        Err(e) => Err(format!("无法加载PDF文件: {}", e)),
    }
}

fn find_ghostscript_command() -> String {
    // 首先检查系统安装的 Ghostscript
    let commands = ["gs", "gswin64c", "gswin32c"];
    
    for cmd in &commands {
        if let Ok(output) = std::process::Command::new(cmd)
            .arg("--version")
            .output()
        {
            if output.status.success() {
                return cmd.to_string();
            }
        }
    }
    
    // 如果系统没有安装，尝试使用捆绑的版本
    if let Ok(bundled_path) = get_bundled_ghostscript_path() {
        if std::path::Path::new(&bundled_path).exists() {
            return bundled_path;
        }
    }
    
    "gs".to_string() // 默认返回 gs
}

fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

fn enhanced_pdf_optimization(document: &mut Document, optimization_level: &str) {
    // 执行基本的 PDF 结构优化
    document.compress();
    
    // 根据优化级别进行不同程度的优化
    let object_ids: Vec<_> = document.objects.keys().copied().collect();
    
    for object_id in object_ids {
        if let Ok(object) = document.get_object(object_id) {
            match object {
                lopdf::Object::Stream(ref _stream) => {
                    // lopdf 的流对象优化能力有限
                    // 主要依赖 document.compress() 进行基础优化
                }
                lopdf::Object::Dictionary(ref _dict) => {
                    // 字典对象的基础优化
                    if matches!(optimization_level, "aggressive" | "balanced") {
                        // 可以在这里添加元数据清理等操作
                    }
                }
                _ => {}
            }
        }
    }
    
    // 执行垃圾回收，移除未引用的对象
    document.prune_objects();
}



fn get_app_data_dir() -> Result<PathBuf, String> {
    
    // 获取应用数据目录
    let app_data = if cfg!(target_os = "windows") {
        // 在Windows上，尝试使用LOCALAPPDATA而不是APPDATA，避免漫游配置文件的问题
        std::env::var("LOCALAPPDATA")
            .or_else(|_| std::env::var("APPDATA"))
            .map(PathBuf::from)
            .map_err(|_| "无法获取应用数据目录".to_string())?
    } else if cfg!(target_os = "macos") {
        std::env::var("HOME")
            .map(|home| PathBuf::from(home).join("Library").join("Application Support"))
            .map_err(|_| "无法获取用户目录".to_string())?
    } else {
        std::env::var("HOME")
            .map(|home| PathBuf::from(home).join(".local").join("share"))
            .map_err(|_| "无法获取用户目录".to_string())?
    };
    
    let app_dir = app_data.join("PDF_Compressor");
    
    // 确保目录存在
    if !app_dir.exists() {
        std::fs::create_dir_all(&app_dir)
            .map_err(|e| format!("无法创建应用数据目录: {}", e))?;
    }
    
    println!("应用数据目录: {}", app_dir.to_string_lossy());
    
    Ok(app_dir)
}

async fn extract_ghostscript_binary(target_path: &std::path::Path) -> Result<(), String> {
    // 自动下载并安装 Ghostscript
    let gs_dir = target_path.parent().unwrap();
    
    // 确保目录存在
    if !gs_dir.exists() {
        std::fs::create_dir_all(gs_dir)
            .map_err(|e| format!("创建 Ghostscript 目录失败: {}", e))?;
    }
    
    println!("Ghostscript 安装目录: {}", gs_dir.to_string_lossy());
    
    // 根据操作系统确定下载URL和处理方式
    let (download_url, is_archive) = get_ghostscript_download_info()?;
    
    println!("下载 URL: {}", download_url);
    println!("是否为压缩包: {}", is_archive);
    
    // 更新全局状态，表示开始下载
    {
        let mut gs_state = GHOSTSCRIPT_STATE.lock().await;
        gs_state.is_downloading = true;
        gs_state.download_progress = 0.0;
    }
    
    // 下载 Ghostscript
    println!("开始下载 Ghostscript...");
    let downloaded_data = download_file(&download_url).await?;
    println!("下载完成，数据大小: {} 字节", downloaded_data.len());
    
    // 更新全局状态，表示下载完成，开始安装
    {
        let mut gs_state = GHOSTSCRIPT_STATE.lock().await;
        gs_state.download_progress = 90.0; // 设置为90%，表示下载完成，开始安装
    }
    
    println!("开始安装 Ghostscript...");
    
    if is_archive {
        // 解压缩档案文件
        println!("解压缩档案文件...");
        extract_archive(&downloaded_data, gs_dir, target_path)?;
    } else {
        if cfg!(target_os = "windows") {
            // Windows 使用安装程序
            println!("使用 Windows 安装程序...");
            extract_windows_installer(&downloaded_data, gs_dir, target_path)?;
        } else {
            // 直接保存二进制文件
            println!("保存二进制文件到: {}", target_path.to_string_lossy());
            std::fs::write(target_path, downloaded_data)
                .map_err(|e| format!("保存 Ghostscript 失败: {}", e))?;
        }
    }
    
    println!("安装完成");
    
    // 更新全局状态，表示安装完成
    {
        let mut gs_state = GHOSTSCRIPT_STATE.lock().await;
        gs_state.download_progress = 100.0;
    }
    
    Ok(())
}

fn get_ghostscript_download_info() -> Result<(String, bool), String> {
    // 根据操作系统返回对应的下载链接
    if cfg!(target_os = "windows") {
        if cfg!(target_arch = "x86_64") {
            // 使用官方最新版本的下载链接
            Ok((
                "https://github.com/ArtifexSoftware/ghostpdl-downloads/releases/download/gs10021/gs10021w64.exe".to_string(),
                false
            ))
        } else {
            Ok((
                "https://github.com/ArtifexSoftware/ghostpdl-downloads/releases/download/gs10021/gs10021w32.exe".to_string(),
                false
            ))
        }
    } else if cfg!(target_os = "macos") {
        // macOS 使用 Homebrew 提供的预编译版本
        // 由于直接下载二进制文件比较复杂，我们使用一个简化的方案
        // 提示用户手动安装或使用系统已安装的版本
        Err("macOS 用户请使用 'brew install ghostscript' 安装 Ghostscript".to_string())
    } else {
        // Linux 用户也建议使用包管理器安装
        Err("Linux 用户请使用包管理器安装 Ghostscript，如: sudo apt install ghostscript".to_string())
    }
}

async fn download_file(url: &str) -> Result<Vec<u8>, String> {
    use reqwest;
    use futures_util::StreamExt;
    
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("下载失败: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("下载失败，HTTP状态码: {}", response.status()));
    }
    
    // 获取内容长度（如果有）
    let total_size = response.content_length().unwrap_or(0);
    
    // 使用流式下载以便可以报告进度
    let mut stream = response.bytes_stream();
    let mut downloaded: u64 = 0;
    let mut bytes = Vec::new();
    
    // 获取全局状态的可变引用
    let mut gs_state = GHOSTSCRIPT_STATE.lock().await;
    
    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|e| format!("下载过程中出错: {}", e))?;
        downloaded += chunk.len() as u64;
        bytes.extend_from_slice(&chunk);
        
        // 更新下载进度
        if total_size > 0 {
            let progress = (downloaded as f32 / total_size as f32) * 100.0;
            gs_state.download_progress = progress;
            
            // 进度更新通过全局状态在其他地方处理
        }
    }
    
    Ok(bytes)
}

fn extract_archive(data: &[u8], extract_dir: &std::path::Path, target_path: &std::path::Path) -> Result<(), String> {
    
    if cfg!(target_os = "windows") {
        // Windows 的 .exe 安装包需要特殊处理
        // 这里我们使用一个简化的方案：直接将下载的安装包保存，然后静默安装
        extract_windows_installer(data, extract_dir, target_path)
    } else {
        // Unix 系统使用 tar.gz 解压
        extract_tar_gz(data, extract_dir, target_path)
    }
}

fn extract_windows_installer(data: &[u8], extract_dir: &std::path::Path, target_path: &std::path::Path) -> Result<(), String> {
    use std::process::Command;
    use std::fs;
    use std::io::Write;
    
    // 确保目录存在
    if !extract_dir.exists() {
        fs::create_dir_all(extract_dir)
            .map_err(|e| format!("创建目录失败: {}", e))?;
    }
    
    // 保存安装包到临时位置
    let installer_path = extract_dir.join("gs_installer.exe");
    fs::write(&installer_path, data)
        .map_err(|e| format!("保存安装包失败: {}", e))?;
    
    // 创建一个批处理文件来执行安装，避免路径问题
    let batch_path = extract_dir.join("install_gs.bat");
    let install_dir = extract_dir.to_string_lossy().replace("\\", "\\\\");
    let batch_content = format!(
        "@echo off\r\n\
         \"{}\" /S /D=\"{}\"\r\n\
         exit %errorlevel%",
        installer_path.to_string_lossy(),
        install_dir
    );
    
    let mut batch_file = fs::File::create(&batch_path)
        .map_err(|e| format!("创建批处理文件失败: {}", e))?;
    batch_file.write_all(batch_content.as_bytes())
        .map_err(|e| format!("写入批处理文件失败: {}", e))?;
    
    // 执行批处理文件
    let output = Command::new("cmd")
        .args(&["/C", batch_path.to_string_lossy().as_ref()])
        .output()
        .map_err(|e| format!("执行安装失败: {}", e))?;
    
    // 输出安装日志以便调试
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("安装输出: {}", stdout);
    println!("安装错误: {}", stderr);
    
    if !output.status.success() {
        return Err(format!("Ghostscript 安装失败，错误码: {}", output.status));
    }
    
    // 查找安装后的 gs.exe 文件
    // 首先在标准位置查找
    let standard_paths = [
        extract_dir.join("bin").join("gswin64c.exe"),
        extract_dir.join("bin").join("gswin32c.exe"),
        extract_dir.join("gs").join("bin").join("gswin64c.exe"),
        extract_dir.join("gs").join("bin").join("gswin32c.exe"),
        extract_dir.join("gs10021").join("bin").join("gswin64c.exe"),
        extract_dir.join("gs10021").join("bin").join("gswin32c.exe"),
    ];
    
    // 尝试查找安装的可执行文件
    for path in &standard_paths {
        if path.exists() {
            println!("找到 Ghostscript 可执行文件: {}", path.to_string_lossy());
            fs::copy(path, target_path)
                .map_err(|e| format!("复制 Ghostscript 可执行文件失败: {}", e))?;
            
            // 清理临时文件
            let _ = fs::remove_file(&installer_path);
            let _ = fs::remove_file(&batch_path);
            
            return Ok(());
        }
    }
    
    // 如果在标准位置找不到，尝试递归查找
    if let Some(found_path) = find_gs_executable_recursive(extract_dir) {
        println!("在非标准位置找到 Ghostscript: {}", found_path.to_string_lossy());
        fs::copy(&found_path, target_path)
            .map_err(|e| format!("复制 Ghostscript 可执行文件失败: {}", e))?;
        
        // 清理临时文件
        let _ = fs::remove_file(&installer_path);
        let _ = fs::remove_file(&batch_path);
        
        return Ok(());
    }
    
    // 如果找不到，尝试直接复制安装程序作为备用方案
    println!("未找到 Ghostscript 可执行文件，使用安装程序作为备用");
    fs::copy(&installer_path, target_path)
        .map_err(|e| format!("复制安装程序失败: {}", e))?;
    
    // 清理临时文件
    let _ = fs::remove_file(&batch_path);
    
    Ok(())
}

// 递归查找 Ghostscript 可执行文件
fn find_gs_executable_recursive(dir: &std::path::Path) -> Option<std::path::PathBuf> {
    use std::fs;
    
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            
            // 检查是否是我们要找的可执行文件
            if path.is_file() {
                let file_name = path.file_name()?.to_string_lossy().to_lowercase();
                if file_name == "gswin64c.exe" || file_name == "gswin32c.exe" || file_name == "gs.exe" {
                    return Some(path);
                }
            } else if path.is_dir() {
                // 递归查找子目录
                if let Some(found) = find_gs_executable_recursive(&path) {
                    return Some(found);
                }
            }
        }
    }
    
    None
}

fn extract_tar_gz(data: &[u8], extract_dir: &std::path::Path, target_path: &std::path::Path) -> Result<(), String> {
    use std::io::Cursor;
    use flate2::read::GzDecoder;
    use tar::Archive;
    
    let cursor = Cursor::new(data);
    let gz_decoder = GzDecoder::new(cursor);
    let mut archive = Archive::new(gz_decoder);
    
    // 解压到临时目录
    archive.unpack(extract_dir)
        .map_err(|e| format!("解压失败: {}", e))?;
    
    // 查找 gs 可执行文件
    find_and_copy_gs_binary(extract_dir, target_path)?;
    
    Ok(())
}

fn find_and_copy_gs_binary(search_dir: &std::path::Path, target_path: &std::path::Path) -> Result<(), String> {
    use std::fs;
    
    // 递归查找 gs 可执行文件
    fn find_gs_recursive(dir: &std::path::Path) -> Option<std::path::PathBuf> {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(name) = path.file_name() {
                        if name == "gs" || name == "ghostscript" {
                            return Some(path);
                        }
                    }
                } else if path.is_dir() {
                    if let Some(found) = find_gs_recursive(&path) {
                        return Some(found);
                    }
                }
            }
        }
        None
    }
    
    if let Some(gs_path) = find_gs_recursive(search_dir) {
        fs::copy(&gs_path, target_path)
            .map_err(|e| format!("复制 Ghostscript 可执行文件失败: {}", e))?;
        Ok(())
    } else {
        Err("在解压的文件中找不到 Ghostscript 可执行文件".to_string())
    }
}

#[tauri::command]
async fn check_ghostscript_status() -> Result<GhostscriptStatus, String> {
    // 检查系统安装的 Ghostscript
    let system_gs_available = is_ghostscript_available();
    
    // 检查捆绑的 Ghostscript
    let bundled_gs_available = if let Ok(path) = get_bundled_ghostscript_path() {
        std::path::Path::new(&path).exists()
    } else {
        false
    };
    
    Ok(GhostscriptStatus {
        is_installed: system_gs_available || bundled_gs_available,
        is_downloading: false,
        download_progress: if system_gs_available || bundled_gs_available { 100.0 } else { 0.0 },
    })
}

#[tauri::command]
async fn get_manual_install_instructions() -> Result<String, String> {
    let instructions = if cfg!(target_os = "windows") {
        "Windows 用户安装说明：\n\
         1. 访问 https://www.ghostscript.com/releases/gsdnld.html\n\
         2. 下载适合您系统的版本（64位或32位）\n\
         3. 运行安装程序并按照提示完成安装\n\
         4. 重启应用以检测已安装的 Ghostscript"
    } else if cfg!(target_os = "macos") {
        "macOS 用户安装说明：\n\
         方法1 - 使用 Homebrew（推荐）：\n\
         1. 打开终端\n\
         2. 运行命令：brew install ghostscript\n\
         \n\
         方法2 - 手动安装：\n\
         1. 访问 https://www.ghostscript.com/releases/gsdnld.html\n\
         2. 下载 macOS 版本\n\
         3. 按照安装说明完成安装\n\
         4. 重启应用以检测已安装的 Ghostscript"
    } else {
        "Linux 用户安装说明：\n\
         Ubuntu/Debian：sudo apt install ghostscript\n\
         Fedora/RHEL：sudo dnf install ghostscript\n\
         Arch Linux：sudo pacman -S ghostscript\n\
         \n\
         安装完成后重启应用以检测已安装的 Ghostscript"
    };
    
    Ok(instructions.to_string())
}

#[tauri::command]
async fn uninstall_ghostscript() -> Result<bool, String> {
    use std::fs;
    
    // 获取应用数据目录
    let app_data_dir = get_app_data_dir()?;
    let gs_dir = app_data_dir.join("ghostscript");
    
    println!("卸载 Ghostscript，目录: {}", gs_dir.to_string_lossy());
    
    // 检查目录是否存在
    if !gs_dir.exists() {
        println!("Ghostscript 目录不存在，无需卸载");
        return Ok(false); // 没有安装，无需卸载
    }
    
    // 在 Windows 上，可能需要先关闭所有 Ghostscript 进程
    if cfg!(target_os = "windows") {
        use std::process::Command;
        
        println!("尝试关闭 Ghostscript 进程...");
        // 尝试关闭可能正在运行的 Ghostscript 进程
        let _ = Command::new("taskkill")
            .args(&["/F", "/IM", "gswin64c.exe"])
            .output();
        let _ = Command::new("taskkill")
            .args(&["/F", "/IM", "gswin32c.exe"])
            .output();
        let _ = Command::new("taskkill")
            .args(&["/F", "/IM", "gs.exe"])
            .output();
    }
    
    // 删除 Ghostscript 目录
    match fs::remove_dir_all(&gs_dir) {
        Ok(_) => {
            println!("成功删除 Ghostscript 目录");
            // 更新全局状态
            let mut gs_state = GHOSTSCRIPT_STATE.lock().await;
            gs_state.is_installed = false;
            gs_state.executable_path = None;
            
            Ok(true)
        },
        Err(e) => {
            println!("删除 Ghostscript 目录失败: {}", e);
            
            // 在 Windows 上，如果删除失败，尝试使用系统命令强制删除
            if cfg!(target_os = "windows") {
                println!("尝试使用系统命令强制删除...");
                use std::process::Command;
                
                let output = Command::new("cmd")
                    .args(&["/C", "rmdir", "/S", "/Q", &gs_dir.to_string_lossy()])
                    .output();
                
                match output {
                    Ok(output) if output.status.success() => {
                        println!("使用系统命令成功删除目录");
                        // 更新全局状态
                        let mut gs_state = GHOSTSCRIPT_STATE.lock().await;
                        gs_state.is_installed = false;
                        gs_state.executable_path = None;
                        
                        Ok(true)
                    },
                    Ok(_) => Err(format!("使用系统命令删除失败: {}", e)),
                    Err(e2) => Err(format!("卸载 Ghostscript 失败: {} (系统命令错误: {})", e, e2)),
                }
            } else {
                Err(format!("卸载 Ghostscript 失败: {}", e))
            }
        },
    }
}

#[tauri::command]
async fn download_ghostscript(app_handle: tauri::AppHandle) -> Result<GhostscriptStatus, String> {
    // 检查是否已经安装
    if is_ghostscript_available() {
        return Ok(GhostscriptStatus {
            is_installed: true,
            is_downloading: false,
            download_progress: 100.0,
        });
    }
    
    // 检查平台是否支持自动下载
    if !cfg!(target_os = "windows") {
        if cfg!(target_os = "macos") {
            return Err("macOS 用户请使用 'brew install ghostscript' 安装 Ghostscript，或从官网下载安装包".to_string());
        } else {
            return Err("Linux 用户请使用包管理器安装 Ghostscript，如: sudo apt install ghostscript".to_string());
        }
    }
    
    // 更新全局状态
    {
        let mut gs_state = GHOSTSCRIPT_STATE.lock().await;
        gs_state.is_downloading = true;
        gs_state.download_progress = 0.0;
    }
    
    // 在后台线程中下载和安装
    let app_handle_clone = app_handle.clone();
    tokio::spawn(async move {
        // 获取应用数据目录
        if let Ok(app_data_dir) = get_app_data_dir() {
            let gs_dir = app_data_dir.join("ghostscript");
            let gs_executable = "gs.exe"; // Windows only
            let gs_path = gs_dir.join(gs_executable);
            
            // 发送开始下载事件
            let _ = app_handle_clone.emit("ghostscript-download-start", true);
            
            // 下载和安装 Ghostscript
            let result = extract_ghostscript_binary(&gs_path).await;
            
            // 更新全局状态
            {
                let mut gs_state = GHOSTSCRIPT_STATE.lock().await;
                gs_state.is_downloading = false;
                
                if result.is_ok() {
                    gs_state.is_installed = true;
                    gs_state.download_progress = 100.0;
                    gs_state.executable_path = Some(gs_path);
                    
                    // 发送安装完成事件
                    let _ = app_handle_clone.emit("ghostscript-installed", true);
                } else {
                    // 发送安装失败事件
                    let _ = app_handle_clone.emit("ghostscript-install-failed", 
                        result.err().unwrap_or_else(|| "Unknown error".to_string()));
                }
            }
        }
    });
    
    // 返回当前状态
    Ok(GhostscriptStatus {
        is_installed: false,
        is_downloading: true,
        download_progress: 0.0,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet, 
            select_input_file, 
            select_output_path, 
            compress_pdf,
            check_ghostscript_status,
            download_ghostscript,
            get_manual_install_instructions,
            uninstall_ghostscript
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
