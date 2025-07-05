# 📱 Flutter Integration Guide - Hafiz Assistant Backend

## Overview
Library ini menyediakan **15 fungsi utama** yang dapat dipanggil dari Flutter melalui FFI (Foreign Function Interface). Semua fungsi sudah dioptimalkan untuk Android dan menggunakan memory management yang aman.

## 🔧 Setup Flutter FFI

### 1. Tambahkan Dependency
```yaml
# pubspec.yaml
dependencies:
  ffi: ^2.0.1
```

### 2. Copy Library Files
```bash
# Copy file .so ke Flutter project
cp jniLibs/* flutter_project/android/app/src/main/jniLibs/
```

### 3. Dart FFI Setup
```dart
import 'dart:ffi' as ffi;
import 'dart:io';
import 'package:ffi/ffi.dart';

// Load library
final DynamicLibrary _lib = Platform.isAndroid
    ? DynamicLibrary.open('libhafiz_assistant_engine.so')
    : DynamicLibrary.process();
```

## 📚 Available Functions

### 1. **Initialization**
```c
bool initialize_hafiz_assistant_engine()
```
**Dart Implementation:**
```dart
typedef InitializeFunc = ffi.Bool Function();
typedef InitializeFuncDart = bool Function();

final InitializeFuncDart initialize = _lib
    .lookup<ffi.NativeFunction<InitializeFunc>>('initialize_hafiz_assistant_engine')
    .asFunction();

// Usage
bool success = initialize();
```

### 2. **Get Single Ayah**
```c
AyahDataFFI* get_ayah_data_ffi(const char* verse_key)
```
**Dart Implementation:**
```dart
typedef GetAyahFunc = ffi.Pointer<AyahDataFFI> Function(ffi.Pointer<Utf8>);
typedef GetAyahFuncDart = ffi.Pointer<AyahDataFFI> Function(ffi.Pointer<Utf8>);

final GetAyahFuncDart getAyah = _lib
    .lookup<ffi.NativeFunction<GetAyahFunc>>('get_ayah_data_ffi')
    .asFunction();

// Usage
final verseKey = "1:1".toNativeUtf8();
final ayahPtr = getAyah(verseKey);
```

### 3. **Text Search**
```c
AyahDataFFI* search_ayahs_by_text_ffi(const char* query, usize limit, usize* result_count)
```
**Dart Implementation:**
```dart
typedef SearchTextFunc = ffi.Pointer<AyahDataFFI> Function(
    ffi.Pointer<Utf8>, ffi.Size, ffi.Pointer<ffi.Size>);
typedef SearchTextFuncDart = ffi.Pointer<AyahDataFFI> Function(
    ffi.Pointer<Utf8>, int, ffi.Pointer<ffi.Size>);

final SearchTextFuncDart searchText = _lib
    .lookup<ffi.NativeFunction<SearchTextFunc>>('search_ayahs_by_text_ffi')
    .asFunction();

// Usage
final query = "الله".toNativeUtf8();
final countPtr = malloc<ffi.Size>();
final resultsPtr = searchText(query, 10, countPtr);
final count = countPtr.value;
```

### 4. **Similar Ayahs Search**
```c
SimilarAyahWithTextFFI* search_similar_ayahs_ffi(const char* verse_key, usize limit, usize* result_count)
```

### 5. **Get Surah Info**
```c
SurahInfoFFI* get_surah_info_ffi(u16 surah_number)
```

### 6. **Get Ayahs by Surah**
```c
AyahDataFFI* get_ayahs_by_surah_ffi(u16 surah_number, usize* result_count)
```

### 7. **Get Ayahs by Juz**
```c
AyahDataFFI* get_ayahs_by_juz_ffi(u8 juz_number, usize* result_count)
```

### 8. **Get Ayahs by Page**
```c
AyahDataFFI* get_ayahs_by_page_ffi(u16 page_number, usize* result_count)
```

