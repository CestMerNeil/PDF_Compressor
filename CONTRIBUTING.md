# Contributing to PDF Compressor

Thank you for your interest in contributing to PDF Compressor! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:
- [Node.js](https://nodejs.org/) (v18 or later)
- [Rust](https://rustup.rs/) (latest stable)
- [Yarn](https://yarnpkg.com/) package manager
- [Git](https://git-scm.com/)

### Development Setup

1. **Fork the repository**
   - Click the "Fork" button on the GitHub repository page
   - Clone your forked repository locally

2. **Clone and setup**
   ```bash
   git clone https://github.com/CestMerNeil/PDF_Compressor.git
   cd PDF_Compressor
   yarn install
   ```

3. **Run in development mode**
   ```bash
   yarn tauri dev
   ```

## 📝 How to Contribute

### Reporting Bugs

1. **Search existing issues** at https://github.com/CestMerNeil/PDF_Compressor/issues to avoid duplicates
2. **Use the bug report template** when creating new issues
3. **Provide detailed information**:
   - Operating system and version
   - Application version
   - Steps to reproduce
   - Expected vs actual behavior
   - Screenshots if applicable

### Suggesting Features

1. **Check existing feature requests** to avoid duplicates
2. **Use the feature request template**
3. **Describe the problem** you're trying to solve
4. **Explain your proposed solution**
5. **Consider alternative solutions**

### Code Contributions

#### Branch Naming Convention
- `feature/description` - for new features
- `fix/description` - for bug fixes
- `docs/description` - for documentation changes
- `refactor/description` - for code refactoring

#### Pull Request Process

1. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   - Follow the coding standards (see below)
   - Add tests if applicable
   - Update documentation if needed

3. **Test your changes**
   ```bash
   yarn tauri dev    # Test in development
   yarn tauri build  # Test production build
   ```

4. **Commit your changes**
   - Use conventional commit messages (see below)
   - Keep commits focused and atomic

5. **Push and create PR**
   ```bash
   git push origin feature/your-feature-name
   ```
   - Create a pull request with a clear title and description
   - Link any relevant issues

## 📋 Coding Standards

### Rust (Backend)

- **Format**: Use `cargo fmt` before committing
- **Lint**: Run `cargo clippy` and fix warnings
- **Error Handling**: Use `Result<T, E>` for error handling
- **Documentation**: Add doc comments for public functions
- **Testing**: Write unit tests for new functionality

Example:
```rust
/// Compresses a PDF file using the specified quality level
/// 
/// # Arguments
/// * `input_path` - Path to the input PDF file
/// * `output_path` - Path where compressed PDF will be saved
/// * `quality` - Compression quality level (0.0 to 1.0)
/// 
/// # Returns
/// * `Ok(CompressionResult)` on success
/// * `Err(String)` on failure
pub async fn compress_pdf(
    input_path: String, 
    output_path: String, 
    quality: f64
) -> Result<CompressionResult, String> {
    // Implementation
}
```

### TypeScript/React (Frontend)

- **Format**: Use Prettier for code formatting
- **Lint**: Follow ESLint rules
- **Types**: Always use TypeScript types, avoid `any`
- **Components**: Use functional components with hooks
- **State**: Use appropriate React hooks for state management

Example:
```typescript
interface CompressionSettings {
  level: string;
  inputPath: string;
  outputPath: string;
}

const App: React.FC = () => {
  const [settings, setSettings] = useState<CompressionSettings>({
    level: "/ebook",
    inputPath: "",
    outputPath: "",
  });
  
  // Component implementation
};
```

### CSS/Styling

- **Framework**: Use Tailwind CSS classes
- **Components**: Use DaisyUI components when possible
- **Responsive**: Ensure responsive design for all screen sizes
- **Dark Mode**: Support both light and dark themes

## 📄 Commit Message Convention

Use conventional commits for clear and consistent commit messages:

### Format
```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or modifying tests
- `chore`: Build process or auxiliary tool changes

### Examples
```
feat(ui): add dark mode toggle to navigation bar

Add theme switcher component that allows users to toggle between
light and dark modes. Theme preference is saved to localStorage
and applied automatically on app startup.

Closes #123
```

```
fix(compression): handle invalid PDF files gracefully

Previously, the app would crash when trying to process corrupted
or invalid PDF files. Now displays user-friendly error message
and suggests troubleshooting steps.

Fixes #456
```

## 🧪 Testing

### Running Tests
```bash
# Run Rust tests
cd src-tauri
cargo test

# Run frontend tests (if available)
yarn test
```

### Test Guidelines
- Write tests for new features and bug fixes
- Ensure all tests pass before submitting PR
- Include both positive and negative test cases
- Mock external dependencies appropriately

## 📚 Documentation

### Code Documentation
- Add JSDoc comments for TypeScript functions
- Add doc comments for Rust public functions
- Update README.md for significant changes
- Include code examples in documentation

### User Documentation
- Update user-facing documentation for new features
- Include screenshots for UI changes
- Maintain accuracy of installation instructions

## 🎯 Performance Guidelines

### Frontend Performance
- Minimize bundle size
- Use React.memo for expensive components
- Implement proper loading states
- Optimize image assets

### Backend Performance
- Profile Rust code for bottlenecks
- Use appropriate data structures
- Minimize memory allocations in hot paths
- Handle large PDF files efficiently

## 🔍 Code Review Process

### For Contributors
- Respond to review feedback promptly
- Make requested changes in new commits
- Keep discussion focused and professional
- Test changes after addressing feedback

### For Reviewers
- Be constructive and specific in feedback
- Suggest improvements with examples
- Approve when code meets standards
- Test functionality when possible

## 📞 Getting Help

If you need help with contributing:

1. **Check existing documentation** in this repository
2. **Search closed issues** for similar problems
3. **Ask in GitHub Discussions** for general questions
4. **Create a new issue** for specific problems

## 🏆 Recognition

Contributors will be recognized in:
- GitHub contributors list
- Release notes for significant contributions
- Special mentions in documentation

---

Thank you for contributing to PDF Compressor! Your efforts help make this tool better for everyone. 🚀