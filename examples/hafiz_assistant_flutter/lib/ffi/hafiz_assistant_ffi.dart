import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';

// FFI type definitions matching Rust structs
base class AyahDataNative extends Struct {
  external Pointer<Utf8> verseKey;
  external Pointer<Utf8> arabicText;
  external Pointer<Utf8> translation;
  external Pointer<Utf8> transliteration;
  external Pointer<Utf8> surahName;
  external Pointer<Utf8> surahNameArabic;
  
  @Uint16()
  external int surahNumber;
  
  @Uint16()
  external int ayahNumber;
  
  @Uint8()
  external int juzNumber;
  
  @Uint8()
  external int hizbNumber;
  
  @Uint8()
  external int rubNumber;
  
  @Uint8()
  external int rukuNumber;
  
  @Uint8()
  external int manzilNumber;
  
  @Uint16()
  external int pageNumber;
}

base class SurahInfoNative extends Struct {
  @Uint16()
  external int id;
  
  external Pointer<Utf8> nameSimple;
  external Pointer<Utf8> nameArabic;
  external Pointer<Utf8> nameEnglish;
  
  @Uint16()
  external int revelationOrder;
  
  external Pointer<Utf8> revelationPlace;
  
  @Uint16()
  external int versesCount;
  
  @Bool()
  external bool bismillahPre;
}

base class QuranStatisticsNative extends Struct {
  @Uint16()
  external int totalSurahs;
  
  @Uint32()
  external int totalAyahs;
  
  @Uint32()
  external int totalWords;
  
  @Uint32()
  external int totalLetters;
  
  @Uint16()
  external int totalPages;
  
  @Uint8()
  external int totalJuz;
  
  @Uint8()
  external int totalHizb;
  
  @Uint16()
  external int totalRuku;
  
  @Uint8()
  external int totalManzil;
  
  @Uint8()
  external int totalSajda;
}

// Function signatures for FFI calls
typedef InitializeNative = Int32 Function();
typedef Initialize = int Function();

typedef GetRandomAyahNative = Pointer<AyahDataNative> Function();
typedef GetRandomAyah = Pointer<AyahDataNative> Function();

typedef GetAyahNative = Pointer<AyahDataNative> Function(Pointer<Utf8> verseKey);
typedef GetAyah = Pointer<AyahDataNative> Function(Pointer<Utf8> verseKey);

typedef GetSurahInfoNative = Pointer<SurahInfoNative> Function(Uint16 surahNumber);
typedef GetSurahInfo = Pointer<SurahInfoNative> Function(int surahNumber);

typedef GetQuranStatisticsNative = Pointer<QuranStatisticsNative> Function();
typedef GetQuranStatistics = Pointer<QuranStatisticsNative> Function();

typedef SearchAyahsNative = Pointer<AyahDataNative> Function(Pointer<Utf8> query, Uint64 limit, Pointer<Uint64> resultCount);
typedef SearchAyahs = Pointer<AyahDataNative> Function(Pointer<Utf8> query, int limit, Pointer<Uint64> resultCount);

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
      _initialize = _library.lookupFunction<InitializeNative, Initialize>('initialize_data_ffi');
      _getRandomAyah = _library.lookupFunction<GetRandomAyahNative, GetRandomAyah>('get_random_ayah_ffi');
      _getAyah = _library.lookupFunction<GetAyahNative, GetAyah>('get_ayah_data_ffi');
      _getSurahInfo = _library.lookupFunction<GetSurahInfoNative, GetSurahInfo>('get_surah_info_ffi');
      _getQuranStatistics = _library.lookupFunction<GetQuranStatisticsNative, GetQuranStatistics>('get_quran_statistics_ffi');
      _searchAyahs = _library.lookupFunction<SearchAyahsNative, SearchAyahs>('search_ayahs_by_text_ffi');
      _freeAyahData = _library.lookupFunction<FreeAyahDataNative, FreeAyahData>('free_ayah_data_ffi');
      _freeSurahInfo = _library.lookupFunction<FreeSurahInfoNative, FreeSurahInfo>('free_surah_info_ffi');
      _freeStatistics = _library.lookupFunction<FreeStatisticsNative, FreeStatistics>('free_quran_statistics_ffi');
      _freeSearchResults = _library.lookupFunction<FreeSearchResultsNative, FreeSearchResults>('free_ayah_array_ffi');

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

  // Updated to use verse key string instead of separate surah/ayah numbers
  Pointer<AyahDataNative>? getAyahByVerseKey(String verseKey) {
    if (!_isInitialized) return null;
    try {
      final verseKeyPtr = verseKey.toNativeUtf8();
      final result = _getAyah(verseKeyPtr);
      malloc.free(verseKeyPtr);
      return result;
    } catch (e) {
      print('Failed to get ayah: $e');
      return null;
    }
  }

  // Helper method for backward compatibility
  Pointer<AyahDataNative>? getAyah(int surahNumber, int ayahNumber) {
    return getAyahByVerseKey('$surahNumber:$ayahNumber');
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

  List<Pointer<AyahDataNative>> searchAyahs(String query, {int limit = 10}) {
    if (!_isInitialized) return [];
    
    try {
      final queryPtr = query.toNativeUtf8();
      final countPtr = malloc<Uint64>();
      
      final resultsPtr = _searchAyahs(queryPtr, limit, countPtr);
      final count = countPtr.value;
      
      List<Pointer<AyahDataNative>> results = [];
      if (resultsPtr != nullptr && count > 0) {
        for (int i = 0; i < count; i++) {
          final ayahPtr = resultsPtr.elementAt(i);
          if (ayahPtr != nullptr) {
            results.add(ayahPtr);
          }
        }
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
    if (_isInitialized && ayah != nullptr) {
      try {
        _freeAyahData(ayah);
      } catch (e) {
        print('Failed to free ayah data: $e');
      }
    }
  }

  void freeSurahInfo(Pointer<SurahInfoNative> surah) {
    if (_isInitialized && surah != nullptr) {
      try {
        _freeSurahInfo(surah);
      } catch (e) {
        print('Failed to free surah info: $e');
      }
    }
  }

  void freeStatistics(Pointer<QuranStatisticsNative> stats) {
    if (_isInitialized && stats != nullptr) {
      try {
        _freeStatistics(stats);
      } catch (e) {
        print('Failed to free statistics: $e');
      }
    }
  }

  void freeSearchResults(List<Pointer<AyahDataNative>> results) {
    if (_isInitialized && results.isNotEmpty) {
      try {
        // Free individual ayah data structures
        for (final ayahPtr in results) {
          if (ayahPtr != nullptr) {
            freeAyahData(ayahPtr);
          }
        }
      } catch (e) {
        print('Failed to free search results: $e');
      }
    }
  }

  bool get isInitialized => _isInitialized;
}