### 9. **Advanced Search**
```c
AyahDataFFI* advanced_search_ffi(const AdvancedSearchCriteriaFFI* criteria, usize* result_count)
```

### 10. **Get Quran Statistics**
```c
QuranStatisticsFFI* get_quran_statistics_ffi()
```

### 11. **Validate Verse Key**
```c
bool validate_verse_key_ffi(const char* verse_key)
```

### 12. **Get Random Ayah**
```c
AyahDataFFI* get_random_ayah_ffi()
```

### 13-15. **Memory Management**
```c
void free_ayah_data_ffi(AyahDataFFI* data)
void free_ayah_array_ffi(AyahDataFFI* data, usize count)
void free_surah_info_ffi(SurahInfoFFI* data)
```

## 🏗️ Data Structures

### AyahDataFFI
```dart
class AyahDataFFI extends ffi.Struct {
  external ffi.Pointer<Utf8> verse_key;        // "1:1"
  external ffi.Pointer<Utf8> arabic_text;      // "بِسْمِ اللَّهِ..."
  external ffi.Pointer<Utf8> translation;      // "Dengan nama Allah..."
  external ffi.Pointer<Utf8> transliteration;  // "Bismillahi..."
  external ffi.Pointer<Utf8> surah_name;       // "Al-Fatihah"
  external ffi.Pointer<Utf8> surah_name_arabic; // "الفاتحة"
  
  @ffi.Uint16()
  external int surah_number;    // 1-114
  
  @ffi.Uint16()
  external int ayah_number;     // 1-286
  
  @ffi.Uint8()
  external int juz_number;      // 1-30
  
  @ffi.Uint8()
  external int hizb_number;     // 1-60
  
  @ffi.Uint8()
  external int rub_number;      // 1-240
  
  @ffi.Uint8()
  external int ruku_number;     // 1-558
  
  @ffi.Uint8()
  external int manzil_number;   // 1-7
  
  @ffi.Uint16()
  external int page_number;     // 1-604
}
```

### SurahInfoFFI
```dart
class SurahInfoFFI extends ffi.Struct {
  @ffi.Uint16()
  external int id;              // 1-114
  
  external ffi.Pointer<Utf8> name_simple;    // "Al-Fatihah"
  external ffi.Pointer<Utf8> name_arabic;    // "الفاتحة"
  external ffi.Pointer<Utf8> name_english;   // "The Opening"
  
  @ffi.Uint16()
  external int revelation_order;             // 1-114
  
  external ffi.Pointer<Utf8> revelation_place; // "Mecca" / "Medina"
  
  @ffi.Uint16()
  external int verses_count;                 // 3-286
  
  @ffi.Bool()
  external bool bismillah_pre;              // true/false
}
```

### QuranStatisticsFFI
```dart
class QuranStatisticsFFI extends ffi.Struct {
  @ffi.Uint16()
  external int total_surahs;    // 114
  
  @ffi.Uint32()
  external int total_ayahs;     // 6236
  
  @ffi.Uint32()
  external int total_words;     // ~77,000
  
  @ffi.Uint32()
  external int total_letters;   // ~320,000
  
  @ffi.Uint16()
  external int total_pages;     // 604
  
  @ffi.Uint8()
  external int total_juz;       // 30
  
  @ffi.Uint8()
  external int total_hizb;      // 60
  
  @ffi.Uint16()
  external int total_ruku;      // 558
  
  @ffi.Uint8()
  external int total_manzil;    // 7
  
  @ffi.Uint8()
  external int total_sajda;     // 15
}
```

## 🔍 Usage Examples

### 1. Basic Ayah Retrieval
```dart
// Initialize
bool success = initialize();
if (!success) {
  print("Failed to initialize");
  return;
}

// Get single ayah
final verseKey = "1:1".toNativeUtf8();
final ayahPtr = getAyah(verseKey);

if (ayahPtr != ffi.nullptr) {
  final ayah = ayahPtr.ref;
  print("Arabic: ${ayah.arabic_text.toDartString()}");
  print("Translation: ${ayah.translation.toDartString()}");
  print("Surah: ${ayah.surah_name.toDartString()}");
  
  // Free memory
  freeAyah(ayahPtr);
}

malloc.free(verseKey);
```

