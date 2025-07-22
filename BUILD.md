# PDF Compressor 构建说明

本项目使用 GitHub Actions 进行自动化构建，支持多平台构建并自动为不同系统版本添加系统名称标识。

## 支持的平台

- **Windows**: Windows 11 x64
- **macOS**: 
  - Intel (x64)
  - Apple Silicon (ARM64)
- **Linux**: Ubuntu 22.04 x64

## 构建触发条件

- 推送到 `main` 或 `develop` 分支
- 创建以 `v` 开头的标签（如 `v2.1.0`）
- 手动触发工作流
- Pull Request 到 `main` 分支

## 构建产物命名规则

构建完成后，文件将按以下规则命名：

### Windows
- `PDF-Compressor-v3.0.0-Windows-x64.msi`
- `PDF-Compressor-v3.0.0-Windows-x64-setup.exe`

### macOS
- `PDF-Compressor-v3.0.0-macOS-Intel.dmg` (Intel 版本)
- `PDF-Compressor-v3.0.0-macOS-AppleSilicon.dmg` (Apple Silicon 版本)

### Linux
- `PDF-Compressor-v3.0.0-ubuntu-x64.deb`
- `PDF-Compressor-v3.0.0-ubuntu-x64.AppImage`

## 如何触发构建

### 开发构建
推送代码到 `main` 或 `develop` 分支即可自动触发构建。

### 发布构建
1. 创建并推送标签：
   ```bash
   git tag v3.0.0
   git push origin v3.0.0
   ```
2. GitHub Actions 将自动构建所有平台版本并创建 GitHub Release

### 手动构建
1. 访问 GitHub 仓库的 Actions 页面
2. 选择 "Build PDF Compressor" 工作流
3. 点击 "Run workflow" 按钮

## 构建产物下载

- **开发构建**: 在 Actions 页面的对应构建中下载 Artifacts
- **发布构建**: 在 Releases 页面下载对应版本的文件

## 故障排除

### Linux 构建失败
如果 Linux 构建失败，通常是由于依赖问题。GitHub Actions 已配置安装所有必要的依赖，包括：
- webkit2gtk-4.0/4.1
- GTK3
- 其他必要的系统库

### macOS 代码签名
如果需要代码签名，请在仓库设置中添加以下 Secrets：
- `TAURI_PRIVATE_KEY`: 签名私钥
- `TAURI_KEY_PASSWORD`: 私钥密码

### Windows 代码签名
Windows 代码签名配置在 `tauri.conf.json` 中，如需启用请修改相关配置。