import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';

// FFI type definitions matching Rust structs
base class AyahDataNative extends Struct {
  @Int32()
  external int surahNumber;
  
  @Int32()
  external int ayahNumber;
  
  external Pointer<Utf8> arabicText;
  external Pointer<Utf8> transliteration;
  external Pointer<Utf8> translation;
  external Pointer<Utf8> surahNameArabic;
  external Pointer<Utf8> surahNameTransliteration;
}

base class SurahInfoNative extends Struct {
  @Int32()
  external int number;
  
  external Pointer<Utf8> nameArabic;
  external Pointer<Utf8> nameTransliteration;
  external Pointer<Utf8> nameTranslation;
  
  @Int32()
  external int ayahCount;
  
  @Int32()
  external int revelationType; // 0 = Meccan, 1 = Medinan
}

base class QuranStatisticsNative extends Struct {
  @Int32()
  external int totalSurahs;
  
  @Int32()
  external int totalAyahs;
  
  @Int32()
  external int meccanSurahs;
  
  @Int32()
  external int medinanSurahs;
}

// Function signatures for FFI calls
typedef InitializeNative = Int32 Function();
typedef Initialize = int Function();

typedef GetRandomAyahNative = Pointer<AyahDataNative> Function();
typedef GetRandomAyah = Pointer<AyahDataNative> Function();

typedef GetAyahNative = Pointer<AyahDataNative> Function(Int32 surahNumber, Int32 ayahNumber);
typedef GetAyah = Pointer<AyahDataNative> Function(int surahNumber, int ayahNumber);

typedef GetSurahInfoNative = Pointer<SurahInfoNative> Function(Int32 surahNumber);
typedef GetSurahInfo = Pointer<SurahInfoNative> Function(int surahNumber);

typedef GetQuranStatisticsNative = Pointer<QuranStatisticsNative> Function();
typedef GetQuranStatistics = Pointer<QuranStatisticsNative> Function();

typedef SearchAyahsNative = Pointer<Pointer<AyahDataNative>> Function(Pointer<Utf8> query, Pointer<Int32> resultCount);
typedef SearchAyahs = Pointer<Pointer<AyahDataNative>> Function(Pointer<Utf8> query, Pointer<Int32> resultCount);

typedef FreeAyahDataNative = Void Function(Pointer<AyahDataNative> ayah);
typedef FreeAyahData = void Function(Pointer<AyahDataNative> ayah);

typedef FreeSurahInfoNative = Void Function(Pointer<SurahInfoNative> surah);
typedef FreeSurahInfo = void Function(Pointer<SurahInfoNative> surah);

typedef FreeStatisticsNative = Void Function(Pointer<QuranStatisticsNative> stats);
typedef FreeStatistics = void Function(Pointer<QuranStatisticsNative> stats);

typedef FreeSearchResultsNative = Void Function(Pointer<Pointer<AyahDataNative>> results, Int32 count);
typedef FreeSearchResults = void Function(Pointer<Pointer<AyahDataNative>> results, int count);

class HafizAssistantFFI {
  static HafizAssistantFFI? _instance;
  late DynamicLibrary _library;
  
  // Function pointers
  late Initialize _initialize;
  late GetRandomAyah _getRandomAyah;
  late GetAyah _getAyah;
  late GetSurahInfo _getSurahInfo;
  late GetQuranStatistics _getQuranStatistics;
  late SearchAyahs _searchAyahs;
  late FreeAyahData _freeAyahData;
  late FreeSurahInfo _freeSurahInfo;
  late FreeStatistics _freeStatistics;
  late FreeSearchResults _freeSearchResults;

  HafizAssistantFFI._();

  static HafizAssistantFFI get instance {
    _instance ??= HafizAssistantFFI._();
    return _instance!;
  }

  bool _isInitialized = false;

