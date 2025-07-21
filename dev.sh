#!/bin/bash

# PDF 压缩工具开发启动脚本

echo "🚀 启动 PDF 压缩工具开发环境..."

# 检查依赖
echo "📋 检查依赖..."

# 检查 Node.js
if ! command -v node &> /dev/null; then
    echo "❌ 错误: 未找到 Node.js，请先安装 Node.js"
    exit 1
fi

# 检查 Yarn
if ! command -v yarn &> /dev/null; then
    echo "❌ 错误: 未找到 Yarn，请先安装 Yarn"
    exit 1
fi

# 检查 Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ 错误: 未找到 Rust，请先安装 Rust"
    exit 1
fi

# 检查 Ghostscript
if ! command -v gs &> /dev/null; then
    echo "⚠️  警告: 未找到 Ghostscript，应用可能无法正常工作"
    echo "请安装 Ghostscript:"
    echo "  macOS: brew install ghostscript"
    echo "  Windows: 下载并安装 https://www.ghostscript.com/releases/gsdnld.html"
    echo "  Linux: sudo apt-get install ghostscript"
else
    echo "✅ Ghostscript 已安装: $(gs --version)"
fi

# 安装依赖
echo "📦 安装依赖..."
yarn install

# 启动开发服务器
echo "🔧 启动开发服务器..."
yarn tauri dev 