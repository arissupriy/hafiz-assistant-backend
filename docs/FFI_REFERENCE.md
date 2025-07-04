# FFI Reference for Hafiz Assistant Backend

## Overview

This document provides a comprehensive reference for the Foreign Function Interface (FFI) of the Hafiz Assistant Backend. The FFI allows integration with other programming languages like Dart/Flutter, Python, JavaScript (Node.js), and others.

## Table of Contents

1. [Function List](#function-list)
2. [Data Structures](#data-structures)
3. [Error Codes](#error-codes)
4. [Memory Management](#memory-management)
5. [Usage Examples](#usage-examples)
6. [Platform-Specific Notes](#platform-specific-notes)

## Function List

### Initialization Functions

#### `initialize_data_ffi()`
```c
bool initialize_data_ffi();
```
- **Description**: Initializes the Quran data. Must be called before any other functions.
- **Returns**: `true` if successful, `false` otherwise.
- **Note**: This function loads all JSON data files and may take a few seconds.

---

### Ayah Retrieval Functions

#### `get_ayah_data_ffi(verse_key)`
```c
AyahDataFFI* get_ayah_data_ffi(const char* verse_key);
```
- **Description**: Retrieves complete ayah data by verse key.
- **Parameters**: 
  - `verse_key`: String in format "surah:ayah" (e.g., "1:1", "2:255")
- **Returns**: Pointer to `AyahDataFFI` or `NULL` if not found.
- **Memory**: Must be freed with `free_ayah_data_ffi()`.

#### `get_ayahs_by_surah_ffi(surah_number, result_count)`
```c
AyahDataFFI* get_ayahs_by_surah_ffi(uint16_t surah_number, size_t* result_count);
```
- **Description**: Gets all ayahs in a specific surah.
- **Parameters**:
  - `surah_number`: Surah number (1-114)
  - `result_count`: Pointer to store the number of results
- **Returns**: Array of `AyahDataFFI` or `NULL` if not found.
- **Memory**: Must be freed with `free_ayah_array_ffi()`.

#### `get_ayahs_by_juz_ffi(juz_number, result_count)`
```c
AyahDataFFI* get_ayahs_by_juz_ffi(uint8_t juz_number, size_t* result_count);
```
- **Description**: Gets all ayahs in a specific juz (para).
- **Parameters**:
  - `juz_number`: Juz number (1-30)
  - `result_count`: Pointer to store the number of results
- **Returns**: Array of `AyahDataFFI` or `NULL` if not found.
- **Memory**: Must be freed with `free_ayah_array_ffi()`.

#### `get_ayahs_by_page_ffi(page_number, result_count)`
```c
AyahDataFFI* get_ayahs_by_page_ffi(uint16_t page_number, size_t* result_count);
```
- **Description**: Gets all ayahs on a specific Mushaf page.
- **Parameters**:
  - `page_number`: Page number (1-604)
  - `result_count`: Pointer to store the number of results
- **Returns**: Array of `AyahDataFFI` or `NULL` if not found.
- **Memory**: Must be freed with `free_ayah_array_ffi()`.

#### `get_random_ayah_ffi()`
```c
AyahDataFFI* get_random_ayah_ffi();
```
- **Description**: Gets a random ayah from the Quran.
- **Returns**: Pointer to `AyahDataFFI` or `NULL` if error.
- **Memory**: Must be freed with `free_ayah_data_ffi()`.

---

### Search Functions

#### `search_ayahs_by_text_ffi(query, limit, result_count)`
```c
AyahDataFFI* search_ayahs_by_text_ffi(const char* query, size_t limit, size_t* result_count);
```
- **Description**: Searches ayahs by Arabic text.
- **Parameters**:
  - `query`: Arabic text to search for
  - `limit`: Maximum number of results
  - `result_count`: Pointer to store the number of results
- **Returns**: Array of `AyahDataFFI` or `NULL` if no results.
- **Memory**: Must be freed with `free_ayah_array_ffi()`.

#### `search_similar_ayahs_ffi(verse_key, limit, result_count)`
```c
SimilarAyahWithTextFFI* search_similar_ayahs_ffi(const char* verse_key, size_t limit, size_t* result_count);
```
- **Description**: Finds ayahs similar to a given verse.
- **Parameters**:
  - `verse_key`: Reference verse key (e.g., "2:255")
  - `limit`: Maximum number of results
  - `result_count`: Pointer to store the number of results
- **Returns**: Array of `SimilarAyahWithTextFFI` or `NULL` if no results.
- **Memory**: Must be freed with `free_similar_ayahs_ffi()`.

#### `advanced_search_ffi(criteria, result_count)`
```c
AyahDataFFI* advanced_search_ffi(const AdvancedSearchCriteriaFFI* criteria, size_t* result_count);
```
- **Description**: Performs advanced search with multiple criteria.
- **Parameters**:
  - `criteria`: Pointer to `AdvancedSearchCriteriaFFI` structure
  - `result_count`: Pointer to store the number of results
- **Returns**: Array of `AyahDataFFI` or `NULL` if no results.
- **Memory**: Must be freed with `free_ayah_array_ffi()`.

---

### Surah Information Functions

#### `get_surah_info_ffi(surah_number)`
```c
SurahInfoFFI* get_surah_info_ffi(uint16_t surah_number);
```
- **Description**: Gets detailed information about a surah.
- **Parameters**:
  - `surah_number`: Surah number (1-114)
- **Returns**: Pointer to `SurahInfoFFI` or `NULL` if not found.
- **Memory**: Must be freed with `free_surah_info_ffi()`.

---

### Statistics Functions

#### `get_quran_statistics_ffi()`
```c
QuranStatisticsFFI* get_quran_statistics_ffi();
```
- **Description**: Gets overall Quran statistics.
- **Returns**: Pointer to `QuranStatisticsFFI` or `NULL` if error.
- **Memory**: Must be freed with `free_quran_statistics_ffi()`.

---

### Validation Functions

#### `validate_verse_key_ffi(verse_key)`
```c
bool validate_verse_key_ffi(const char* verse_key);
```
- **Description**: Validates a verse key format and existence.
- **Parameters**:
  - `verse_key`: String in format "surah:ayah"
- **Returns**: `true` if valid, `false` otherwise.

---

### Memory Management Functions

#### `free_ayah_data_ffi(data)`
```c
void free_ayah_data_ffi(AyahDataFFI* data);
```
- **Description**: Frees memory allocated for a single `AyahDataFFI`.
- **Parameters**:
  - `data`: Pointer to `AyahDataFFI` to free

#### `free_ayah_array_ffi(data, count)`
```c
void free_ayah_array_ffi(AyahDataFFI* data, size_t count);
```
- **Description**: Frees memory allocated for an array of `AyahDataFFI`.
- **Parameters**:
  - `data`: Pointer to array of `AyahDataFFI`
  - `count`: Number of elements in the array

#### `free_similar_ayahs_ffi(data, count)`
```c
void free_similar_ayahs_ffi(SimilarAyahWithTextFFI* data, size_t count);
```
- **Description**: Frees memory allocated for an array of `SimilarAyahWithTextFFI`.
- **Parameters**:
  - `data`: Pointer to array of `SimilarAyahWithTextFFI`
  - `count`: Number of elements in the array

#### `free_surah_info_ffi(data)`
```c
void free_surah_info_ffi(SurahInfoFFI* data);
```
- **Description**: Frees memory allocated for a `SurahInfoFFI`.
- **Parameters**:
  - `data`: Pointer to `SurahInfoFFI` to free

#### `free_quran_statistics_ffi(data)`
```c
void free_quran_statistics_ffi(QuranStatisticsFFI* data);
```
- **Description**: Frees memory allocated for a `QuranStatisticsFFI`.
- **Parameters**:
  - `data`: Pointer to `QuranStatisticsFFI` to free

---

## Data Structures

### AyahDataFFI
```c
typedef struct {
    char* verse_key;
    char* arabic_text;
    char* translation;
    char* transliteration;
    char* surah_name;
    char* surah_name_arabic;
    uint16_t surah_number;
    uint16_t ayah_number;
    uint8_t juz_number;
    uint8_t hizb_number;
    uint8_t rub_number;
    uint8_t ruku_number;
    uint8_t manzil_number;
    uint16_t page_number;
} AyahDataFFI;
```

**Fields:**
- `verse_key`: Verse identifier (e.g., "2:255")
- `arabic_text`: Arabic text of the ayah
- `translation`: Indonesian translation
- `transliteration`: Latin transliteration
- `surah_name`: Surah name in Latin
- `surah_name_arabic`: Surah name in Arabic
- `surah_number`: Surah number (1-114)
- `ayah_number`: Ayah number within the surah
- `juz_number`: Juz (para) number (1-30)
- `hizb_number`: Hizb number (1-60)
- `rub_number`: Rub number (1-240)
- `ruku_number`: Ruku number (1-556)
- `manzil_number`: Manzil number (1-7)
- `page_number`: Mushaf page number (1-604)

### SimilarAyahWithTextFFI
```c
typedef struct {
    char* verse_key;
    char* arabic_text;
    char* translation;
    char* transliteration;
    double similarity_score;
    uint16_t surah_number;
    uint16_t ayah_number;
} SimilarAyahWithTextFFI;
```

**Fields:**
- `verse_key`: Verse identifier
- `arabic_text`: Arabic text of the ayah
- `translation`: Indonesian translation
- `transliteration`: Latin transliteration
- `similarity_score`: Similarity score (0.0-1.0)
- `surah_number`: Surah number
- `ayah_number`: Ayah number

### SurahInfoFFI
```c
typedef struct {
    uint16_t id;
    char* name_simple;
    char* name_arabic;
    char* name_english;
    uint16_t revelation_order;
    char* revelation_place;
    uint16_t verses_count;
    bool bismillah_pre;
} SurahInfoFFI;
```

**Fields:**
- `id`: Surah number (1-114)
- `name_simple`: Simple name in Latin
- `name_arabic`: Name in Arabic
- `name_english`: Name in English
- `revelation_order`: Order of revelation
- `revelation_place`: Place of revelation ("Makkah" or "Madinah")
- `verses_count`: Number of verses in the surah
- `bismillah_pre`: Whether bismillah precedes the surah

### QuranStatisticsFFI
```c
typedef struct {
    uint16_t total_surahs;
    uint32_t total_ayahs;
    uint32_t total_words;
    uint32_t total_letters;
    uint16_t total_pages;
    uint8_t total_juz;
    uint8_t total_hizb;
    uint16_t total_ruku;
    uint8_t total_manzil;
    uint8_t total_sajda;
} QuranStatisticsFFI;
```

**Fields:**
- `total_surahs`: Total number of surahs (114)
- `total_ayahs`: Total number of ayahs (6236)
- `total_words`: Total number of words
- `total_letters`: Total number of letters
- `total_pages`: Total number of pages (604)
- `total_juz`: Total number of juz (30)
- `total_hizb`: Total number of hizb (60)
- `total_ruku`: Total number of ruku (556)
- `total_manzil`: Total number of manzil (7)
- `total_sajda`: Total number of sajda (15)

### AdvancedSearchCriteriaFFI
```c
typedef struct {
    const char* text_query;
    const char* translation_query;
    const char* transliteration_query;
    uint16_t surah_number;    // 0 means no filter
    uint8_t juz_number;       // 0 means no filter
    uint16_t page_number;     // 0 means no filter
    size_t limit;
} AdvancedSearchCriteriaFFI;
```

**Fields:**
- `text_query`: Arabic text to search for (can be NULL)
- `translation_query`: Translation text to search for (can be NULL)
- `transliteration_query`: Transliteration text to search for (can be NULL)
- `surah_number`: Filter by surah number (0 for no filter)
- `juz_number`: Filter by juz number (0 for no filter)
- `page_number`: Filter by page number (0 for no filter)
- `limit`: Maximum number of results

### SearchTypeFFI
```c
typedef enum {
    TextSearch = 0,
    TranslationSearch = 1,
    TransliterationSearch = 2,
    SimilaritySearch = 3,
    ThemeSearch = 4,
    RangeSearch = 5,
    AdvancedSearch = 6,
    FuzzySearch = 7
} SearchTypeFFI;
```

### ErrorFFI
```c
typedef struct {
    int32_t error_code;
    char* error_message;
} ErrorFFI;
```

**Fields:**
- `error_code`: Error code (see Error Codes section)
- `error_message`: Human-readable error message

---

## Error Codes

```c
#define ERROR_NONE 0
#define ERROR_DATA_NOT_INITIALIZED 1
#define ERROR_INVALID_VERSE_KEY 2
#define ERROR_INVALID_SURAH_NUMBER 3
#define ERROR_INVALID_PARAMETERS 4
#define ERROR_MEMORY_ALLOCATION 5
#define ERROR_DATA_NOT_FOUND 6
#define ERROR_UNKNOWN 99
```

**Error Descriptions:**
- `ERROR_NONE`: No error
- `ERROR_DATA_NOT_INITIALIZED`: Data not initialized, call `initialize_data_ffi()` first
- `ERROR_INVALID_VERSE_KEY`: Invalid verse key format
- `ERROR_INVALID_SURAH_NUMBER`: Invalid surah number (must be 1-114)
- `ERROR_INVALID_PARAMETERS`: Invalid function parameters
- `ERROR_MEMORY_ALLOCATION`: Memory allocation failed
- `ERROR_DATA_NOT_FOUND`: Requested data not found
- `ERROR_UNKNOWN`: Unknown error

---

## Memory Management

### Critical Memory Management Rules

1. **Always free allocated memory**: Every function that returns a pointer allocates memory that must be freed.

2. **Use correct free functions**: Each structure type has its own free function.

3. **Free arrays properly**: Array functions return allocated arrays that must be freed with array-specific functions.

4. **Check for NULL**: Always check if returned pointers are NULL before use.

### Memory Management Examples

```c
// Single ayah - must be freed
AyahDataFFI* ayah = get_ayah_data_ffi("2:255");
if (ayah != NULL) {
    // Use ayah data
    printf("Arabic: %s\n", ayah->arabic_text);
    
    // Free memory
    free_ayah_data_ffi(ayah);
}

// Array of ayahs - must be freed
size_t count;
AyahDataFFI* ayahs = get_ayahs_by_surah_ffi(2, &count);
if (ayahs != NULL) {
    // Use ayahs
    for (size_t i = 0; i < count; i++) {
        printf("Ayah %zu: %s\n", i + 1, ayahs[i].arabic_text);
    }
    
    // Free memory
    free_ayah_array_ffi(ayahs, count);
}

// Similar ayahs - must be freed
size_t similar_count;
SimilarAyahWithTextFFI* similar = search_similar_ayahs_ffi("2:255", 5, &similar_count);
if (similar != NULL) {
    // Use similar ayahs
    for (size_t i = 0; i < similar_count; i++) {
        printf("Similar: %s (%.2f)\n", similar[i].verse_key, similar[i].similarity_score);
    }
    
    // Free memory
    free_similar_ayahs_ffi(similar, similar_count);
}
```

---

## Usage Examples

### Python Example

```python
import ctypes
from ctypes import c_char_p, c_bool, c_uint16, c_uint8, c_size_t, POINTER

# Load the library
lib = ctypes.CDLL('./libhafiz_assistant_core.so')  # Linux
# lib = ctypes.CDLL('./hafiz_assistant_core.dll')  # Windows

# Define function signatures
lib.initialize_data_ffi.restype = c_bool
lib.get_ayah_data_ffi.argtypes = [c_char_p]
lib.get_ayah_data_ffi.restype = ctypes.c_void_p
lib.free_ayah_data_ffi.argtypes = [ctypes.c_void_p]

# Initialize data
if lib.initialize_data_ffi():
    print("Data initialized successfully")
    
    # Get ayah data
    ayah_ptr = lib.get_ayah_data_ffi(b"2:255")
    if ayah_ptr:
        # Access ayah data (would need to define structure)
        print("Got ayah data")
        
        # Free memory
        lib.free_ayah_data_ffi(ayah_ptr)
else:
    print("Failed to initialize data")
```

### C Example

```c
#include <stdio.h>
#include <stdlib.h>

// Function declarations
extern bool initialize_data_ffi();
extern AyahDataFFI* get_ayah_data_ffi(const char* verse_key);
extern void free_ayah_data_ffi(AyahDataFFI* data);
extern AyahDataFFI* search_ayahs_by_text_ffi(const char* query, size_t limit, size_t* result_count);
extern void free_ayah_array_ffi(AyahDataFFI* data, size_t count);

int main() {
    // Initialize data
    if (!initialize_data_ffi()) {
        printf("Failed to initialize data\n");
        return 1;
    }
    
    printf("Data initialized successfully\n");
    
    // Get specific ayah
    AyahDataFFI* ayah = get_ayah_data_ffi("2:255");
    if (ayah) {
        printf("Ayah 2:255:\n");
        printf("Arabic: %s\n", ayah->arabic_text);
        printf("Translation: %s\n", ayah->translation);
        printf("Surah: %s\n", ayah->surah_name);
        
        free_ayah_data_ffi(ayah);
    }
    
    // Search for ayahs
    size_t count;
    AyahDataFFI* results = search_ayahs_by_text_ffi("الله", 5, &count);
    if (results) {
        printf("\nFound %zu ayahs containing 'الله':\n", count);
        for (size_t i = 0; i < count; i++) {
            printf("%s: %s\n", results[i].verse_key, results[i].arabic_text);
        }
        
        free_ayah_array_ffi(results, count);
    }
    
    return 0;
}
```

### JavaScript (Node.js) Example

```javascript
const ffi = require('ffi-napi');
const ref = require('ref-napi');

// Load the library
const lib = ffi.Library('./libhafiz_assistant_core.so', {
    'initialize_data_ffi': ['bool', []],
    'get_ayah_data_ffi': ['pointer', ['string']],
    'free_ayah_data_ffi': ['void', ['pointer']],
    'search_ayahs_by_text_ffi': ['pointer', ['string', 'size_t', 'pointer']],
    'free_ayah_array_ffi': ['void', ['pointer', 'size_t']]
});

// Initialize data
if (lib.initialize_data_ffi()) {
    console.log('Data initialized successfully');
    
    // Get ayah data
    const ayahPtr = lib.get_ayah_data_ffi('2:255');
    if (!ayahPtr.isNull()) {
        console.log('Got ayah data');
        
        // Free memory
        lib.free_ayah_data_ffi(ayahPtr);
    }
} else {
    console.log('Failed to initialize data');
}
```

---

## Platform-Specific Notes

### Windows
- **Library File**: `hafiz_assistant_core.dll`
- **Loading**: Use `LoadLibrary()` in C/C++ or appropriate method in other languages
- **Path**: Ensure DLL is in the same directory as executable or in system PATH
- **Calling Convention**: Uses standard C calling convention (`__cdecl`)

### Linux
- **Library File**: `libhafiz_assistant_core.so`
- **Loading**: Use `dlopen()` in C/C++ or appropriate method in other languages
- **Path**: May need to set `LD_LIBRARY_PATH` or use absolute path
- **Dependencies**: Ensure required system libraries are available

### macOS
- **Library File**: `libhafiz_assistant_core.dylib`
- **Loading**: Use `dlopen()` in C/C++ or appropriate method in other languages
- **Path**: May need to set `DYLD_LIBRARY_PATH` or use absolute path
- **Code Signing**: May need to handle code signing requirements

### Building from Source
```bash
# Build release version
cargo build --release

# Library files will be in target/release/
# Windows: hafiz_assistant_core.dll
# Linux: libhafiz_assistant_core.so
# macOS: libhafiz_assistant_core.dylib
```

---

## Performance Considerations

1. **Initialization**: First call to `initialize_data_ffi()` loads all data and may take 2-5 seconds.

2. **Memory Usage**: Each ayah structure uses approximately 1-2KB of memory.

3. **Search Performance**: 
   - Text search: Very fast (< 10ms for most queries)
   - Similar ayah search: Moderate (100-500ms depending on corpus size)
   - Advanced search: Depends on criteria complexity

4. **Caching**: Consider caching frequently accessed data in the calling application.

5. **Threading**: FFI functions are thread-safe after initialization, but avoid calling `initialize_data_ffi()` from multiple threads.

---

## Troubleshooting

### Common Issues

1. **Library Not Found**:
   - Check file path and name
   - Verify library is in correct location
   - Check platform-specific environment variables

2. **Initialization Fails**:
   - Ensure `data/` directory exists with all JSON files
   - Check file permissions
   - Verify JSON files are valid

3. **Memory Errors**:
   - Always check for NULL pointers
   - Use correct free functions for each structure type
   - Don't access freed memory

4. **Invalid Results**:
   - Validate input parameters
   - Check error codes
   - Ensure data is initialized

### Debug Information

The library provides detailed error information through:
- Return values (NULL for errors)
- Error codes (for detailed error types)
- Console output (in debug builds)

For additional debugging, build with debug symbols:
```bash
cargo build --release --features debug
```

This enables additional logging and error reporting.
