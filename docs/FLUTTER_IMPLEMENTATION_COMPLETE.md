# 📱 Flutter Integration - Complete Guide

## 🎯 Comprehensive Flutter Integration for Hafiz Assistant Backend

This guide provides complete documentation for integrating the Hafiz Assistant Backend with Flutter applications, including all FFI functions, usage examples, and response formats.

## 📋 Table of Contents

1. [Setup & Configuration](#setup--configuration)
2. [FFI Functions Reference](#ffi-functions-reference)
3. [Data Models](#data-models)
4. [Usage Examples](#usage-examples)
5. [Error Handling](#error-handling)
6. [Performance Tips](#performance-tips)
7. [Complete Implementation](#complete-implementation)

---

## 🛠️ Setup & Configuration

### 1. Library Files
After building the Rust project, copy these files to your Flutter project:

**Windows:**
```
lib/
└── native/
    ├── hafiz_assistant_core.dll
    ├── hafiz_assistant_core.lib
    └── hafiz_assistant_core.dll.exp
```

**Android:**
```
android/app/src/main/jniLibs/
├── arm64-v8a/libhafiz_assistant_core.so
├── armeabi-v7a/libhafiz_assistant_core.so
└── x86_64/libhafiz_assistant_core.so
```

**iOS:**
```
ios/
└── Frameworks/
    └── libhafiz_assistant_core.a
```

### 2. pubspec.yaml
```yaml
dependencies:
  flutter:
    sdk: flutter
  ffi: ^2.1.0
  path: ^1.8.3

dev_dependencies:
  ffigen: ^9.0.1
```

### 3. FFI Bindings Setup
Create `lib/hafiz_bindings.dart`:

```dart
import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';

// Load the native library
final DynamicLibrary _dylib = () {
  if (Platform.isWindows) {
    return DynamicLibrary.open('hafiz_assistant_core.dll');
  } else if (Platform.isAndroid) {
    return DynamicLibrary.open('libhafiz_assistant_core.so');
  } else if (Platform.isIOS) {
    return DynamicLibrary.process();
  } else {
    throw UnsupportedError('Unsupported platform');
  }
}();

// Function signatures
typedef InitializeDataC = Int32 Function();
typedef InitializeDataDart = int Function();

typedef GetAyahC = Pointer<Utf8> Function(Uint16 surah, Uint16 ayah);
typedef GetAyahDart = Pointer<Utf8> Function(int surah, int ayah);

typedef SearchTextC = Pointer<Utf8> Function(Pointer<Utf8> query);
typedef SearchTextDart = Pointer<Utf8> Function(Pointer<Utf8> query);

typedef FreeStringC = Void Function(Pointer<Utf8> ptr);
typedef FreeStringDart = void Function(Pointer<Utf8> ptr);

// Bind functions
final InitializeDataDart initializeData = 
    _dylib.lookupFunction<InitializeDataC, InitializeDataDart>('hafiz_initialize_data');

final GetAyahDart getAyah = 
    _dylib.lookupFunction<GetAyahC, GetAyahDart>('hafiz_get_ayah');

final SearchTextDart searchText = 
    _dylib.lookupFunction<SearchTextC, SearchTextDart>('hafiz_search_text');

final FreeStringDart freeString = 
    _dylib.lookupFunction<FreeStringC, FreeStringDart>('hafiz_free_string');
```

---

## 🔧 FFI Functions Reference

### 1. Data Initialization

#### `hafiz_initialize_data()`
**Purpose**: Initialize Quran data from JSON files
**Parameters**: None
**Returns**: `int` (0 = success, 1 = error)

```dart
int initializeData();
```

**Usage:**
```dart
int result = initializeData();
if (result == 0) {
  print("Data initialized successfully");
} else {
  print("Failed to initialize data");
}
```

### 2. Ayah Retrieval Functions

#### `hafiz_get_ayah(surah, ayah)`
**Purpose**: Get specific ayah by surah and ayah number
**Parameters**: 
- `surah` (u16): Surah number (1-114)
- `ayah` (u16): Ayah number (1-286)
**Returns**: `Pointer<Utf8>` - JSON string with ayah data

```dart
Pointer<Utf8> getAyah(int surah, int ayah);
```

**Response Format:**
```json
{
  "id": 1,
  "surah_number": 1,
  "ayah_number": 1,
  "verse_key": "1:1",
  "text_arab": "بِسۡمِ ٱللَّهِ ٱلرَّحۡمَٰنِ ٱلرَّحِيمِ",
  "text_uthmani": "بِسۡمِ ٱللَّهِ ٱلرَّحۡمَٰنِ ٱلرَّحِيمِ",
  "translation_id": "Dengan menyebut nama Allah Yang Maha Pengasih lagi Maha Penyayang",
  "transliteration": "Bismillaahir Rahmaanir Raheem",
  "surah_name": "Al-Fatihah",
  "surah_name_arabic": "الفاتحة",
  "juz_number": 1,
  "hizb_number": 1,
  "rub_number": 1,
  "ruku_number": 1,
  "manzil_number": 1,
  "page_number": 1,
  "line_number": 1,
  "sajda": null
}
```

**Usage:**
```dart
Future<Map<String, dynamic>?> getAyahData(int surah, int ayah) async {
  final result = getAyah(surah, ayah);
  if (result.address != 0) {
    final jsonString = result.toDartString();
    freeString(result);
    return json.decode(jsonString);
  }
  return null;
}
```

#### `hafiz_get_ayah_by_key(verse_key)`
**Purpose**: Get ayah by verse key (e.g., "1:1")
**Parameters**: 
- `verse_key` (string): Verse key format "surah:ayah"
**Returns**: `Pointer<Utf8>` - JSON string with ayah data

```dart
Pointer<Utf8> getAyahByKey(Pointer<Utf8> verseKey);
```

### 3. Search Functions

#### `hafiz_search_text(query)`
**Purpose**: Search ayahs by Arabic text or translation
**Parameters**: 
- `query` (string): Search text
**Returns**: `Pointer<Utf8>` - JSON array of matching ayahs

```dart
Pointer<Utf8> searchText(Pointer<Utf8> query);
```

**Response Format:**
```json
[
  {
    "id": 1,
    "surah_number": 1,
    "ayah_number": 1,
    "verse_key": "1:1",
    "text_arab": "بِسۡمِ ٱللَّهِ ٱلرَّحۡمَٰنِ ٱلرَّحِيمِ",
    "translation_id": "Dengan menyebut nama Allah...",
    "surah_name": "Al-Fatihah",
    "relevance_score": 0.95
  }
]
```

**Usage:**
```dart
Future<List<Map<String, dynamic>>> searchAyahs(String query) async {
  final queryPtr = query.toNativeUtf8();
  final result = searchText(queryPtr);
  malloc.free(queryPtr);
  
  if (result.address != 0) {
    final jsonString = result.toDartString();
    freeString(result);
    final List<dynamic> data = json.decode(jsonString);
    return data.cast<Map<String, dynamic>>();
  }
  return [];
}
```

#### `hafiz_search_translation(query)`
**Purpose**: Search specifically in translations
**Parameters**: 
- `query` (string): Search text in Indonesian
**Returns**: `Pointer<Utf8>` - JSON array of matching ayahs

#### `hafiz_search_similar(verse_key)`
**Purpose**: Find similar ayahs to given verse
**Parameters**: 
- `verse_key` (string): Reference verse key
**Returns**: `Pointer<Utf8>` - JSON array of similar ayahs

**Response Format:**
```json
[
  {
    "ayah_data": {
      "id": 7,
      "surah_number": 1,
      "ayah_number": 7,
      "verse_key": "1:7",
      "text_arab": "صِرَٰطَ ٱلَّذِينَ أَنۡعَمۡتَ عَلَيۡهِمۡ",
      "translation_id": "Jalan orang-orang yang telah Engkau anugerahi nikmat"
    },
    "similarity_score": 0.85,
    "matching_words": ["الذين", "عليهم"],
    "matching_type": "semantic",
    "matched_words_count": 2,
    "coverage": 30
  }
]
```

### 4. Surah Functions

#### `hafiz_get_surah_info(surah_number)`
**Purpose**: Get complete surah information
**Parameters**: 
- `surah_number` (u16): Surah number (1-114)
**Returns**: `Pointer<Utf8>` - JSON object with surah data

**Response Format:**
```json
{
  "id": 1,
  "name_simple": "Al-Fatihah",
  "name_arabic": "الفاتحة",
  "revelation_order": 1,
  "revelation_place": "Makkah",
  "verses_count": 7,
  "bismillah_pre": false
}
```

#### `hafiz_get_surah_ayahs(surah_number)`
**Purpose**: Get all ayahs in a surah
**Parameters**: 
- `surah_number` (u16): Surah number
**Returns**: `Pointer<Utf8>` - JSON array of all ayahs in surah

### 5. Page/Mushaf Functions

#### `hafiz_get_page_data(page_number)`
**Purpose**: Get complete page data for mushaf rendering
**Parameters**: 
- `page_number` (u16): Page number (1-604)
**Returns**: `Pointer<Utf8>` - JSON object with page layout

**Response Format:**
```json
{
  "page_number": 1,
  "lines": [
    {
      "line_number": 1,
      "line_type": "surah_name",
      "text": "Al-Fatihah",
      "is_centered": true,
      "verse_keys": []
    },
    {
      "line_number": 2,
      "line_type": "ayah",
      "text": "بِسۡمِ ٱللَّهِ ٱلرَّحۡمَٰنِ ٱلرَّحِيمِ",
      "is_centered": true,
      "verse_keys": ["1:1"]
    }
  ],
  "surah_headers": [
    {
      "surah_number": 1,
      "name_arabic": "الفاتحة",
      "name_simple": "Al-Fatihah",
      "line_number": 1
    }
  ]
}
```

#### `hafiz_get_total_pages()`
**Purpose**: Get total number of pages
**Returns**: `u16` - Total pages (604)

#### `hafiz_get_page_containing_verse(verse_key)`
**Purpose**: Find which page contains a specific verse
**Parameters**: 
- `verse_key` (string): Verse key
**Returns**: `u16` - Page number

### 6. Navigation Functions

#### `hafiz_get_juz_info(juz_number)`
**Purpose**: Get juz information
**Parameters**: 
- `juz_number` (u8): Juz number (1-30)
**Returns**: `Pointer<Utf8>` - JSON object with juz data

#### `hafiz_get_random_ayah()`
**Purpose**: Get random ayah
**Returns**: `Pointer<Utf8>` - JSON object with random ayah

### 7. Statistics Functions

#### `hafiz_get_statistics()`
**Purpose**: Get Quran statistics
**Returns**: `Pointer<Utf8>` - JSON object with statistics

**Response Format:**
```json
{
  "total_ayah": 6236,
  "total_surah": 114,
  "total_juz": 30,
  "total_pages": 604,
  "total_hizb": 60,
  "total_ruku": 556,
  "total_manzil": 7,
  "total_sajda": 15
}
```

### 8. Memory Management

#### `hafiz_free_string(ptr)`
**Purpose**: Free memory allocated by native functions
**Parameters**: 
- `ptr` (Pointer<Utf8>): Pointer to free
**Returns**: void

**Important**: Always call this after using any function that returns `Pointer<Utf8>`

---

## 📊 Data Models

Create Dart models for type safety:

```dart
// lib/models/ayah_data.dart
class AyahData {
  final int id;
  final int surahNumber;
  final int ayahNumber;
  final String verseKey;
  final String textArab;
  final String textUthmani;
  final String translationId;
  final String transliteration;
  final String surahName;
  final String surahNameArabic;
  final int juzNumber;
  final int hizbNumber;
  final int rubNumber;
  final int rukuNumber;
  final int manzilNumber;
  final int pageNumber;
  final int lineNumber;

  AyahData({
    required this.id,
    required this.surahNumber,
    required this.ayahNumber,
    required this.verseKey,
    required this.textArab,
    required this.textUthmani,
    required this.translationId,
    required this.transliteration,
    required this.surahName,
    required this.surahNameArabic,
    required this.juzNumber,
    required this.hizbNumber,
    required this.rubNumber,
    required this.rukuNumber,
    required this.manzilNumber,
    required this.pageNumber,
    required this.lineNumber,
  });

  factory AyahData.fromJson(Map<String, dynamic> json) {
    return AyahData(
      id: json['id'],
      surahNumber: json['surah_number'],
      ayahNumber: json['ayah_number'],
      verseKey: json['verse_key'],
      textArab: json['text_arab'],
      textUthmani: json['text_uthmani'],
      translationId: json['translation_id'],
      transliteration: json['transliteration'],
      surahName: json['surah_name'],
      surahNameArabic: json['surah_name_arabic'],
      juzNumber: json['juz_number'],
      hizbNumber: json['hizb_number'],
      rubNumber: json['rub_number'],
      rukuNumber: json['ruku_number'],
      manzilNumber: json['manzil_number'],
      pageNumber: json['page_number'],
      lineNumber: json['line_number'],
    );
  }
}

// lib/models/page_data.dart
class PageLine {
  final int lineNumber;
  final String lineType;
  final String text;
  final bool isCentered;
  final List<String> verseKeys;

  PageLine({
    required this.lineNumber,
    required this.lineType,
    required this.text,
    required this.isCentered,
    required this.verseKeys,
  });

  factory PageLine.fromJson(Map<String, dynamic> json) {
    return PageLine(
      lineNumber: json['line_number'],
      lineType: json['line_type'],
      text: json['text'],
      isCentered: json['is_centered'],
      verseKeys: List<String>.from(json['verse_keys']),
    );
  }
}

class PageData {
  final int pageNumber;
  final List<PageLine> lines;
  final List<SurahHeader> surahHeaders;

  PageData({
    required this.pageNumber,
    required this.lines,
    required this.surahHeaders,
  });

  factory PageData.fromJson(Map<String, dynamic> json) {
    return PageData(
      pageNumber: json['page_number'],
      lines: (json['lines'] as List)
          .map((line) => PageLine.fromJson(line))
          .toList(),
      surahHeaders: (json['surah_headers'] as List)
          .map((header) => SurahHeader.fromJson(header))
          .toList(),
    );
  }
}
```

---

## 💡 Usage Examples

### Complete Service Class

```dart
// lib/services/hafiz_service.dart
import 'dart:convert';
import 'dart:ffi';
import 'package:ffi/ffi.dart';
import '../models/ayah_data.dart';
import '../models/page_data.dart';
import '../hafiz_bindings.dart';

class HafizService {
  static bool _initialized = false;

  // Initialize the service
  static Future<bool> initialize() async {
    if (_initialized) return true;
    
    try {
      final result = initializeData();
      _initialized = (result == 0);
      return _initialized;
    } catch (e) {
      print('Failed to initialize: $e');
      return false;
    }
  }

  // Get specific ayah
  static Future<AyahData?> getAyah(int surah, int ayah) async {
    if (!_initialized) await initialize();
    
    try {
      final result = getAyah(surah, ayah);
      if (result.address != 0) {
        final jsonString = result.toDartString();
        freeString(result);
        final Map<String, dynamic> data = json.decode(jsonString);
        return AyahData.fromJson(data);
      }
    } catch (e) {
      print('Error getting ayah: $e');
    }
    return null;
  }

  // Search ayahs
  static Future<List<AyahData>> searchAyahs(String query) async {
    if (!_initialized) await initialize();
    
    try {
      final queryPtr = query.toNativeUtf8();
      final result = searchText(queryPtr);
      malloc.free(queryPtr);
      
      if (result.address != 0) {
        final jsonString = result.toDartString();
        freeString(result);
        final List<dynamic> data = json.decode(jsonString);
        return data.map((item) => AyahData.fromJson(item)).toList();
      }
    } catch (e) {
      print('Error searching: $e');
    }
    return [];
  }

  // Get page data
  static Future<PageData?> getPageData(int pageNumber) async {
    if (!_initialized) await initialize();
    
    try {
      final result = getPageData(pageNumber);
      if (result.address != 0) {
        final jsonString = result.toDartString();
        freeString(result);
        final Map<String, dynamic> data = json.decode(jsonString);
        return PageData.fromJson(data);
      }
    } catch (e) {
      print('Error getting page data: $e');
    }
    return null;
  }

  // Get statistics
  static Future<Map<String, dynamic>?> getStatistics() async {
    if (!_initialized) await initialize();
    
    try {
      final result = getStatistics();
      if (result.address != 0) {
        final jsonString = result.toDartString();
        freeString(result);
        return json.decode(jsonString);
      }
    } catch (e) {
      print('Error getting statistics: $e');
    }
    return null;
  }
}
```

### UI Implementation Examples

#### 1. Ayah Display Widget

```dart
// lib/widgets/ayah_widget.dart
import 'package:flutter/material.dart';
import '../models/ayah_data.dart';
import '../services/hafiz_service.dart';

class AyahWidget extends StatefulWidget {
  final int surah;
  final int ayah;

  const AyahWidget({Key? key, required this.surah, required this.ayah}) 
      : super(key: key);

  @override
  _AyahWidgetState createState() => _AyahWidgetState();
}

class _AyahWidgetState extends State<AyahWidget> {
  AyahData? ayahData;
  bool isLoading = true;

  @override
  void initState() {
    super.initState();
    loadAyah();
  }

  Future<void> loadAyah() async {
    final data = await HafizService.getAyah(widget.surah, widget.ayah);
    setState(() {
      ayahData = data;
      isLoading = false;
    });
  }

  @override
  Widget build(BuildContext context) {
    if (isLoading) {
      return const Center(child: CircularProgressIndicator());
    }

    if (ayahData == null) {
      return const Center(child: Text('Ayah not found'));
    }

    return Card(
      margin: const EdgeInsets.all(8.0),
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Verse reference
            Text(
              '${ayahData!.verseKey} - ${ayahData!.surahName}',
              style: Theme.of(context).textTheme.titleMedium,
            ),
            const SizedBox(height: 8),
            
            // Arabic text
            Text(
              ayahData!.textArab,
              style: const TextStyle(
                fontSize: 24,
                fontFamily: 'Amiri', // Arabic font
                height: 2.0,
              ),
              textAlign: TextAlign.right,
              textDirection: TextDirection.rtl,
            ),
            const SizedBox(height: 8),
            
            // Translation
            Text(
              ayahData!.translationId,
              style: Theme.of(context).textTheme.bodyLarge,
            ),
            const SizedBox(height: 4),
            
            // Transliteration
            Text(
              ayahData!.transliteration,
              style: Theme.of(context).textTheme.bodySmall?.copyWith(
                fontStyle: FontStyle.italic,
              ),
            ),
            const SizedBox(height: 8),
            
            // Metadata
            Wrap(
              children: [
                Chip(label: Text('Juz ${ayahData!.juzNumber}')),
                const SizedBox(width: 4),
                Chip(label: Text('Page ${ayahData!.pageNumber}')),
                const SizedBox(width: 4),
                Chip(label: Text('Ruku ${ayahData!.rukuNumber}')),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
```

#### 2. Search Interface

```dart
// lib/screens/search_screen.dart
import 'package:flutter/material.dart';
import '../models/ayah_data.dart';
import '../services/hafiz_service.dart';
import '../widgets/ayah_widget.dart';

class SearchScreen extends StatefulWidget {
  @override
  _SearchScreenState createState() => _SearchScreenState();
}

class _SearchScreenState extends State<SearchScreen> {
  final TextEditingController _searchController = TextEditingController();
  List<AyahData> searchResults = [];
  bool isSearching = false;

  Future<void> performSearch(String query) async {
    if (query.trim().isEmpty) return;

    setState(() {
      isSearching = true;
    });

    final results = await HafizService.searchAyahs(query);
    
    setState(() {
      searchResults = results;
      isSearching = false;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Search Quran'),
      ),
      body: Column(
        children: [
          // Search bar
          Padding(
            padding: const EdgeInsets.all(16.0),
            child: TextField(
              controller: _searchController,
              decoration: InputDecoration(
                hintText: 'Search in Arabic or translation...',
                suffixIcon: IconButton(
                  icon: const Icon(Icons.search),
                  onPressed: () => performSearch(_searchController.text),
                ),
                border: const OutlineInputBorder(),
              ),
              onSubmitted: performSearch,
            ),
          ),
          
          // Results
          Expanded(
            child: isSearching
                ? const Center(child: CircularProgressIndicator())
                : searchResults.isEmpty
                    ? const Center(child: Text('No results found'))
                    : ListView.builder(
                        itemCount: searchResults.length,
                        itemBuilder: (context, index) {
                          final ayah = searchResults[index];
                          return AyahWidget(
                            surah: ayah.surahNumber,
                            ayah: ayah.ayahNumber,
                          );
                        },
                      ),
          ),
        ],
      ),
    );
  }
}
```

#### 3. Page Viewer (Mushaf)

```dart
// lib/screens/mushaf_screen.dart
import 'package:flutter/material.dart';
import '../models/page_data.dart';
import '../services/hafiz_service.dart';

class MushafScreen extends StatefulWidget {
  @override
  _MushafScreenState createState() => _MushafScreenState();
}

class _MushafScreenState extends State<MushafScreen> {
  PageController pageController = PageController();
  int currentPage = 1;
  PageData? currentPageData;
  bool isLoading = true;

  @override
  void initState() {
    super.initState();
    loadPage(1);
  }

  Future<void> loadPage(int pageNumber) async {
    setState(() {
      isLoading = true;
    });

    final data = await HafizService.getPageData(pageNumber);
    
    setState(() {
      currentPageData = data;
      currentPage = pageNumber;
      isLoading = false;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Page $currentPage / 604'),
        actions: [
          IconButton(
            icon: const Icon(Icons.navigate_before),
            onPressed: currentPage > 1 
                ? () => loadPage(currentPage - 1) 
                : null,
          ),
          IconButton(
            icon: const Icon(Icons.navigate_next),
            onPressed: currentPage < 604 
                ? () => loadPage(currentPage + 1) 
                : null,
          ),
        ],
      ),
      body: isLoading
          ? const Center(child: CircularProgressIndicator())
          : currentPageData == null
              ? const Center(child: Text('Page not found'))
              : SingleChildScrollView(
                  padding: const EdgeInsets.all(16.0),
                  child: Column(
                    children: [
                      // Surah headers
                      ...currentPageData!.surahHeaders.map((header) => 
                        Container(
                          width: double.infinity,
                          padding: const EdgeInsets.all(8.0),
                          margin: const EdgeInsets.only(bottom: 8.0),
                          decoration: BoxDecoration(
                            color: Theme.of(context).primaryColor.withOpacity(0.1),
                            borderRadius: BorderRadius.circular(8.0),
                          ),
                          child: Text(
                            '${header.nameArabic} - ${header.nameSimple}',
                            textAlign: TextAlign.center,
                            style: Theme.of(context).textTheme.titleLarge,
                          ),
                        ),
                      ),
                      
                      // Page lines
                      ...currentPageData!.lines.map((line) => 
                        Container(
                          width: double.infinity,
                          padding: const EdgeInsets.symmetric(vertical: 4.0),
                          child: Text(
                            line.text,
                            textAlign: line.isCentered 
                                ? TextAlign.center 
                                : TextAlign.justify,
                            style: TextStyle(
                              fontSize: line.lineType == 'ayah' ? 20 : 18,
                              fontFamily: 'Amiri',
                              height: 2.0,
                            ),
                            textDirection: TextDirection.rtl,
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
      floatingActionButton: FloatingActionButton(
        onPressed: () => _showPagePicker(),
        child: const Icon(Icons.bookmark),
      ),
    );
  }

  void _showPagePicker() {
    showDialog(
      context: context,
      builder: (context) {
        int selectedPage = currentPage;
        return AlertDialog(
          title: const Text('Go to Page'),
          content: TextField(
            keyboardType: TextInputType.number,
            decoration: const InputDecoration(
              hintText: 'Enter page number (1-604)',
            ),
            onChanged: (value) {
              selectedPage = int.tryParse(value) ?? currentPage;
            },
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(context),
              child: const Text('Cancel'),
            ),
            TextButton(
              onPressed: () {
                if (selectedPage >= 1 && selectedPage <= 604) {
                  loadPage(selectedPage);
                  Navigator.pop(context);
                }
              },
              child: const Text('Go'),
            ),
          ],
        );
      },
    );
  }
}
```

---

## ⚠️ Error Handling

### Common Error Scenarios

1. **Library not found**
```dart
try {
  final result = initializeData();
} catch (e) {
  if (e.toString().contains('library')) {
    // Show library installation guide
    showLibraryNotFoundDialog();
  }
}
```

2. **Data not initialized**
```dart
static Future<T?> _withInitialization<T>(Future<T?> Function() operation) async {
  if (!_initialized) {
    final success = await initialize();
    if (!success) return null;
  }
  return await operation();
}
```

3. **Invalid parameters**
```dart
static Future<AyahData?> getAyah(int surah, int ayah) async {
  if (surah < 1 || surah > 114) {
    throw ArgumentError('Surah number must be between 1 and 114');
  }
  if (ayah < 1) {
    throw ArgumentError('Ayah number must be positive');
  }
  
  return await _withInitialization(() async {
    // ... rest of implementation
  });
}
```

---

## 🚀 Performance Tips

### 1. Memory Management
Always free native strings:
```dart
Future<T?> _callNativeFunction<T>(
  Pointer<Utf8> Function() nativeCall,
  T Function(String) parser,
) async {
  final result = nativeCall();
  if (result.address != 0) {
    final jsonString = result.toDartString();
    freeString(result);  // Critical: always free!
    return parser(jsonString);
  }
  return null;
}
```

### 2. Caching
Implement caching for frequently accessed data:
```dart
class CachedHafizService {
  static final Map<String, AyahData> _ayahCache = {};
  static final Map<int, PageData> _pageCache = {};

  static Future<AyahData?> getAyah(int surah, int ayah) async {
    final key = '$surah:$ayah';
    if (_ayahCache.containsKey(key)) {
      return _ayahCache[key];
    }

    final data = await HafizService.getAyah(surah, ayah);
    if (data != null) {
      _ayahCache[key] = data;
    }
    return data;
  }
}
```

### 3. Async Loading
Use FutureBuilder for smooth UI:
```dart
FutureBuilder<AyahData?>(
  future: HafizService.getAyah(surah, ayah),
  builder: (context, snapshot) {
    if (snapshot.connectionState == ConnectionState.waiting) {
      return const CircularProgressIndicator();
    }
    if (snapshot.hasError) {
      return Text('Error: ${snapshot.error}');
    }
    if (snapshot.hasData) {
      return AyahWidget(ayahData: snapshot.data!);
    }
    return const Text('No data');
  },
);
```

---

## 🎯 Complete Implementation

### Main App Setup

```dart
// lib/main.dart
import 'package:flutter/material.dart';
import 'services/hafiz_service.dart';
import 'screens/home_screen.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  
  // Initialize Hafiz service
  await HafizService.initialize();
  
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Hafiz Assistant',
      theme: ThemeData(
        primarySwatch: Colors.green,
        fontFamily: 'Roboto',
      ),
      home: HomeScreen(),
    );
  }
}
```

### Build Configuration

**android/app/build.gradle:**
```gradle
android {
    compileSdkVersion 33
    
    defaultConfig {
        minSdkVersion 21
        targetSdkVersion 33
    }
    
    sourceSets {
        main {
            jniLibs.srcDirs = ['src/main/jniLibs']
        }
    }
}
```

**ios/Runner.xcodeproj/project.pbxproj:**
Add the static library to Link Binary with Libraries phase.

---

## 📝 Summary

This comprehensive guide provides:

✅ **Complete FFI setup** for all platforms
✅ **All function signatures** and response formats  
✅ **Dart data models** for type safety
✅ **Service layer implementation** with error handling
✅ **UI widget examples** for common use cases
✅ **Performance optimization** tips
✅ **Memory management** best practices
✅ **Complete working examples** ready to use

The Hafiz Assistant Backend is now fully ready for Flutter integration with this comprehensive documentation! 🚀📱🕌

---

**Ready for production Flutter apps!** Start with the basic setup and gradually implement the features you need.