  bool initialize() {
    if (_isInitialized) return true;

    try {
      // Load the dynamic library
      if (Platform.isAndroid) {
        _library = DynamicLibrary.open('libhafiz_assistant_core.so');
      } else if (Platform.isWindows) {
        _library = DynamicLibrary.open('hafiz_assistant_core.dll');
      } else if (Platform.isLinux) {
        _library = DynamicLibrary.open('libhafiz_assistant_core.so');
      } else {
        throw UnsupportedError('Platform not supported');
      }

      // Load function pointers
      _initialize = _library.lookupFunction<InitializeNative, Initialize>('initialize_hafiz_assistant');
      _getRandomAyah = _library.lookupFunction<GetRandomAyahNative, GetRandomAyah>('get_random_ayah');
      _getAyah = _library.lookupFunction<GetAyahNative, GetAyah>('get_ayah');
      _getSurahInfo = _library.lookupFunction<GetSurahInfoNative, GetSurahInfo>('get_surah_info');
      _getQuranStatistics = _library.lookupFunction<GetQuranStatisticsNative, GetQuranStatistics>('get_quran_statistics');
      _searchAyahs = _library.lookupFunction<SearchAyahsNative, SearchAyahs>('search_ayahs_by_text');
      _freeAyahData = _library.lookupFunction<FreeAyahDataNative, FreeAyahData>('free_ayah_data');
      _freeSurahInfo = _library.lookupFunction<FreeSurahInfoNative, FreeSurahInfo>('free_surah_info');
      _freeStatistics = _library.lookupFunction<FreeStatisticsNative, FreeStatistics>('free_statistics');
      _freeSearchResults = _library.lookupFunction<FreeSearchResultsNative, FreeSearchResults>('free_search_results');

      // Initialize the Rust library
      int result = _initialize();
      _isInitialized = result == 0;
      
      return _isInitialized;
    } catch (e) {
      print('Failed to initialize FFI: $e');
      return false;
    }
  }

  Pointer<AyahDataNative>? getRandomAyah() {
    if (!_isInitialized) return null;
    try {
      return _getRandomAyah();
    } catch (e) {
      print('Failed to get random ayah: $e');
      return null;
    }
  }

  Pointer<AyahDataNative>? getAyah(int surahNumber, int ayahNumber) {
    if (!_isInitialized) return null;
    try {
      return _getAyah(surahNumber, ayahNumber);
    } catch (e) {
      print('Failed to get ayah: $e');
      return null;
    }
  }

  Pointer<SurahInfoNative>? getSurahInfo(int surahNumber) {
    if (!_isInitialized) return null;
    try {
      return _getSurahInfo(surahNumber);
    } catch (e) {
      print('Failed to get surah info: $e');
      return null;
    }
  }

  Pointer<QuranStatisticsNative>? getQuranStatistics() {
    if (!_isInitialized) return null;
    try {
      return _getQuranStatistics();
    } catch (e) {
      print('Failed to get statistics: $e');
      return null;
    }
  }

  List<Pointer<AyahDataNative>> searchAyahs(String query) {
    if (!_isInitialized) return [];
    
    try {
      final queryPtr = query.toNativeUtf8();
      final countPtr = malloc<Int32>();
      
      final resultsPtr = _searchAyahs(queryPtr, countPtr);
      final count = countPtr.value;
      
      List<Pointer<AyahDataNative>> results = [];
      for (int i = 0; i < count; i++) {
        results.add(resultsPtr.elementAt(i).value);
      }
      
      malloc.free(queryPtr);
      malloc.free(countPtr);
      
      return results;
    } catch (e) {
      print('Failed to search ayahs: $e');
      return [];
    }
  }

  void freeAyahData(Pointer<AyahDataNative> ayah) {
    if (_isInitialized) {
      _freeAyahData(ayah);
    }
  }

  void freeSurahInfo(Pointer<SurahInfoNative> surah) {
    if (_isInitialized) {
      _freeSurahInfo(surah);
    }
  }

  void freeStatistics(Pointer<QuranStatisticsNative> stats) {
    if (_isInitialized) {
      _freeStatistics(stats);
    }
  }

  void freeSearchResults(List<Pointer<AyahDataNative>> results) {
    if (_isInitialized && results.isNotEmpty) {
      // Convert list back to pointer array for freeing
      final arrayPtr = malloc<Pointer<AyahDataNative>>(results.length);
      for (int i = 0; i < results.length; i++) {
        arrayPtr[i] = results[i];
      }
      _freeSearchResults(arrayPtr, results.length);
      malloc.free(arrayPtr);
    }
  }
}
