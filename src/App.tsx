import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

interface CompressionSettings {
  level: string;
  inputPath: string;
  outputPath: string;
}

interface GhostscriptStatus {
  is_installed: boolean;
  is_downloading: boolean;
  download_progress: number;
}

function App() {
  const [settings, setSettings] = useState<CompressionSettings>({
    level: "/ebook",
    inputPath: "",
    outputPath: "",
  });
  const [status, setStatus] = useState<string>("");
  const [isCompressing, setIsCompressing] = useState(false);
  const [theme, setTheme] = useState<string>("light");
  const [gsStatus, setGsStatus] = useState<GhostscriptStatus>({
    is_installed: false,
    is_downloading: false,
    download_progress: 0
  });

  // 初始化主题
  useEffect(() => {
    const savedTheme = localStorage.getItem('theme') || 'light';
    setTheme(savedTheme);
    document.documentElement.setAttribute('data-theme', savedTheme);
  }, []);
  
  // 检查 Ghostscript 状态
  async function checkGhostscriptStatus() {
    try {
      const status = await invoke<GhostscriptStatus>("check_ghostscript_status");
      setGsStatus(status);
    } catch (error) {
      console.error("检查 Ghostscript 状态失败:", error);
    }
  }
  
  // 下载 Ghostscript
  async function downloadGhostscript() {
    try {
      setStatus("正在准备下载 Ghostscript...");
      await invoke("download_ghostscript");
      setGsStatus(prev => ({...prev, is_downloading: true}));
    } catch (error) {
      console.error("下载 Ghostscript 失败:", error);
      
      // 检查是否是平台不支持的错误
      const errorMessage = String(error);
      if (errorMessage.includes("brew install") || errorMessage.includes("包管理器")) {
        setStatus(`${error}`);
      } else {
        setStatus(`下载 Ghostscript 失败: ${error}`);
      }
    }
  }
  
  // 卸载 Ghostscript
  async function uninstallGhostscript() {
    try {
      setStatus("正在卸载 Ghostscript...");
      const result = await invoke<boolean>("uninstall_ghostscript");
      if (result) {
        setGsStatus(prev => ({
          ...prev, 
          is_installed: false,
          download_progress: 0
        }));
        setStatus("Ghostscript 已成功卸载");
      } else {
        setStatus("Ghostscript 未安装，无需卸载");
      }
    } catch (error) {
      console.error("卸载 Ghostscript 失败:", error);
      setStatus(`卸载 Ghostscript 失败: ${error}`);
    }
  }
  
  // 显示手动安装指南
  async function showManualInstallGuide() {
    try {
      const instructions = await invoke<string>("get_manual_install_instructions");
      alert(instructions);
    } catch (error) {
      console.error("获取安装指南失败:", error);
    }
  }
  
  useEffect(() => {
    checkGhostscriptStatus();
    
    // 设置事件监听器
    const unlisten1 = listen('ghostscript-download-progress', (event) => {
      setGsStatus(prev => ({...prev, download_progress: event.payload as number}));
    });
    
    const unlisten2 = listen('ghostscript-installed', () => {
      setGsStatus({
        is_installed: true,
        is_downloading: false,
        download_progress: 100
      });
      setStatus("Ghostscript 安装成功！现在可以使用高级压缩功能。");
    });
    
    const unlisten3 = listen('ghostscript-install-failed', (event) => {
      setGsStatus(prev => ({...prev, is_downloading: false}));
      setStatus(`Ghostscript 安装失败: ${event.payload}`);
    });
    
    return () => {
      unlisten1.then(fn => fn());
      unlisten2.then(fn => fn());
      unlisten3.then(fn => fn());
    };
  }, []);

  // 主题切换函数
  const toggleTheme = (newTheme: string) => {
    setTheme(newTheme);
    document.documentElement.setAttribute('data-theme', newTheme);
    localStorage.setItem('theme', newTheme);
  };

  const compressionLevels = [
    { 
      value: "/screen", 
      label: "Screen 屏幕优化", 
      description: "72 DPI，JPEG质量30%，最高压缩比，文件体积最小",
      details: "适用于网络分享、邮件传输等场景"
    },
    { 
      value: "/ebook", 
      label: "eBook 电子书标准", 
      description: "150 DPI，JPEG质量50%，平衡压缩与质量",
      details: "推荐设置，适用于一般文档阅读和存档"
    },
    { 
      value: "/printer", 
      label: "Printer 打印级别", 
      description: "300 DPI，JPEG质量80%，保持打印质量",
      details: "适用于办公打印、文档输出等高质量需求"
    },
    { 
      value: "/prepress", 
      label: "Prepress 出版印刷", 
      description: "300+ DPI，JPEG质量90%+，专业印刷标准",
      details: "适用于商业印刷、出版物制作等专业用途"
    },
  ];

  async function selectInputFile() {
    try {
      setStatus("正在打开文件选择器...");
      
      // 使用 invoke 调用 Rust 后端的文件选择
      const selected = await invoke("select_input_file");
      
      if (selected && typeof selected === 'string' && selected.trim() !== '') {
        // 验证文件扩展名
        if (!selected.toLowerCase().endsWith('.pdf')) {
          setStatus("请选择有效的PDF文件");
          return;
        }
        setSettings(prev => ({ ...prev, inputPath: selected }));
        setStatus(`已选择文件: ${selected.split('/').pop()}`);
      } else {
        setStatus("未选择文件");
      }
    } catch (error) {
      console.error("选择文件时出错:", error);
      setStatus(`选择文件失败: ${error}`);
    }
  }

  async function selectOutputPath() {
    try {
      setStatus("正在打开保存对话框...");
      
      // 使用 invoke 调用 Rust 后端的保存对话框
      const selected = await invoke("select_output_path");
      
      if (selected && typeof selected === 'string' && selected.trim() !== '') {
        // 验证输出文件扩展名
        if (!selected.toLowerCase().endsWith('.pdf')) {
          const correctedPath = selected.endsWith('.') ? selected + 'pdf' : selected + '.pdf';
          setSettings(prev => ({ ...prev, outputPath: correctedPath }));
          setStatus(`已选择保存位置: ${correctedPath.split('/').pop()}`);
        } else {
          setSettings(prev => ({ ...prev, outputPath: selected }));
          setStatus(`已选择保存位置: ${selected.split('/').pop()}`);
        }
      } else {
        setStatus("未选择保存位置");
      }
    } catch (error) {
      console.error("选择输出路径时出错:", error);
      setStatus(`选择保存位置失败: ${error}`);
    }
  }

  async function startCompression() {
    if (!settings.inputPath) {
      setStatus("请先选择输入文件");
      return;
    }

    if (!settings.outputPath) {
      setStatus("请先选择输出路径");
      return;
    }

    setIsCompressing(true);
    setStatus("正在压缩中...");

    try {
      const result = await invoke("compress_pdf", {
        inputPath: settings.inputPath,
        outputPath: settings.outputPath,
        compressionLevel: settings.level,
      });
      
      console.log("压缩结果:", result);
      setStatus("压缩完成！");
    } catch (error) {
      console.error("压缩失败:", error);
      setStatus(`压缩失败: ${error}`);
    } finally {
      setIsCompressing(false);
    }
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-base-100 to-base-200">
      {/* 简约顶部栏 */}
      <div className="navbar bg-base-100/80 backdrop-blur-sm border-b border-base-300">
        <div className="navbar-start">
          <div className="flex items-center space-x-3">
            <div className="w-8 h-8 bg-primary rounded-lg flex items-center justify-center">
              <svg className="w-5 h-5 text-primary-content" fill="currentColor" viewBox="0 0 20 20">
                <path d="M4 3a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V5a2 2 0 00-2-2H4zm12 12H4l4-8 3 6 2-4 3 6z"/>
              </svg>
            </div>
            <h1 className="text-xl font-bold text-base-content">PDF Compressor</h1>
          </div>
        </div>
        <div className="navbar-end">
          <div className="dropdown dropdown-end">
            <div tabIndex={0} role="button" className="btn btn-ghost btn-sm btn-circle">
              <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="2">
                <path strokeLinecap="round" strokeLinejoin="round" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
              </svg>
            </div>
            <ul tabIndex={0} className="dropdown-content z-[1] menu p-2 shadow-lg bg-base-100 rounded-box w-40 border border-base-300">
              <li>
                <button 
                  onClick={() => toggleTheme('light')}
                  className={`text-sm ${theme === 'light' ? 'active' : ''}`}
                >
                  <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M10 2L13.09 8.26L20 9L14 14.74L15.18 21.02L10 18L4.82 21.02L6 14.74L0 9L6.91 8.26L10 2Z"/>
                  </svg>
                  浅色
                </button>
              </li>
              <li>
                <button 
                  onClick={() => toggleTheme('dark')}
                  className={`text-sm ${theme === 'dark' ? 'active' : ''}`}
                >
                  <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z"/>
                  </svg>
                  深色
                </button>
              </li>
            </ul>
          </div>
        </div>
      </div>

      {/* 主要内容区域 */}
      <div className="flex flex-col items-center justify-center min-h-[calc(100vh-80px)] p-6">
        <div className="w-full max-w-2xl space-y-8">
          
          {/* 标题区域 */}
          <div className="text-center space-y-2">
            <h2 className="text-3xl font-light text-base-content">PDF Compression</h2>
            <p className="text-base-content/60 text-sm">Professional PDF optimization with multiple quality presets</p>
          </div>

          {/* 主要操作卡片 */}
          <div className="card bg-base-100/50 backdrop-blur-sm shadow-xl border border-base-300">
            <div className="card-body p-8">
              <div className="space-y-6">
                {/* 文件输入区域 */}
                <div className="space-y-4">
                  <div className="form-control">
                    <div className="flex items-center justify-between mb-2">
                      <span className="text-sm font-medium text-base-content">Input File</span>
                    </div>
                    <div className="flex gap-3">
                      <input
                        type="text"
                        placeholder="Select PDF file to compress..."
                        className="input input-bordered flex-1 bg-base-100 border-base-300 focus:border-primary text-sm"
                        value={settings.inputPath ? settings.inputPath.split('/').pop() : ''}
                        readOnly
                      />
                      <button
                        className="btn btn-outline btn-primary min-h-0 h-12 px-6"
                        onClick={selectInputFile}
                        disabled={isCompressing}
                      >
                        Browse
                      </button>
                    </div>
                  </div>

                  {/* 压缩等级选择 */}
                  <div className="form-control">
                    <div className="flex items-center justify-between mb-3">
                      <span className="text-sm font-medium text-base-content">Compression Level</span>
                    </div>
                    <select
                      className="select select-bordered w-full bg-base-100 border-base-300 focus:border-primary text-sm"
                      value={settings.level}
                      onChange={(e) => setSettings(prev => ({ ...prev, level: e.target.value }))}
                      disabled={isCompressing}
                    >
                      {compressionLevels.map((level) => (
                        <option key={level.value} value={level.value}>
                          {level.label}
                        </option>
                      ))}
                    </select>
                    
                    {/* 压缩等级详情 */}
                    <div className="mt-3 p-4 bg-base-200/50 rounded-lg border border-base-300">
                      <div className="text-sm font-medium text-base-content mb-1">
                        {compressionLevels.find(l => l.value === settings.level)?.description}
                      </div>
                      <div className="text-xs text-base-content/60">
                        {compressionLevels.find(l => l.value === settings.level)?.details}
                      </div>
                    </div>
                  </div>

                  {/* 输出路径选择 */}
                <div className="form-control">
                  <div className="flex items-center justify-between mb-2">
                    <span className="text-sm font-medium text-base-content">Output Location</span>
                  </div>
                  <div className="flex gap-3">
                    <input
                      type="text"
                      placeholder="Choose where to save compressed file..."
                      className="input input-bordered flex-1 bg-base-100 border-base-300 focus:border-primary text-sm"
                      value={settings.outputPath ? settings.outputPath.split('/').pop() : ''}
                      readOnly
                    />
                    <button
                      className="btn btn-outline btn-secondary min-h-0 h-12 px-6"
                      onClick={selectOutputPath}
                      disabled={isCompressing}
                    >
                      Save As
                    </button>
                  </div>
                </div>

                {/* Ghostscript 状态 */}
                <div className="form-control">
                  <div className="flex items-center justify-between mb-2">
                    <span className="text-sm font-medium text-base-content">压缩引擎状态</span>
                  </div>
                  <div className="p-4 bg-base-200/50 rounded-lg border border-base-300">
                    <div className="flex items-center justify-between">
                      <div>
                        <div className="text-sm font-medium text-base-content mb-1">
                          {gsStatus.is_installed ? 
                            "✅ 高级压缩引擎已安装" : 
                            "⚠️ 高级压缩引擎未安装"}
                        </div>
                        <div className="text-xs text-base-content/60">
                          {gsStatus.is_installed ? 
                            "使用 Ghostscript 引擎可获得最佳压缩效果" : 
                            "安装 Ghostscript 引擎可获得更好的压缩效果"}
                        </div>
                      </div>
                      <div className="flex gap-2">
                        {!gsStatus.is_installed && !gsStatus.is_downloading && (
                          <>
                            <button 
                              className="btn btn-sm btn-accent" 
                              onClick={downloadGhostscript}
                              disabled={isCompressing}
                            >
                              自动安装
                            </button>
                            <button 
                              className="btn btn-sm btn-outline btn-accent" 
                              onClick={showManualInstallGuide}
                              disabled={isCompressing}
                            >
                              安装指南
                            </button>
                          </>
                        )}
                        {gsStatus.is_installed && (
                          <button 
                            className="btn btn-sm btn-error" 
                            onClick={uninstallGhostscript}
                            disabled={isCompressing}
                          >
                            卸载引擎
                          </button>
                        )}
                      </div>
                    </div>
                    
                    {/* 下载进度条 */}
                    {gsStatus.is_downloading && (
                      <div className="mt-3">
                        <div className="text-xs text-base-content/60 mb-1">
                          正在下载压缩引擎 ({Math.round(gsStatus.download_progress)}%)
                        </div>
                        <progress 
                          className="progress progress-accent w-full" 
                          value={gsStatus.download_progress} 
                          max="100"
                        ></progress>
                      </div>
                    )}
                  </div>
                </div>

                {/* 状态显示 */}
                {status && (
                  <div className="alert alert-info border-0 bg-base-200/50">
                    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="2">
                      <path strokeLinecap="round" strokeLinejoin="round" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                    <span className="text-sm">{status}</span>
                  </div>
                )}

                {/* 主要操作按钮 */}
                <div className="pt-4">
                  <button
                    className="btn btn-primary w-full h-14 text-base font-medium"
                    onClick={startCompression}
                    disabled={isCompressing || !settings.inputPath || !settings.outputPath}
                  >
                    {isCompressing ? (
                      <>
                        <span className="loading loading-spinner loading-md mr-2"></span>
                        Compressing...
                      </>
                    ) : (
                      <>
                        <svg className="w-5 h-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="2">
                          <path strokeLinecap="round" strokeLinejoin="round" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                        </svg>
                        Compress PDF
                      </>
                    )}
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>

          {/* 简约功能说明 */}
          <div className="text-center">
            <div className="flex justify-center items-center space-x-8 text-xs text-base-content/50">
              <div className="flex items-center space-x-2">
                <div className="w-2 h-2 bg-primary rounded-full"></div>
                <span>Built-in PDF Engine</span>
              </div>
              <div className="flex items-center space-x-2">
                <div className="w-2 h-2 bg-secondary rounded-full"></div>
                <span>Professional Quality Presets</span>
              </div>
              <div className="flex items-center space-x-2">
                <div className="w-2 h-2 bg-accent rounded-full"></div>
                <span>No External Dependencies</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