### 2. Text Search
```dart
// Search for ayahs containing "الله"
final query = "الله".toNativeUtf8();
final countPtr = malloc<ffi.Size>();
final resultsPtr = searchText(query, 10, countPtr);
final count = countPtr.value;

if (resultsPtr != ffi.nullptr && count > 0) {
  for (int i = 0; i < count; i++) {
    final ayah = resultsPtr.elementAt(i).ref;
    print("${ayah.verse_key.toDartString()}: ${ayah.arabic_text.toDartString()}");
  }
  
  // Free memory
  freeAyahArray(resultsPtr, count);
}

malloc.free(query);
malloc.free(countPtr);
```

### 3. Get Surah Information
```dart
// Get Al-Fatihah info
final surahPtr = getSurahInfo(1);

if (surahPtr != ffi.nullptr) {
  final surah = surahPtr.ref;
  print("Name: ${surah.name_simple.toDartString()}");
  print("Arabic: ${surah.name_arabic.toDartString()}");
  print("Verses: ${surah.verses_count}");
  print("Revelation Place: ${surah.revelation_place.toDartString()}");
  
  // Free memory
  freeSurahInfo(surahPtr);
}
```

### 4. Get Ayahs by Surah
```dart
// Get all ayahs in Al-Fatihah
final countPtr = malloc<ffi.Size>();
final ayahsPtr = getAyahsBySurah(1, countPtr);
final count = countPtr.value;

if (ayahsPtr != ffi.nullptr && count > 0) {
  print("Al-Fatihah has $count ayahs:");
  for (int i = 0; i < count; i++) {
    final ayah = ayahsPtr.elementAt(i).ref;
    print("${i + 1}. ${ayah.arabic_text.toDartString()}");
  }
  
  // Free memory
  freeAyahArray(ayahsPtr, count);
}

malloc.free(countPtr);
```

### 5. Advanced Search
```dart
// Advanced search with multiple criteria
final criteria = malloc<AdvancedSearchCriteriaFFI>();
criteria.ref.text_query = "الله".toNativeUtf8();
criteria.ref.translation_query = ffi.nullptr;
criteria.ref.surah_number = 1; // Only Al-Fatihah
criteria.ref.juz_number = 0;   // No filter
criteria.ref.page_number = 0;  // No filter
criteria.ref.limit = 10;

final countPtr = malloc<ffi.Size>();
final resultsPtr = advancedSearch(criteria, countPtr);
final count = countPtr.value;

// Process results and free memory
if (resultsPtr != ffi.nullptr && count > 0) {
  // Process results...
  freeAyahArray(resultsPtr, count);
}

malloc.free(criteria.ref.text_query);
malloc.free(criteria);
malloc.free(countPtr);
```

### 6. Get Quran Statistics
```dart
final statsPtr = getQuranStatistics();

if (statsPtr != ffi.nullptr) {
  final stats = statsPtr.ref;
  print("Total Surahs: ${stats.total_surahs}");
  print("Total Ayahs: ${stats.total_ayahs}");
  print("Total Words: ${stats.total_words}");
  print("Total Pages: ${stats.total_pages}");
  print("Total Juz: ${stats.total_juz}");
  
  // Free memory
  freeQuranStatistics(statsPtr);
}
```

## ⚠️ Important Notes

### Memory Management
- **Always free memory** after use to prevent memory leaks
- Use specific free functions for each data type
- Check for null pointers before accessing data

### Error Handling
- Most functions return `null` on error
- Always check return values before use
- Initialize the library before calling other functions

### Performance Tips
- Initialize once at app startup
- Cache frequently used data
- Use appropriate limits for search functions
- Free memory as soon as possible

