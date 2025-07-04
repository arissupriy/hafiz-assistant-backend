# Hafiz Assistant Backend

🕌 **Rust-based Quran Data Processing Library with FFI Support**

A comprehensive, high-performance backend for Quran data processing with Foreign Function Interface (FFI) support for Flutter integration and multi-platform deployment.

## ✨ Features

### 🔍 **Advanced Search Capabilities**
- **Text Search**: Full-text search in Arabic and translations
- **Translation Search**: Search within Indonesian translations
- **Similarity Search**: Bidirectional similar ayah discovery
- **Range Search**: Search within surah/juz/page ranges
- **Theme Search**: Search by topics and themes
- **Advanced Search**: Multiple criteria combination
- **Fuzzy Search**: Approximate string matching

### 📖 **Interactive Page Viewer (NEW!)**
- **Page-by-page navigation** with intuitive controls
- **Search within pages** for specific text
- **Jump to specific verses** and find their pages
- **Random page exploration** for discovery
- **Detailed page information** with line-by-line breakdown
- **604 pages** of complete mushaf data

### 🗂️ **Data Management**
- **Complete Quran data** with 6,236 verses across 114 surahs
- **Indonesian translations** fully integrated
- **Bidirectional similar ayah** relationships
- **Page/mushaf data** for accurate rendering
- **Multiple data sources** with validation

### 🔧 **Technical Features**
- **FFI Interface**: C-compatible API for Flutter integration
- **Multi-platform**: Windows, Linux, macOS, Android, iOS, WebAssembly
- **Memory efficient**: Optimized data structures and loading
- **Fast search**: Performance-optimized algorithms
- **CLI Application**: Interactive testing and demonstration

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ installed
- Windows PowerShell or Command Prompt

### Installation
```bash
# Clone or navigate to the project directory
cd hafiz_assistant_backend

# Build the project
cargo build --release

# Run the CLI
.\target\release\hafiz_assistant_cli.exe
```

### CLI Features
1. **Test Ayah Data Retrieval** - Get specific verses
2. **Test Search Features** - Various search types
3. **Test Surah Operations** - Chapter-based operations
4. **Test Navigation Features** - Juz/page navigation
5. **Test Page (Mushaf) Functions** - Page-based access
6. **Test FFI Functions** - Foreign function interface
7. **Show Quran Statistics** - Data overview
8. **Interactive Search** - Real-time search
9. **Interactive Page Viewer** - 📖 **NEW!** Page-by-page exploration
10. **Exit** - Close application

## 📖 Interactive Page Viewer

The new **Interactive Page Viewer** allows you to:
- Navigate through all 604 pages of the Quran
- Search within specific pages
- Jump to verses and find their pages
- Explore random pages for discovery
- View detailed page information with line-by-line breakdown

**Commands:**
- `n` - Next page
- `p` - Previous page
- `g` - Go to specific page
- `f` - Find verse location
- `r` - Random page
- `s` - Search within page
- `h` - Help
- `q` - Quit

## 🔧 FFI Integration

For Flutter integration, the library provides C-compatible FFI functions:

```rust
// Example FFI usage (for Flutter developers)
extern "C" {
    fn hafiz_get_ayah(surah: u16, ayah: u16) -> *mut c_char;
    fn hafiz_search_text(query: *const c_char) -> *mut c_char;
    fn hafiz_get_page_data(page: u16) -> *mut c_char;
}
```

## 📊 Data Statistics

- **Total Ayah**: 6,236 verses
- **Total Surah**: 114 chapters
- **Total Juz**: 30 sections
- **Total Pages**: 604 pages
- **Total Hizb**: 60 sections
- **Total Ruku**: 556 sections
- **Total Manzil**: 7 sections
- **Total Sajda**: 15 prostration verses

## 🏗️ Architecture

```
src/
├── lib.rs              # Main library entry point
├── bin/main.rs         # CLI application
├── core/               # Core processing and search logic
├── data/               # Data structures and loading
├── ffi/                # Foreign Function Interface
└── utils/              # Text processing utilities
```

## 🎯 Use Cases

- **Flutter App Integration**: Use FFI for mobile/desktop apps
- **Research Tools**: Analyze Quran structure and content
- **Educational Applications**: Interactive Quran exploration
- **API Development**: Backend for web services
- **Data Analysis**: Extract insights from Quran data

## 📚 Documentation

- [Project Final Report](docs/PROJECT_FINAL_REPORT.md) - Complete project overview
- [Quick Start Guide](docs/QUICK_START_GUIDE.md) - Getting started
- [Interactive Page Viewer](docs/INTERACTIVE_PAGE_VIEWER.md) - New feature guide
- [Flutter Integration](docs/FLUTTER_INTEGRATION.md) - Integration guide
- [FFI Reference](docs/FFI_REFERENCE.md) - FFI function reference
- [Flutter Ready Status](docs/FLUTTER_READY.md) - Integration readiness

## 🔄 Build Commands

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run CLI
cargo run --bin hafiz_assistant_cli

# Run tests
cargo test

# Clean build
cargo clean
```

## 🌟 Recent Updates

### v1.0.0 (Latest)
- ✅ **NEW: Interactive Page Viewer** - Navigate through 604 pages interactively
- ✅ Complete modular architecture restoration
- ✅ Bidirectional similar ayah search
- ✅ Translation integration
- ✅ Page/mushaf data support
- ✅ FFI interface for Flutter
- ✅ Multi-platform deployment ready
- ✅ Comprehensive CLI with all features

## 🤝 Contributing

This project is production-ready and fully functional. For questions or contributions, please refer to the documentation and example files.

## 📄 License

MIT License - see LICENSE file for details.

---

**Status**: ✅ **COMPLETED** - Production Ready  
**Version**: 1.0.0  
**Last Updated**: January 2025