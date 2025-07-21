# PDF Compressor

<div align="center">

![PDF Compressor Logo](src-tauri/icons/icon.png)

**A modern, professional PDF compression tool with built-in optimization engine**

[![Build Status](https://github.com/CestMerNeil/PDF_Compressor/workflows/Build%20and%20Release/badge.svg)](https://github.com/CestMerNeil/PDF_Compressor/actions)
[![Release](https://img.shields.io/github/v/release/CestMerNeil/PDF_Compressor)](https://github.com/CestMerNeil/PDF_Compressor/releases)
[![License](https://img.shields.io/github/license/CestMerNeil/PDF_Compressor)](LICENSE)
[![Downloads](https://img.shields.io/github/downloads/CestMerNeil/PDF_Compressor/total)](https://github.com/CestMerNeil/PDF_Compressor/releases)

[Download Latest](https://github.com/CestMerNeil/PDF_Compressor/releases/latest) • [Report Issues](https://github.com/CestMerNeil/PDF_Compressor/issues)

</div>

## ✨ Features

- **🚀 Built-in PDF Engine**: No external dependencies like Ghostscript required
- **📊 Professional Quality Presets**: Four optimized compression levels for different use cases
- **🎨 Modern Interface**: Clean, minimalist design with professional workflow
- **🔒 Privacy First**: All processing happens locally - no data leaves your machine
- **⚡ Fast & Efficient**: Built with Rust and Tauri for optimal performance
- **🌍 Cross-platform**: Native apps for Windows, macOS, and Linux
- **📱 Responsive Design**: Optimized interface that works beautifully on any screen size

## 📊 Compression Levels

| Level | DPI | JPEG Quality | Use Case | Description |
|-------|-----|--------------|----------|-------------|
| **Screen** | 72 | 30% | Web sharing, email | Maximum compression for web sharing and email |
| **eBook** | 150 | 50% | Document reading | Balanced quality for general document reading |
| **Printer** | 300 | 80% | Office printing | High quality for office printing and documents |
| **Prepress** | 300+ | 90%+ | Commercial printing | Professional printing and commercial use |

## 🛠️ Tech Stack

- **Frontend**: React 18, TypeScript, Tailwind CSS, DaisyUI
- **Backend**: Rust, Tauri 2.0
- **PDF Processing**: lopdf (pure Rust PDF library)
- **Build System**: Vite, Cargo
- **CI/CD**: GitHub Actions

## 💾 Installation

### Windows
- **Recommended**: Download `.msi` installer for automatic installation
- **Alternative**: Download `.exe` portable installer

### macOS
- Download `.dmg` file and drag PDF Compressor to Applications folder
- **Note**: You may need to right-click and select "Open" first time due to security settings

### Linux
- **AppImage**: Download `.AppImage` file, make executable with `chmod +x`, and run directly
- **Debian/Ubuntu**: Download `.deb` file and install with `sudo dpkg -i filename.deb`

## 🛠️ Development

### Prerequisites

- [Node.js](https://nodejs.org/) (v18 or later)
- [Rust](https://rustup.rs/) (latest stable)
- [Yarn](https://yarnpkg.com/) package manager

### Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/CestMerNeil/PDF_Compressor.git
   cd PDF_Compressor
   ```

2. **Install dependencies**
   ```bash
   yarn install
   ```

3. **Run in development mode**
   ```bash
   yarn tauri dev
   ```

4. **Build for production**
   ```bash
   yarn tauri build
   ```

### Project Structure

```
PDF_Compressor/
├── src/                    # React frontend source
│   ├── App.tsx            # Main application component
│   ├── App.css            # Application styles
│   └── main.tsx           # Entry point
├── src-tauri/             # Rust backend source
│   ├── src/
│   │   ├── lib.rs         # Core PDF processing logic
│   │   └── main.rs        # Application entry point
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
├── public/                # Static assets
└── .github/workflows/     # GitHub Actions CI/CD
```

## 🤝 Contributing

We welcome contributions! Please follow these steps:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) - For the excellent desktop app framework
- [lopdf](https://github.com/J-F-Liu/lopdf) - For pure Rust PDF processing
- [DaisyUI](https://daisyui.com/) - For beautiful UI components

---

<div align="center">

**Made with ❤️ for the open source community**

[⭐ Star us on GitHub](https://github.com/CestMerNeil/PDF_Compressor) if you find this project useful!

</div>