## 🎯 Complete Flutter Example

```dart
import 'dart:ffi' as ffi;
import 'dart:io';
import 'package:ffi/ffi.dart';

class HafizAssistantEngine {
  static final DynamicLibrary _lib = Platform.isAndroid
      ? DynamicLibrary.open('libhafiz_assistant_engine.so')
      : DynamicLibrary.process();

  // Function definitions
  static final InitializeFuncDart initialize = _lib
      .lookup<ffi.NativeFunction<InitializeFunc>>('initialize_hafiz_assistant_engine')
      .asFunction();

  static final GetAyahFuncDart getAyah = _lib
      .lookup<ffi.NativeFunction<GetAyahFunc>>('get_ayah_data_ffi')
      .asFunction();

  static final SearchTextFuncDart searchText = _lib
      .lookup<ffi.NativeFunction<SearchTextFunc>>('search_ayahs_by_text_ffi')
      .asFunction();

  // ... other function definitions

  static bool initializeEngine() {
    return initialize();
  }

  static AyahData? getAyahData(String verseKey) {
    final key = verseKey.toNativeUtf8();
    final ayahPtr = getAyah(key);
    
    if (ayahPtr == ffi.nullptr) {
      malloc.free(key);
      return null;
    }
    
    final ayah = ayahPtr.ref;
    final result = AyahData.fromFFI(ayah);
    
    // Free memory
    freeAyah(ayahPtr);
    malloc.free(key);
    
    return result;
  }

  static List<AyahData> searchAyahs(String query, int limit) {
    final queryPtr = query.toNativeUtf8();
    final countPtr = malloc<ffi.Size>();
    final resultsPtr = searchText(queryPtr, limit, countPtr);
    final count = countPtr.value;
    
    List<AyahData> results = [];
    
    if (resultsPtr != ffi.nullptr && count > 0) {
      for (int i = 0; i < count; i++) {
        final ayah = resultsPtr.elementAt(i).ref;
        results.add(AyahData.fromFFI(ayah));
      }
      
      freeAyahArray(resultsPtr, count);
    }
    
    malloc.free(queryPtr);
    malloc.free(countPtr);
    
    return results;
  }
}

// Usage in Flutter app
void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: QuranScreen(),
    );
  }
}

class QuranScreen extends StatefulWidget {
  @override
  _QuranScreenState createState() => _QuranScreenState();
}

class _QuranScreenState extends State<QuranScreen> {
  List<AyahData> searchResults = [];
  bool isInitialized = false;

  @override
  void initState() {
    super.initState();
    initializeEngine();
  }

  void initializeEngine() {
    bool success = HafizAssistantEngine.initializeEngine();
    setState(() {
      isInitialized = success;
    });
  }

  void searchAyahs(String query) {
    if (!isInitialized) return;
    
    List<AyahData> results = HafizAssistantEngine.searchAyahs(query, 10);
    setState(() {
      searchResults = results;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('Hafiz Assistant')),
      body: Column(
        children: [
          if (!isInitialized)
            Center(child: CircularProgressIndicator()),
          if (isInitialized)
            Padding(
              padding: EdgeInsets.all(16),
              child: TextField(
                onSubmitted: searchAyahs,
                decoration: InputDecoration(
                  hintText: 'Search Quran...',
                  border: OutlineInputBorder(),
                ),
              ),
            ),
          Expanded(
            child: ListView.builder(
              itemCount: searchResults.length,
              itemBuilder: (context, index) {
                final ayah = searchResults[index];
                return Card(
                  child: ListTile(
                    title: Text(ayah.arabicText),
                    subtitle: Text(ayah.translation),
                    trailing: Text(ayah.verseKey),
                  ),
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

Dengan dokumentasi ini, Anda dapat mengintegrasikan semua 15 fungsi library Hafiz Assistant ke dalam aplikasi Flutter Anda! 🚀
