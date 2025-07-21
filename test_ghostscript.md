# Ghostscript 测试

## 检查 Ghostscript 是否安装

运行以下命令检查 Ghostscript 是否已安装：

```bash
gs --version
```

如果已安装，应该显示版本信息。

## 测试 PDF 压缩

创建一个简单的测试 PDF 文件，然后使用以下命令测试压缩：

```bash
gs -sDEVICE=pdfwrite -dCompatibilityLevel=1.4 -dPDFSETTINGS=/ebook -dNOPAUSE -dQUIET -dBATCH -sOutputFile="test_output.pdf" "test_input.pdf"
```

## 安装 Ghostscript

### macOS
```bash
brew install ghostscript
```

### Windows
下载并安装 [Ghostscript for Windows](https://www.ghostscript.com/releases/gsdnld.html)

### Linux (Ubuntu/Debian)
```bash
sudo apt-get install ghostscript
``` 