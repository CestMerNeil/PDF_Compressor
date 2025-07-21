# 🚀 Deployment Guide

This guide explains how to upload and deploy the PDF Compressor project to GitHub with automated builds.

## 📋 Prerequisites

- GitHub account
- Git installed locally
- Project already committed locally (✅ completed)

## 🌐 GitHub Deployment Steps

### 1. Create GitHub Repository

1. Go to [GitHub](https://github.com) and sign in
2. Click the "+" icon → "New repository"
3. Repository settings:
   - **Repository name**: `pdf-compressor`
   - **Description**: `A modern, professional PDF compression tool with built-in optimization engine`
   - **Visibility**: Public (recommended for open source)
   - **Initialize**: ❌ Do NOT initialize (we have existing code)

### 2. Connect Local Repository to GitHub

```bash
# Add GitHub remote origin
git remote add origin https://github.com/YOUR_USERNAME/pdf-compressor.git

# Verify remote was added
git remote -v

# Push to GitHub
git branch -M main
git push -u origin main
```

### 3. Update README Badges

After creating the repository, update the badges in `README.md`:

```markdown
[![Build Status](https://github.com/YOUR_USERNAME/pdf-compressor/workflows/Build%20and%20Release/badge.svg)](https://github.com/YOUR_USERNAME/pdf-compressor/actions)
[![Release](https://img.shields.io/github/v/release/YOUR_USERNAME/pdf-compressor)](https://github.com/YOUR_USERNAME/pdf-compressor/releases)
[![License](https://img.shields.io/github/license/YOUR_USERNAME/pdf-compressor)](LICENSE)
[![Downloads](https://img.shields.io/github/downloads/YOUR_USERNAME/pdf-compressor/total)](https://github.com/YOUR_USERNAME/pdf-compressor/releases)
```

Replace `YOUR_USERNAME` with your actual GitHub username.

## 🔧 GitHub Actions Setup

The project includes two automated workflows:

### Build Workflow (`.github/workflows/build.yml`)
- **Triggers**: Push to `main`, pull requests
- **Platforms**: Windows, macOS, Linux
- **Actions**: Build and test on all platforms

### Release Workflow (`.github/workflows/release.yml`)  
- **Triggers**: Git tags (e.g., `v1.0.0`)
- **Platforms**: Windows, macOS, Linux (including Apple Silicon)
- **Actions**: Build and publish releases automatically

## 📦 Creating Releases

### Manual Release Process

1. **Tag a release locally**:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. **GitHub Actions will automatically**:
   - Build for all platforms
   - Create release with binaries
   - Generate release notes

### Automated Release Assets

The release will include:

**Windows**:
- `pdf-compressor_1.0.0_x64-setup.exe` (NSIS installer)
- `pdf-compressor_1.0.0_x64.msi` (MSI installer)

**macOS**:
- `pdf-compressor_1.0.0_aarch64.dmg` (Apple Silicon)
- `pdf-compressor_1.0.0_x64.dmg` (Intel Mac)

**Linux**:
- `pdf-compressor_1.0.0_amd64.AppImage` (Universal)
- `pdf-compressor_1.0.0_amd64.deb` (Debian/Ubuntu)

## 🎯 Post-Deployment Tasks

### 1. Repository Settings

Configure these settings in GitHub:

- **Settings → General → Features**:
  - ✅ Issues
  - ✅ Discussions  
  - ✅ Projects
  - ✅ Wiki

- **Settings → Pages**:
  - Enable if you want a project website

### 2. Branch Protection

- **Settings → Branches → Add rule**:
  - Branch name: `main`
  - ✅ Require pull request reviews
  - ✅ Require status checks to pass
  - ✅ Require branches to be up to date

### 3. Issue Templates

GitHub will automatically use the issue templates from `.github/` directory.

### 4. Community Standards

GitHub will show a community standards checklist. We've included:
- ✅ README.md
- ✅ LICENSE
- ✅ CONTRIBUTING.md
- ✅ .gitignore

## 📊 Monitoring Builds

### GitHub Actions Dashboard

1. Go to repository → **Actions** tab
2. Monitor workflow runs
3. Check build logs for any issues
4. View artifact downloads

### Release Management

1. **Releases** tab shows all published versions
2. Download statistics are available
3. Release notes are auto-generated

## 🛠️ Development Workflow

### For Contributors

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/pdf-compressor.git
cd pdf-compressor

# Install dependencies
yarn install

# Run development mode
yarn tauri dev

# Build locally
yarn tauri build
```

### For Maintainers

```bash
# Create new release
git tag v1.1.0
git push origin v1.1.0

# GitHub Actions will handle the rest automatically
```

## 🔍 Troubleshooting

### Common Issues

1. **Build Failures**:
   - Check GitHub Actions logs
   - Verify Rust/Node.js versions in workflows
   - Ensure all dependencies are properly declared

2. **Release Creation Fails**:
   - Verify tag format (v1.0.0)
   - Check GitHub token permissions
   - Review workflow file syntax

3. **Missing Artifacts**:
   - Build may have failed on specific platform
   - Check individual job logs
   - Platform-specific dependencies might be missing

### Getting Help

- Check [GitHub Actions Documentation](https://docs.github.com/en/actions)
- Review [Tauri GitHub Actions Guide](https://tauri.app/guides/distribution/github-actions)
- Create issue in repository for specific problems

## 🎉 Success Checklist

After deployment, verify:

- ✅ Repository is public and accessible
- ✅ README displays correctly with badges
- ✅ GitHub Actions workflows run successfully
- ✅ Releases are created with proper artifacts
- ✅ Download links work for all platforms
- ✅ Installation instructions are accurate

---

**🚀 Your PDF Compressor is now live on GitHub with automated cross-platform builds!**