# Hafiz Assistant Backend - Quick Start Guide

## Prerequisites

- Rust 1.70+ installed
- Windows PowerShell or Command Prompt
- Git (optional, for cloning)

## Installation & Setup

### 1. Navigate to Project Directory
```powershell
cd "C:\Users\Dell XPS 13 7390\Documents\DATA QURAN\V2\hafiz_assistant_backend"
```

### 2. Build the Project
```powershell
# Development build
cargo build

# Release build (recommended for production)
cargo build --release
```

### 3. Run the CLI
```powershell
# Run the interactive CLI
.\target\release\hafiz_assistant_cli.exe

# Or use cargo run
cargo run --bin hafiz_assistant_cli
```

## Quick Feature Overview

### Main Menu Options
1. **Test Ayah Data Retrieval** - Get specific verses
2. **Test Search Features** - Various search types
3. **Test Surah Operations** - Chapter-based operations
4. **Test Navigation Features** - Juz/page navigation
5. **Test Page (Mushaf) Functions** - Page-based access
6. **Test FFI Functions** - Foreign function interface
7. **Show Quran Statistics** - Data overview
8. **Interactive Search** - Real-time search
9. **Exit** - Close application

### Example Usage

#### Get a Specific Ayah
```
Choose option: 1
Enter surah number: 1
Enter ayah number: 1
```

#### Search for Text
```
Choose option: 2
Choose search type: 1 (Text Search)
Enter search query: rahman
```

#### Get Page Data
```
Choose option: 5
Choose page function: 1 (Get Page Data)
Enter page number: 1
```

## Key Features Demonstrated

### 1. Data Retrieval
- Get any verse by surah:ayah reference
- Access complete surah information
- Navigate by juz (para) numbers
- Access page-based data for mushaf rendering

### 2. Search Capabilities
- **Text Search**: Search Arabic text and translations
- **Translation Search**: Search Indonesian translations
- **Similarity Search**: Find similar verses
- **Range Search**: Search within specific ranges
- **Theme Search**: Search by topics
- **Advanced Search**: Multiple criteria
- **Fuzzy Search**: Approximate matching

### 3. Navigation
- Browse by surah (1-114)
- Navigate by juz (1-30)
- Access by page (1-604)
- Random verse exploration

### 4. Page (Mushaf) Functions
- Get complete page data with verses
- Find which page contains a specific verse
- Get verses within page ranges
- Access line-by-line verse information

## Data Files Used

The application loads from the following JSON files in the `data/` directory:

- `quran-metadata-ayah.json` - Complete ayah metadata
- `quran-id-simple.json` - Simple Quran text
- `indonesian-mokhtasar.json` - Indonesian translations
- `matching-ayah.json` - Similar ayah relationships
- `surah-info-id.json` - Surah information
- `qpc-v2-15-lines.db.json` - Page/mushaf data
- And many more specialized datasets

## Performance Notes

- First launch may take a few seconds to load all data
- Subsequent operations are very fast due to in-memory caching
- Search operations are optimized for real-time response
- Page data is efficiently loaded for quick mushaf rendering

## Troubleshooting

### Build Issues
If you encounter build errors:
```powershell
cargo clean
cargo build --release
```

### Data Loading Issues
Ensure all JSON files are present in the `data/` directory.

### Missing Dependencies
```powershell
cargo update
```

## FFI Integration

For Flutter integration, the library provides C-compatible FFI functions:

```rust
// Example FFI usage (for Flutter developers)
extern "C" {
    fn hafiz_get_ayah(surah: u16, ayah: u16) -> *mut c_char;
    fn hafiz_search_text(query: *const c_char) -> *mut c_char;
    fn hafiz_get_page_data(page: u16) -> *mut c_char;
}
```

The library generates:
- `hafiz_assistant_core.dll` (Windows)
- `libhafiz_assistant_core.so` (Linux)
- `libhafiz_assistant_core.dylib` (macOS)

## Testing All Features

To quickly test all features:

1. **Start the CLI**: `.\target\release\hafiz_assistant_cli.exe`
2. **Try each menu option**: Go through options 1-8
3. **Test search**: Try different search types with various queries
4. **Test navigation**: Browse different surahs, juz, and pages
5. **Test page functions**: Access mushaf data for rendering

## Next Steps

- Integrate with Flutter using the FFI interface
- Use the library in other Rust projects
- Extend functionality with additional features
- Deploy to other platforms (Android, iOS, WebAssembly)

## Support

For issues or questions:
- Check the `PROJECT_FINAL_REPORT.md` for detailed information
- Review the source code in `src/` directory
- Test features using the interactive CLI

---

**Happy coding! 🚀**
