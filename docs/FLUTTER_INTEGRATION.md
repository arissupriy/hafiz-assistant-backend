# Flutter Integration Guide for Hafiz Assistant Backend

## Overview

This document provides comprehensive guidance for integrating the Hafiz Assistant Backend with Flutter applications. The backend provides FFI (Foreign Function Interface) capabilities that allow Flutter/Dart applications to directly access all Quran data and features.

## Table of Contents

1. [Quick Setup](#quick-setup)
2. [FFI Structure Overview](#ffi-structure-overview)
3. [Function Reference](#function-reference)
4. [Data Structures](#data-structures)
5. [Example Integration](#example-integration)
6. [Error Handling](#error-handling)
7. [Memory Management](#memory-management)
8. [Best Practices](#best-practices)

## Quick Setup

### 1. Build the Dynamic Library

```bash
# Build the release version
cargo build --release

# The dynamic library will be created at:
# target/release/hafiz_assistant_core.dll (Windows)
# target/release/libhafiz_assistant_core.so (Linux)
# target/release/libhafiz_assistant_core.dylib (macOS)
```

### 2. Add to Flutter Project

1. Copy the dynamic library to your Flutter project's platform-specific directories:
   - Windows: `windows/` or `build/windows/runner/Release/`
   - Linux: `linux/` or `build/linux/x64/release/bundle/lib/`
   - macOS: `macos/` or `build/macos/Build/Products/Release/`

2. Add FFI dependency to your `pubspec.yaml`:
```yaml
dependencies:
  ffi: ^2.1.0
```

### 3. Create Dart Bindings

```dart
import 'dart:ffi' as ffi;
import 'dart:io' show Platform;
import 'package:ffi/ffi.dart';

class HafizAssistantBindings {
  static late ffi.DynamicLibrary _dylib;
  
  static void initialize() {
    if (Platform.isWindows) {
      _dylib = ffi.DynamicLibrary.open('hafiz_assistant_core.dll');
    } else if (Platform.isLinux) {
      _dylib = ffi.DynamicLibrary.open('libhafiz_assistant_core.so');
    } else if (Platform.isMacOS) {
      _dylib = ffi.DynamicLibrary.open('libhafiz_assistant_core.dylib');
    }
  }
  
  // Function bindings will be added here
}
```

## FFI Structure Overview

The backend provides the following main FFI structures:

- **AyahDataFFI**: Complete ayah information with all metadata
- **SurahInfoFFI**: Surah information and metadata
- **SimilarAyahWithTextFFI**: Similar ayah results with similarity scores
- **QuranStatisticsFFI**: Overall Quran statistics
- **AdvancedSearchCriteriaFFI**: Advanced search parameters
- **ErrorFFI**: Error handling information

## Function Reference

### Core Functions

#### `initialize_data_ffi()`
Initializes the Quran data. Must be called before using any other functions.

**C Signature:**
```c
bool initialize_data_ffi();
```

**Dart Binding:**
```dart
late final _initializeData = _dylib.lookupFunction<
    ffi.Bool Function(),
    bool Function()
>('initialize_data_ffi');

bool initializeData() => _initializeData();
```

**Returns:** `true` if successful, `false` otherwise.

**Example:**
```dart
if (HafizAssistantBindings.initializeData()) {
  print('Data initialized successfully');
} else {
  print('Failed to initialize data');
}
```

---

#### `get_ayah_data_ffi(verse_key)`
Gets complete ayah data by verse key.

**C Signature:**
```c
AyahDataFFI* get_ayah_data_ffi(const char* verse_key);
```

**Dart Binding:**
```dart
late final _getAyahData = _dylib.lookupFunction<
    ffi.Pointer<AyahDataFFI> Function(ffi.Pointer<Utf8>),
    ffi.Pointer<AyahDataFFI> Function(ffi.Pointer<Utf8>)
>('get_ayah_data_ffi');

AyahData? getAyahData(String verseKey) {
  final verseKeyPtr = verseKey.toNativeUtf8();
  final result = _getAyahData(verseKeyPtr);
  calloc.free(verseKeyPtr);
  
  if (result.address == 0) return null;
  
  final ayahData = AyahData.fromFFI(result.ref);
  _freeAyahData(result);
  return ayahData;
}
```

**Parameters:**
- `verse_key`: String in format "surah:ayah" (e.g., "1:1", "2:255")

**Returns:** Pointer to `AyahDataFFI` structure or null if not found.

**Example:**
```dart
final ayah = HafizAssistantBindings.getAyahData('2:255');
if (ayah != null) {
  print('Arabic: ${ayah.arabicText}');
  print('Translation: ${ayah.translation}');
  print('Surah: ${ayah.surahName}');
}
```

---

#### `search_ayahs_by_text_ffi(query, limit, result_count)`
Search ayahs by Arabic text.

**C Signature:**
```c
AyahDataFFI* search_ayahs_by_text_ffi(const char* query, size_t limit, size_t* result_count);
```

**Dart Binding:**
```dart
late final _searchAyahsByText = _dylib.lookupFunction<
    ffi.Pointer<AyahDataFFI> Function(ffi.Pointer<Utf8>, ffi.Size, ffi.Pointer<ffi.Size>),
    ffi.Pointer<AyahDataFFI> Function(ffi.Pointer<Utf8>, int, ffi.Pointer<ffi.Size>)
>('search_ayahs_by_text_ffi');

List<AyahData> searchAyahsByText(String query, int limit) {
  final queryPtr = query.toNativeUtf8();
  final countPtr = calloc<ffi.Size>();
  
  final result = _searchAyahsByText(queryPtr, limit, countPtr);
  final count = countPtr.value;
  
  calloc.free(queryPtr);
  calloc.free(countPtr);
  
  if (result.address == 0) return [];
  
  final ayahs = <AyahData>[];
  for (int i = 0; i < count; i++) {
    ayahs.add(AyahData.fromFFI(result[i]));
  }
  
  _freeAyahArray(result, count);
  return ayahs;
}
```

**Parameters:**
- `query`: Arabic text to search for
- `limit`: Maximum number of results to return

**Returns:** Array of `AyahDataFFI` structures.

**Example:**
```dart
final results = HafizAssistantBindings.searchAyahsByText('الله', 10);
print('Found ${results.length} ayahs containing "الله"');
for (final ayah in results) {
  print('${ayah.verseKey}: ${ayah.arabicText}');
}
```

---

#### `search_similar_ayahs_ffi(verse_key, limit, result_count)`
Find ayahs similar to a given verse.

**C Signature:**
```c
SimilarAyahWithTextFFI* search_similar_ayahs_ffi(const char* verse_key, size_t limit, size_t* result_count);
```

**Dart Binding:**
```dart
late final _searchSimilarAyahs = _dylib.lookupFunction<
    ffi.Pointer<SimilarAyahWithTextFFI> Function(ffi.Pointer<Utf8>, ffi.Size, ffi.Pointer<ffi.Size>),
    ffi.Pointer<SimilarAyahWithTextFFI> Function(ffi.Pointer<Utf8>, int, ffi.Pointer<ffi.Size>)
>('search_similar_ayahs_ffi');

List<SimilarAyahWithText> searchSimilarAyahs(String verseKey, int limit) {
  final verseKeyPtr = verseKey.toNativeUtf8();
  final countPtr = calloc<ffi.Size>();
  
  final result = _searchSimilarAyahs(verseKeyPtr, limit, countPtr);
  final count = countPtr.value;
  
  calloc.free(verseKeyPtr);
  calloc.free(countPtr);
  
  if (result.address == 0) return [];
  
  final ayahs = <SimilarAyahWithText>[];
  for (int i = 0; i < count; i++) {
    ayahs.add(SimilarAyahWithText.fromFFI(result[i]));
  }
  
  _freeSimilarAyahs(result, count);
  return ayahs;
}
```

**Parameters:**
- `verse_key`: Reference verse key (e.g., "2:255")
- `limit`: Maximum number of similar ayahs to return

**Returns:** Array of `SimilarAyahWithTextFFI` structures with similarity scores.

**Example:**
```dart
final similar = HafizAssistantBindings.searchSimilarAyahs('2:255', 5);
print('Found ${similar.length} similar ayahs:');
for (final ayah in similar) {
  print('${ayah.verseKey}: ${ayah.similarityScore.toStringAsFixed(2)}');
}
```

---

#### `get_surah_info_ffi(surah_number)`
Get detailed surah information.

**C Signature:**
```c
SurahInfoFFI* get_surah_info_ffi(uint16_t surah_number);
```

**Dart Binding:**
```dart
late final _getSurahInfo = _dylib.lookupFunction<
    ffi.Pointer<SurahInfoFFI> Function(ffi.Uint16),
    ffi.Pointer<SurahInfoFFI> Function(int)
>('get_surah_info_ffi');

SurahInfo? getSurahInfo(int surahNumber) {
  final result = _getSurahInfo(surahNumber);
  
  if (result.address == 0) return null;
  
  final surahInfo = SurahInfo.fromFFI(result.ref);
  _freeSurahInfo(result);
  return surahInfo;
}
```

**Parameters:**
- `surah_number`: Surah number (1-114)

**Returns:** Pointer to `SurahInfoFFI` structure or null if not found.

**Example:**
```dart
final surahInfo = HafizAssistantBindings.getSurahInfo(2);
if (surahInfo != null) {
  print('Surah: ${surahInfo.nameEnglish}');
  print('Arabic: ${surahInfo.nameArabic}');
  print('Verses: ${surahInfo.versesCount}');
  print('Revelation: ${surahInfo.revelationPlace}');
}
```

---

#### `get_ayahs_by_surah_ffi(surah_number, result_count)`
Get all ayahs in a specific surah.

**C Signature:**
```c
AyahDataFFI* get_ayahs_by_surah_ffi(uint16_t surah_number, size_t* result_count);
```

**Dart Binding:**
```dart
late final _getAyahsBySurah = _dylib.lookupFunction<
    ffi.Pointer<AyahDataFFI> Function(ffi.Uint16, ffi.Pointer<ffi.Size>),
    ffi.Pointer<AyahDataFFI> Function(int, ffi.Pointer<ffi.Size>)
>('get_ayahs_by_surah_ffi');

List<AyahData> getAyahsBySurah(int surahNumber) {
  final countPtr = calloc<ffi.Size>();
  final result = _getAyahsBySurah(surahNumber, countPtr);
  final count = countPtr.value;
  
  calloc.free(countPtr);
  
  if (result.address == 0) return [];
  
  final ayahs = <AyahData>[];
  for (int i = 0; i < count; i++) {
    ayahs.add(AyahData.fromFFI(result[i]));
  }
  
  _freeAyahArray(result, count);
  return ayahs;
}
```

**Parameters:**
- `surah_number`: Surah number (1-114)

**Returns:** Array of all ayahs in the surah.

**Example:**
```dart
final ayahs = HafizAssistantBindings.getAyahsBySurah(1);
print('Surah Al-Fatihah has ${ayahs.length} ayahs:');
for (final ayah in ayahs) {
  print('${ayah.ayahNumber}: ${ayah.arabicText}');
}
```

---

#### `get_ayahs_by_juz_ffi(juz_number, result_count)`
Get all ayahs in a specific juz (para).

**C Signature:**
```c
AyahDataFFI* get_ayahs_by_juz_ffi(uint8_t juz_number, size_t* result_count);
```

**Dart Binding:**
```dart
late final _getAyahsByJuz = _dylib.lookupFunction<
    ffi.Pointer<AyahDataFFI> Function(ffi.Uint8, ffi.Pointer<ffi.Size>),
    ffi.Pointer<AyahDataFFI> Function(int, ffi.Pointer<ffi.Size>)
>('get_ayahs_by_juz_ffi');

List<AyahData> getAyahsByJuz(int juzNumber) {
  final countPtr = calloc<ffi.Size>();
  final result = _getAyahsByJuz(juzNumber, countPtr);
  final count = countPtr.value;
  
  calloc.free(countPtr);
  
  if (result.address == 0) return [];
  
  final ayahs = <AyahData>[];
  for (int i = 0; i < count; i++) {
    ayahs.add(AyahData.fromFFI(result[i]));
  }
  
  _freeAyahArray(result, count);
  return ayahs;
}
```

**Parameters:**
- `juz_number`: Juz number (1-30)

**Returns:** Array of all ayahs in the juz.

**Example:**
```dart
final ayahs = HafizAssistantBindings.getAyahsByJuz(1);
print('Juz 1 has ${ayahs.length} ayahs');
```

---

#### `get_ayahs_by_page_ffi(page_number, result_count)`
Get all ayahs on a specific Mushaf page.

**C Signature:**
```c
AyahDataFFI* get_ayahs_by_page_ffi(uint16_t page_number, size_t* result_count);
```

**Dart Binding:**
```dart
late final _getAyahsByPage = _dylib.lookupFunction<
    ffi.Pointer<AyahDataFFI> Function(ffi.Uint16, ffi.Pointer<ffi.Size>),
    ffi.Pointer<AyahDataFFI> Function(int, ffi.Pointer<ffi.Size>)
>('get_ayahs_by_page_ffi');

List<AyahData> getAyahsByPage(int pageNumber) {
  final countPtr = calloc<ffi.Size>();
  final result = _getAyahsByPage(pageNumber, countPtr);
  final count = countPtr.value;
  
  calloc.free(countPtr);
  
  if (result.address == 0) return [];
  
  final ayahs = <AyahData>[];
  for (int i = 0; i < count; i++) {
    ayahs.add(AyahData.fromFFI(result[i]));
  }
  
  _freeAyahArray(result, count);
  return ayahs;
}
```

**Parameters:**
- `page_number`: Mushaf page number (1-604)

**Returns:** Array of all ayahs on the page.

**Example:**
```dart
final ayahs = HafizAssistantBindings.getAyahsByPage(1);
print('Page 1 has ${ayahs.length} ayahs');
for (final ayah in ayahs) {
  print('${ayah.verseKey}: ${ayah.arabicText}');
}
```

---

#### `advanced_search_ffi(criteria, result_count)`
Perform advanced search with multiple criteria.

**C Signature:**
```c
AyahDataFFI* advanced_search_ffi(const AdvancedSearchCriteriaFFI* criteria, size_t* result_count);
```

**Dart Binding:**
```dart
late final _advancedSearch = _dylib.lookupFunction<
    ffi.Pointer<AyahDataFFI> Function(ffi.Pointer<AdvancedSearchCriteriaFFI>, ffi.Pointer<ffi.Size>),
    ffi.Pointer<AyahDataFFI> Function(ffi.Pointer<AdvancedSearchCriteriaFFI>, ffi.Pointer<ffi.Size>)
>('advanced_search_ffi');

List<AyahData> advancedSearch(AdvancedSearchCriteria criteria) {
  final criteriaPtr = criteria.toFFI();
  final countPtr = calloc<ffi.Size>();
  
  final result = _advancedSearch(criteriaPtr, countPtr);
  final count = countPtr.value;
  
  criteria.freeFFI(criteriaPtr);
  calloc.free(countPtr);
  
  if (result.address == 0) return [];
  
  final ayahs = <AyahData>[];
  for (int i = 0; i < count; i++) {
    ayahs.add(AyahData.fromFFI(result[i]));
  }
  
  _freeAyahArray(result, count);
  return ayahs;
}
```

**Parameters:**
- `criteria`: Advanced search criteria structure

**Returns:** Array of ayahs matching the criteria.

**Example:**
```dart
final criteria = AdvancedSearchCriteria(
  textQuery: 'الله',
  translationQuery: 'Allah',
  surahNumber: 2,
  limit: 10,
);

final results = HafizAssistantBindings.advancedSearch(criteria);
print('Found ${results.length} ayahs matching criteria');
```

---

#### `get_quran_statistics_ffi()`
Get overall Quran statistics.

**C Signature:**
```c
QuranStatisticsFFI* get_quran_statistics_ffi();
```

**Dart Binding:**
```dart
late final _getQuranStatistics = _dylib.lookupFunction<
    ffi.Pointer<QuranStatisticsFFI> Function(),
    ffi.Pointer<QuranStatisticsFFI> Function()
>('get_quran_statistics_ffi');

QuranStatistics? getQuranStatistics() {
  final result = _getQuranStatistics();
  
  if (result.address == 0) return null;
  
  final stats = QuranStatistics.fromFFI(result.ref);
  _freeQuranStatistics(result);
  return stats;
}
```

**Returns:** Pointer to `QuranStatisticsFFI` structure.

**Example:**
```dart
final stats = HafizAssistantBindings.getQuranStatistics();
if (stats != null) {
  print('Total Surahs: ${stats.totalSurahs}');
  print('Total Ayahs: ${stats.totalAyahs}');
  print('Total Words: ${stats.totalWords}');
  print('Total Pages: ${stats.totalPages}');
}
```

---

#### `get_random_ayah_ffi()`
Get a random ayah from the Quran.

**C Signature:**
```c
AyahDataFFI* get_random_ayah_ffi();
```

**Dart Binding:**
```dart
late final _getRandomAyah = _dylib.lookupFunction<
    ffi.Pointer<AyahDataFFI> Function(),
    ffi.Pointer<AyahDataFFI> Function()
>('get_random_ayah_ffi');

AyahData? getRandomAyah() {
  final result = _getRandomAyah();
  
  if (result.address == 0) return null;
  
  final ayah = AyahData.fromFFI(result.ref);
  _freeAyahData(result);
  return ayah;
}
```

**Returns:** Pointer to `AyahDataFFI` structure.

**Example:**
```dart
final randomAyah = HafizAssistantBindings.getRandomAyah();
if (randomAyah != null) {
  print('Random Ayah: ${randomAyah.verseKey}');
  print('Arabic: ${randomAyah.arabicText}');
  print('Translation: ${randomAyah.translation}');
}
```

---

#### `validate_verse_key_ffi(verse_key)`
Validate a verse key format and existence.

**C Signature:**
```c
bool validate_verse_key_ffi(const char* verse_key);
```

**Dart Binding:**
```dart
late final _validateVerseKey = _dylib.lookupFunction<
    ffi.Bool Function(ffi.Pointer<Utf8>),
    bool Function(ffi.Pointer<Utf8>)
>('validate_verse_key_ffi');

bool validateVerseKey(String verseKey) {
  final verseKeyPtr = verseKey.toNativeUtf8();
  final result = _validateVerseKey(verseKeyPtr);
  calloc.free(verseKeyPtr);
  return result;
}
```

**Parameters:**
- `verse_key`: String in format "surah:ayah"

**Returns:** `true` if valid, `false` otherwise.

**Example:**
```dart
if (HafizAssistantBindings.validateVerseKey('2:255')) {
  print('Valid verse key');
} else {
  print('Invalid verse key');
}
```

## Data Structures

### AyahData (Dart)
```dart
class AyahData {
  final String verseKey;
  final String arabicText;
  final String translation;
  final String transliteration;
  final String surahName;
  final String surahNameArabic;
  final int surahNumber;
  final int ayahNumber;
  final int juzNumber;
  final int hizbNumber;
  final int rubNumber;
  final int rukuNumber;
  final int manzilNumber;
  final int pageNumber;
  
  AyahData({
    required this.verseKey,
    required this.arabicText,
    required this.translation,
    required this.transliteration,
    required this.surahName,
    required this.surahNameArabic,
    required this.surahNumber,
    required this.ayahNumber,
    required this.juzNumber,
    required this.hizbNumber,
    required this.rubNumber,
    required this.rukuNumber,
    required this.manzilNumber,
    required this.pageNumber,
  });
  
  factory AyahData.fromFFI(AyahDataFFI ffi) {
    return AyahData(
      verseKey: ffi.verse_key.toDartString(),
      arabicText: ffi.arabic_text.toDartString(),
      translation: ffi.translation.toDartString(),
      transliteration: ffi.transliteration.toDartString(),
      surahName: ffi.surah_name.toDartString(),
      surahNameArabic: ffi.surah_name_arabic.toDartString(),
      surahNumber: ffi.surah_number,
      ayahNumber: ffi.ayah_number,
      juzNumber: ffi.juz_number,
      hizbNumber: ffi.hizb_number,
      rubNumber: ffi.rub_number,
      rukuNumber: ffi.ruku_number,
      manzilNumber: ffi.manzil_number,
      pageNumber: ffi.page_number,
    );
  }
}
```

### SurahInfo (Dart)
```dart
class SurahInfo {
  final int id;
  final String nameSimple;
  final String nameArabic;
  final String nameEnglish;
  final int revelationOrder;
  final String revelationPlace;
  final int versesCount;
  final bool bismillahPre;
  
  SurahInfo({
    required this.id,
    required this.nameSimple,
    required this.nameArabic,
    required this.nameEnglish,
    required this.revelationOrder,
    required this.revelationPlace,
    required this.versesCount,
    required this.bismillahPre,
  });
  
  factory SurahInfo.fromFFI(SurahInfoFFI ffi) {
    return SurahInfo(
      id: ffi.id,
      nameSimple: ffi.name_simple.toDartString(),
      nameArabic: ffi.name_arabic.toDartString(),
      nameEnglish: ffi.name_english.toDartString(),
      revelationOrder: ffi.revelation_order,
      revelationPlace: ffi.revelation_place.toDartString(),
      versesCount: ffi.verses_count,
      bismillahPre: ffi.bismillah_pre,
    );
  }
}
```

### FFI Structure Definitions (Dart)
```dart
// AyahDataFFI
class AyahDataFFI extends ffi.Struct {
  external ffi.Pointer<Utf8> verse_key;
  external ffi.Pointer<Utf8> arabic_text;
  external ffi.Pointer<Utf8> translation;
  external ffi.Pointer<Utf8> transliteration;
  external ffi.Pointer<Utf8> surah_name;
  external ffi.Pointer<Utf8> surah_name_arabic;
  @ffi.Uint16()
  external int surah_number;
  @ffi.Uint16()
  external int ayah_number;
  @ffi.Uint8()
  external int juz_number;
  @ffi.Uint8()
  external int hizb_number;
  @ffi.Uint8()
  external int rub_number;
  @ffi.Uint8()
  external int ruku_number;
  @ffi.Uint8()
  external int manzil_number;
  @ffi.Uint16()
  external int page_number;
}

// SurahInfoFFI
class SurahInfoFFI extends ffi.Struct {
  @ffi.Uint16()
  external int id;
  external ffi.Pointer<Utf8> name_simple;
  external ffi.Pointer<Utf8> name_arabic;
  external ffi.Pointer<Utf8> name_english;
  @ffi.Uint16()
  external int revelation_order;
  external ffi.Pointer<Utf8> revelation_place;
  @ffi.Uint16()
  external int verses_count;
  @ffi.Bool()
  external bool bismillah_pre;
}

// SimilarAyahWithTextFFI
class SimilarAyahWithTextFFI extends ffi.Struct {
  external ffi.Pointer<Utf8> verse_key;
  external ffi.Pointer<Utf8> arabic_text;
  external ffi.Pointer<Utf8> translation;
  external ffi.Pointer<Utf8> transliteration;
  @ffi.Double()
  external double similarity_score;
  @ffi.Uint16()
  external int surah_number;
  @ffi.Uint16()
  external int ayah_number;
}

// QuranStatisticsFFI
class QuranStatisticsFFI extends ffi.Struct {
  @ffi.Uint16()
  external int total_surahs;
  @ffi.Uint32()
  external int total_ayahs;
  @ffi.Uint32()
  external int total_words;
  @ffi.Uint32()
  external int total_letters;
  @ffi.Uint16()
  external int total_pages;
  @ffi.Uint8()
  external int total_juz;
  @ffi.Uint8()
  external int total_hizb;
  @ffi.Uint16()
  external int total_ruku;
  @ffi.Uint8()
  external int total_manzil;
  @ffi.Uint8()
  external int total_sajda;
}

// AdvancedSearchCriteriaFFI
class AdvancedSearchCriteriaFFI extends ffi.Struct {
  external ffi.Pointer<Utf8> text_query;
  external ffi.Pointer<Utf8> translation_query;
  external ffi.Pointer<Utf8> transliteration_query;
  @ffi.Uint16()
  external int surah_number;
  @ffi.Uint8()
  external int juz_number;
  @ffi.Uint16()
  external int page_number;
  @ffi.Size()
  external int limit;
}
```

## Example Integration

Here's a complete example Flutter app using the Hafiz Assistant Backend:

```dart
// main.dart
import 'package:flutter/material.dart';
import 'hafiz_assistant_bindings.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Hafiz Assistant Demo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const MyHomePage(),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key});

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  late HafizAssistantBindings _bindings;
  bool _isInitialized = false;
  String _status = 'Not initialized';

  @override
  void initState() {
    super.initState();
    _initializeBindings();
  }

  void _initializeBindings() async {
    try {
      HafizAssistantBindings.initialize();
      _bindings = HafizAssistantBindings();
      
      final success = _bindings.initializeData();
      setState(() {
        _isInitialized = success;
        _status = success ? 'Initialized successfully' : 'Failed to initialize';
      });
    } catch (e) {
      setState(() {
        _status = 'Error: $e';
      });
    }
  }

  void _getRandomAyah() async {
    if (!_isInitialized) return;
    
    try {
      final ayah = _bindings.getRandomAyah();
      if (ayah != null) {
        showDialog(
          context: context,
          builder: (context) => AlertDialog(
            title: Text('Random Ayah (${ayah.verseKey})'),
            content: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text('Arabic:', style: TextStyle(fontWeight: FontWeight.bold)),
                Text(ayah.arabicText, style: TextStyle(fontSize: 18)),
                SizedBox(height: 10),
                Text('Translation:', style: TextStyle(fontWeight: FontWeight.bold)),
                Text(ayah.translation),
                SizedBox(height: 10),
                Text('Surah:', style: TextStyle(fontWeight: FontWeight.bold)),
                Text('${ayah.surahName} (${ayah.surahNumber})'),
              ],
            ),
            actions: [
              TextButton(
                onPressed: () => Navigator.of(context).pop(),
                child: Text('Close'),
              ),
            ],
          ),
        );
      }
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Error: $e')),
      );
    }
  }

  void _searchAyahs() async {
    if (!_isInitialized) return;
    
    final controller = TextEditingController();
    final result = await showDialog<String>(
      context: context,
      builder: (context) => AlertDialog(
        title: Text('Search Ayahs'),
        content: TextField(
          controller: controller,
          decoration: InputDecoration(
            hintText: 'Enter Arabic text to search',
            border: OutlineInputBorder(),
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: Text('Cancel'),
          ),
          TextButton(
            onPressed: () => Navigator.of(context).pop(controller.text),
            child: Text('Search'),
          ),
        ],
      ),
    );
    
    if (result != null && result.isNotEmpty) {
      try {
        final ayahs = _bindings.searchAyahsByText(result, 10);
        
        showDialog(
          context: context,
          builder: (context) => AlertDialog(
            title: Text('Search Results (${ayahs.length})'),
            content: Container(
              width: double.maxFinite,
              child: ListView.builder(
                shrinkWrap: true,
                itemCount: ayahs.length,
                itemBuilder: (context, index) {
                  final ayah = ayahs[index];
                  return Card(
                    child: ListTile(
                      title: Text(ayah.verseKey),
                      subtitle: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(ayah.arabicText, style: TextStyle(fontSize: 16)),
                          SizedBox(height: 5),
                          Text(ayah.translation),
                        ],
                      ),
                    ),
                  );
                },
              ),
            ),
            actions: [
              TextButton(
                onPressed: () => Navigator.of(context).pop(),
                child: Text('Close'),
              ),
            ],
          ),
        );
      } catch (e) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Error: $e')),
        );
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Hafiz Assistant Demo'),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            Text(
              'Status: $_status',
              style: TextStyle(fontSize: 16),
            ),
            SizedBox(height: 20),
            ElevatedButton(
              onPressed: _isInitialized ? _getRandomAyah : null,
              child: Text('Get Random Ayah'),
            ),
            SizedBox(height: 10),
            ElevatedButton(
              onPressed: _isInitialized ? _searchAyahs : null,
              child: Text('Search Ayahs'),
            ),
          ],
        ),
      ),
    );
  }
}
```

## Error Handling

The backend provides comprehensive error handling through return values and error codes:

```dart
// Check function results
final ayah = _bindings.getAyahData('2:255');
if (ayah == null) {
  print('Ayah not found or error occurred');
}

// Validate inputs before use
if (_bindings.validateVerseKey('2:255')) {
  final ayah = _bindings.getAyahData('2:255');
  // Process ayah
} else {
  print('Invalid verse key format');
}

// Handle search results
final results = _bindings.searchAyahsByText('الله', 10);
if (results.isEmpty) {
  print('No results found');
} else {
  print('Found ${results.length} results');
}
```

## Memory Management

**Critical**: Always ensure proper memory management to prevent memory leaks:

```dart
// The bindings automatically handle memory management
// by calling the appropriate free functions after
// converting FFI structures to Dart objects

// Example: getAyahData automatically calls free_ayah_data_ffi
final ayah = _bindings.getAyahData('2:255');
// Memory is automatically freed after conversion

// Example: searchAyahsByText automatically calls free_ayah_array_ffi
final results = _bindings.searchAyahsByText('الله', 10);
// Memory is automatically freed after conversion
```

## Best Practices

1. **Initialize Once**: Call `initializeData()` once at app startup
2. **Check Results**: Always check if functions return null/empty results
3. **Validate Inputs**: Use `validateVerseKey()` before calling ayah functions
4. **Handle Errors**: Implement proper error handling for all operations
5. **Memory Safety**: Let the bindings handle memory management automatically
6. **Performance**: Cache frequently used data in Dart objects
7. **Threading**: FFI calls are synchronous, use `compute()` for heavy operations

## Performance Considerations

- **Initialization**: First call to `initializeData()` may take a few seconds
- **Search Operations**: Text search is generally fast, similarity search may be slower
- **Memory Usage**: Each ayah structure uses approximately 1-2KB of memory
- **Caching**: Consider caching frequently accessed data at the Dart level

## Platform-Specific Notes

### Windows
- Use `hafiz_assistant_core.dll`
- Ensure DLL is in the same directory as the executable

### Linux
- Use `libhafiz_assistant_core.so`
- May need to set `LD_LIBRARY_PATH`

### macOS
- Use `libhafiz_assistant_core.dylib`
- May need to handle code signing requirements

## Complete Example Project

For a complete example project demonstrating all features, see:
- `examples/flutter_integration_complete.dart`
- Build scripts in `scripts/`
- Documentation in `docs/`

This integration provides full access to all Hafiz Assistant Backend features with type-safe, memory-managed Dart bindings.
