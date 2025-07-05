// Complete Flutter Integration Example for Hafiz Assistant Backend
// This file demonstrates how to integrate the Hafiz Assistant Backend with Flutter

import 'dart:ffi' as ffi;
import 'dart:io' show Platform;
import 'package:ffi/ffi.dart';
import 'package:flutter/material.dart';

// FFI Structure Definitions
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

// Dart Data Models
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

class SimilarAyahWithText {
  final String verseKey;
  final String arabicText;
  final String translation;
  final String transliteration;
  final double similarityScore;
  final int surahNumber;
  final int ayahNumber;
  
  SimilarAyahWithText({
    required this.verseKey,
    required this.arabicText,
    required this.translation,
    required this.transliteration,
    required this.similarityScore,
    required this.surahNumber,
    required this.ayahNumber,
  });
  
  factory SimilarAyahWithText.fromFFI(SimilarAyahWithTextFFI ffi) {
    return SimilarAyahWithText(
      verseKey: ffi.verse_key.toDartString(),
      arabicText: ffi.arabic_text.toDartString(),
      translation: ffi.translation.toDartString(),
      transliteration: ffi.transliteration.toDartString(),
      similarityScore: ffi.similarity_score,
      surahNumber: ffi.surah_number,
      ayahNumber: ffi.ayah_number,
    );
  }
}

class QuranStatistics {
  final int totalSurahs;
  final int totalAyahs;
  final int totalWords;
  final int totalLetters;
  final int totalPages;
  final int totalJuz;
  final int totalHizb;
  final int totalRuku;
  final int totalManzil;
  final int totalSajda;
  
  QuranStatistics({
    required this.totalSurahs,
    required this.totalAyahs,
    required this.totalWords,
    required this.totalLetters,
    required this.totalPages,
    required this.totalJuz,
    required this.totalHizb,
    required this.totalRuku,
    required this.totalManzil,
    required this.totalSajda,
  });
  
  factory QuranStatistics.fromFFI(QuranStatisticsFFI ffi) {
    return QuranStatistics(
      totalSurahs: ffi.total_surahs,
      totalAyahs: ffi.total_ayahs,
      totalWords: ffi.total_words,
      totalLetters: ffi.total_letters,
      totalPages: ffi.total_pages,
      totalJuz: ffi.total_juz,
      totalHizb: ffi.total_hizb,
      totalRuku: ffi.total_ruku,
      totalManzil: ffi.total_manzil,
      totalSajda: ffi.total_sajda,
    );
  }
}

class AdvancedSearchCriteria {
  final String? textQuery;
  final String? translationQuery;
  final String? transliterationQuery;
  final int? surahNumber;
  final int? juzNumber;
  final int? pageNumber;
  final int limit;
  
  AdvancedSearchCriteria({
    this.textQuery,
    this.translationQuery,
    this.transliterationQuery,
    this.surahNumber,
    this.juzNumber,
    this.pageNumber,
    this.limit = 10,
  });
  
  ffi.Pointer<AdvancedSearchCriteriaFFI> toFFI() {
    final criteria = calloc<AdvancedSearchCriteriaFFI>();
    
    criteria.ref.text_query = textQuery?.toNativeUtf8() ?? ffi.nullptr;
    criteria.ref.translation_query = translationQuery?.toNativeUtf8() ?? ffi.nullptr;
    criteria.ref.transliteration_query = transliterationQuery?.toNativeUtf8() ?? ffi.nullptr;
    criteria.ref.surah_number = surahNumber ?? 0;
    criteria.ref.juz_number = juzNumber ?? 0;
    criteria.ref.page_number = pageNumber ?? 0;
    criteria.ref.limit = limit;
    
    return criteria;
  }
  
  void freeFFI(ffi.Pointer<AdvancedSearchCriteriaFFI> criteria) {
    if (criteria.ref.text_query != ffi.nullptr) {
      calloc.free(criteria.ref.text_query);
    }
    if (criteria.ref.translation_query != ffi.nullptr) {
      calloc.free(criteria.ref.translation_query);
    }
    if (criteria.ref.transliteration_query != ffi.nullptr) {
      calloc.free(criteria.ref.transliteration_query);
    }
    calloc.free(criteria);
  }
}

// FFI Bindings Class
class HafizAssistantBindings {
  static late ffi.DynamicLibrary _dylib;
  static bool _initialized = false;
  
  // Function bindings
  late final _initializeData = _dylib.lookupFunction<
      ffi.Bool Function(),
      bool Function()
  >('initialize_data_ffi');
  
  late final _getAyahData = _dylib.lookupFunction<
      ffi.Pointer<AyahDataFFI> Function(ffi.Pointer<Utf8>),
      ffi.Pointer<AyahDataFFI> Function(ffi.Pointer<Utf8>)
  >('get_ayah_data_ffi');
  
  late final _freeAyahData = _dylib.lookupFunction<
      ffi.Void Function(ffi.Pointer<AyahDataFFI>),
      void Function(ffi.Pointer<AyahDataFFI>)
  >('free_ayah_data_ffi');
  
  late final _searchAyahsByText = _dylib.lookupFunction<
      ffi.Pointer<AyahDataFFI> Function(ffi.Pointer<Utf8>, ffi.Size, ffi.Pointer<ffi.Size>),
      ffi.Pointer<AyahDataFFI> Function(ffi.Pointer<Utf8>, int, ffi.Pointer<ffi.Size>)
  >('search_ayahs_by_text_ffi');
  
  late final _searchSimilarAyahs = _dylib.lookupFunction<
      ffi.Pointer<SimilarAyahWithTextFFI> Function(ffi.Pointer<Utf8>, ffi.Size, ffi.Pointer<ffi.Size>),
      ffi.Pointer<SimilarAyahWithTextFFI> Function(ffi.Pointer<Utf8>, int, ffi.Pointer<ffi.Size>)
  >('search_similar_ayahs_ffi');
  
  late final _freeSimilarAyahs = _dylib.lookupFunction<
      ffi.Void Function(ffi.Pointer<SimilarAyahWithTextFFI>, ffi.Size),
      void Function(ffi.Pointer<SimilarAyahWithTextFFI>, int)
  >('free_similar_ayahs_ffi');
  
  late final _getSurahInfo = _dylib.lookupFunction<
      ffi.Pointer<SurahInfoFFI> Function(ffi.Uint16),
      ffi.Pointer<SurahInfoFFI> Function(int)
  >('get_surah_info_ffi');
  
  late final _freeSurahInfo = _dylib.lookupFunction<
      ffi.Void Function(ffi.Pointer<SurahInfoFFI>),
      void Function(ffi.Pointer<SurahInfoFFI>)
  >('free_surah_info_ffi');
  
  late final _getAyahsBySurah = _dylib.lookupFunction<
      ffi.Pointer<AyahDataFFI> Function(ffi.Uint16, ffi.Pointer<ffi.Size>),
      ffi.Pointer<AyahDataFFI> Function(int, ffi.Pointer<ffi.Size>)
  >('get_ayahs_by_surah_ffi');
  
  late final _getAyahsByJuz = _dylib.lookupFunction<
      ffi.Pointer<AyahDataFFI> Function(ffi.Uint8, ffi.Pointer<ffi.Size>),
      ffi.Pointer<AyahDataFFI> Function(int, ffi.Pointer<ffi.Size>)
  >('get_ayahs_by_juz_ffi');
  
  late final _getAyahsByPage = _dylib.lookupFunction<
      ffi.Pointer<AyahDataFFI> Function(ffi.Uint16, ffi.Pointer<ffi.Size>),
      ffi.Pointer<AyahDataFFI> Function(int, ffi.Pointer<ffi.Size>)
  >('get_ayahs_by_page_ffi');
  
  late final _freeAyahArray = _dylib.lookupFunction<
      ffi.Void Function(ffi.Pointer<AyahDataFFI>, ffi.Size),
      void Function(ffi.Pointer<AyahDataFFI>, int)
  >('free_ayah_array_ffi');
  
  late final _advancedSearch = _dylib.lookupFunction<
      ffi.Pointer<AyahDataFFI> Function(ffi.Pointer<AdvancedSearchCriteriaFFI>, ffi.Pointer<ffi.Size>),
      ffi.Pointer<AyahDataFFI> Function(ffi.Pointer<AdvancedSearchCriteriaFFI>, ffi.Pointer<ffi.Size>)
  >('advanced_search_ffi');
  
  late final _getQuranStatistics = _dylib.lookupFunction<
      ffi.Pointer<QuranStatisticsFFI> Function(),
      ffi.Pointer<QuranStatisticsFFI> Function()
  >('get_quran_statistics_ffi');
  
  late final _freeQuranStatistics = _dylib.lookupFunction<
      ffi.Void Function(ffi.Pointer<QuranStatisticsFFI>),
      void Function(ffi.Pointer<QuranStatisticsFFI>)
  >('free_quran_statistics_ffi');
  
  late final _getRandomAyah = _dylib.lookupFunction<
      ffi.Pointer<AyahDataFFI> Function(),
      ffi.Pointer<AyahDataFFI> Function()
  >('get_random_ayah_ffi');
  
  late final _validateVerseKey = _dylib.lookupFunction<
      ffi.Bool Function(ffi.Pointer<Utf8>),
      bool Function(ffi.Pointer<Utf8>)
  >('validate_verse_key_ffi');
  
  // Initialize the library
  static void initialize() {
    if (_initialized) return;
    
    if (Platform.isWindows) {
      _dylib = ffi.DynamicLibrary.open('hafiz_assistant_core.dll');
    } else if (Platform.isLinux) {
      _dylib = ffi.DynamicLibrary.open('libhafiz_assistant_core.so');
    } else if (Platform.isMacOS) {
      _dylib = ffi.DynamicLibrary.open('libhafiz_assistant_core.dylib');
    } else {
      throw UnsupportedError('Unsupported platform');
    }
    
    _initialized = true;
  }
  
  // Initialize Quran data
  bool initializeData() {
    return _initializeData();
  }
  
  // Get ayah data by verse key
  AyahData? getAyahData(String verseKey) {
    final verseKeyPtr = verseKey.toNativeUtf8();
    final result = _getAyahData(verseKeyPtr);
    calloc.free(verseKeyPtr);
    
    if (result.address == 0) return null;
    
    final ayahData = AyahData.fromFFI(result.ref);
    _freeAyahData(result);
    return ayahData;
  }
  
  // Search ayahs by text
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
  
  // Search similar ayahs
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
  
  // Get surah info
  SurahInfo? getSurahInfo(int surahNumber) {
    final result = _getSurahInfo(surahNumber);
    
    if (result.address == 0) return null;
    
    final surahInfo = SurahInfo.fromFFI(result.ref);
    _freeSurahInfo(result);
    return surahInfo;
  }
  
  // Get ayahs by surah
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
  
  // Get ayahs by juz
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
  
  // Get ayahs by page
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
  
  // Advanced search
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
  
  // Get Quran statistics
  QuranStatistics? getQuranStatistics() {
    final result = _getQuranStatistics();
    
    if (result.address == 0) return null;
    
    final stats = QuranStatistics.fromFFI(result.ref);
    _freeQuranStatistics(result);
    return stats;
  }
  
  // Get random ayah
  AyahData? getRandomAyah() {
    final result = _getRandomAyah();
    
    if (result.address == 0) return null;
    
    final ayah = AyahData.fromFFI(result.ref);
    _freeAyahData(result);
    return ayah;
  }
  
  // Validate verse key
  bool validateVerseKey(String verseKey) {
    final verseKeyPtr = verseKey.toNativeUtf8();
    final result = _validateVerseKey(verseKeyPtr);
    calloc.free(verseKeyPtr);
    return result;
  }
}

// Flutter App Example
class HafizAssistantApp extends StatefulWidget {
  const HafizAssistantApp({super.key});

  @override
  State<HafizAssistantApp> createState() => _HafizAssistantAppState();
}

class _HafizAssistantAppState extends State<HafizAssistantApp> {
  late HafizAssistantBindings _bindings;
  bool _isInitialized = false;
  bool _isLoading = false;
  String _status = 'Not initialized';
  
  @override
  void initState() {
    super.initState();
    _initializeBindings();
  }
  
  void _initializeBindings() async {
    setState(() {
      _isLoading = true;
      _status = 'Initializing...';
    });
    
    try {
      HafizAssistantBindings.initialize();
      _bindings = HafizAssistantBindings();
      
      final success = _bindings.initializeData();
      setState(() {
        _isInitialized = success;
        _isLoading = false;
        _status = success ? 'Initialized successfully' : 'Failed to initialize';
      });
    } catch (e) {
      setState(() {
        _isLoading = false;
        _status = 'Error: $e';
      });
    }
  }
  
  void _showAyahDetails(AyahData ayah) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: Text('Ayah ${ayah.verseKey}'),
        content: SingleChildScrollView(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              _buildInfoRow('Arabic:', ayah.arabicText, fontSize: 18),
              const SizedBox(height: 10),
              _buildInfoRow('Translation:', ayah.translation),
              const SizedBox(height: 10),
              _buildInfoRow('Transliteration:', ayah.transliteration),
              const SizedBox(height: 10),
              _buildInfoRow('Surah:', '${ayah.surahName} (${ayah.surahNumber})'),
              const SizedBox(height: 5),
              _buildInfoRow('Juz:', '${ayah.juzNumber}'),
              const SizedBox(height: 5),
              _buildInfoRow('Page:', '${ayah.pageNumber}'),
            ],
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Close'),
          ),
        ],
      ),
    );
  }
  
  Widget _buildInfoRow(String label, String value, {double fontSize = 14}) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          label,
          style: const TextStyle(fontWeight: FontWeight.bold),
        ),
        const SizedBox(height: 2),
        Text(
          value,
          style: TextStyle(fontSize: fontSize),
        ),
      ],
    );
  }
  
  void _getRandomAyah() async {
    if (!_isInitialized) return;
    
    try {
      final ayah = _bindings.getRandomAyah();
      if (ayah != null) {
        _showAyahDetails(ayah);
      } else {
        _showErrorMessage('Failed to get random ayah');
      }
    } catch (e) {
      _showErrorMessage('Error: $e');
    }
  }
  
  void _searchAyahs() async {
    if (!_isInitialized) return;
    
    final controller = TextEditingController();
    final result = await showDialog<String>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Search Ayahs'),
        content: TextField(
          controller: controller,
          decoration: const InputDecoration(
            hintText: 'Enter Arabic text to search',
            border: OutlineInputBorder(),
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () => Navigator.of(context).pop(controller.text),
            child: const Text('Search'),
          ),
        ],
      ),
    );
    
    if (result != null && result.isNotEmpty) {
      _performSearch(result);
    }
  }
  
  void _performSearch(String query) async {
    try {
      final ayahs = _bindings.searchAyahsByText(query, 10);
      
      if (ayahs.isEmpty) {
        _showErrorMessage('No results found for "$query"');
        return;
      }
      
      showDialog(
        context: context,
        builder: (context) => AlertDialog(
          title: Text('Search Results (${ayahs.length})'),
          content: SizedBox(
            width: double.maxFinite,
            height: 400,
            child: ListView.builder(
              itemCount: ayahs.length,
              itemBuilder: (context, index) {
                final ayah = ayahs[index];
                return Card(
                  child: ListTile(
                    title: Text(ayah.verseKey),
                    subtitle: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          ayah.arabicText,
                          style: const TextStyle(fontSize: 16),
                        ),
                        const SizedBox(height: 5),
                        Text(
                          ayah.translation,
                          style: const TextStyle(color: Colors.grey),
                        ),
                      ],
                    ),
                    onTap: () {
                      Navigator.of(context).pop();
                      _showAyahDetails(ayah);
                    },
                  ),
                );
              },
            ),
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.of(context).pop(),
              child: const Text('Close'),
            ),
          ],
        ),
      );
    } catch (e) {
      _showErrorMessage('Error: $e');
    }
  }
  
  void _showSurahList() async {
    if (!_isInitialized) return;
    
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Select Surah'),
        content: SizedBox(
          width: double.maxFinite,
          height: 400,
          child: ListView.builder(
            itemCount: 114,
            itemBuilder: (context, index) {
              final surahNumber = index + 1;
              return ListTile(
                title: Text('Surah $surahNumber'),
                onTap: () {
                  Navigator.of(context).pop();
                  _showSurahDetails(surahNumber);
                },
              );
            },
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Close'),
          ),
        ],
      ),
    );
  }
  
  void _showSurahDetails(int surahNumber) async {
    try {
      final surahInfo = _bindings.getSurahInfo(surahNumber);
      if (surahInfo == null) {
        _showErrorMessage('Surah not found');
        return;
      }
      
      final ayahs = _bindings.getAyahsBySurah(surahNumber);
      
      showDialog(
        context: context,
        builder: (context) => AlertDialog(
          title: Text('${surahInfo.nameEnglish} (${surahInfo.nameArabic})'),
          content: SizedBox(
            width: double.maxFinite,
            height: 400,
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                _buildInfoRow('Verses:', '${surahInfo.versesCount}'),
                _buildInfoRow('Revelation:', surahInfo.revelationPlace),
                _buildInfoRow('Order:', '${surahInfo.revelationOrder}'),
                const SizedBox(height: 20),
                const Text(
                  'Ayahs:',
                  style: TextStyle(fontWeight: FontWeight.bold),
                ),
                const SizedBox(height: 10),
                Expanded(
                  child: ListView.builder(
                    itemCount: ayahs.length,
                    itemBuilder: (context, index) {
                      final ayah = ayahs[index];
                      return Card(
                        child: ListTile(
                          title: Text('${ayah.ayahNumber}'),
                          subtitle: Text(
                            ayah.arabicText,
                            style: const TextStyle(fontSize: 16),
                          ),
                          onTap: () {
                            Navigator.of(context).pop();
                            _showAyahDetails(ayah);
                          },
                        ),
                      );
                    },
                  ),
                ),
              ],
            ),
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.of(context).pop(),
              child: const Text('Close'),
            ),
          ],
        ),
      );
    } catch (e) {
      _showErrorMessage('Error: $e');
    }
  }
  
  void _showQuranStatistics() async {
    if (!_isInitialized) return;
    
    try {
      final stats = _bindings.getQuranStatistics();
      if (stats == null) {
        _showErrorMessage('Failed to get statistics');
        return;
      }
      
      showDialog(
        context: context,
        builder: (context) => AlertDialog(
          title: const Text('Quran Statistics'),
          content: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              _buildInfoRow('Total Surahs:', '${stats.totalSurahs}'),
              _buildInfoRow('Total Ayahs:', '${stats.totalAyahs}'),
              _buildInfoRow('Total Words:', '${stats.totalWords}'),
              _buildInfoRow('Total Letters:', '${stats.totalLetters}'),
              _buildInfoRow('Total Pages:', '${stats.totalPages}'),
              _buildInfoRow('Total Juz:', '${stats.totalJuz}'),
              _buildInfoRow('Total Hizb:', '${stats.totalHizb}'),
              _buildInfoRow('Total Ruku:', '${stats.totalRuku}'),
              _buildInfoRow('Total Manzil:', '${stats.totalManzil}'),
              _buildInfoRow('Total Sajda:', '${stats.totalSajda}'),
            ],
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.of(context).pop(),
              child: const Text('Close'),
            ),
          ],
        ),
      );
    } catch (e) {
      _showErrorMessage('Error: $e');
    }
  }
  
  void _showErrorMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(message),
        backgroundColor: Colors.red,
      ),
    );
  }
  
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Hafiz Assistant'),
        backgroundColor: Colors.green,
      ),
      body: Center(
        child: _isLoading
            ? const CircularProgressIndicator()
            : Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Card(
                    child: Padding(
                      padding: const EdgeInsets.all(16.0),
                      child: Text(
                        'Status: $_status',
                        style: const TextStyle(fontSize: 16),
                      ),
                    ),
                  ),
                  const SizedBox(height: 20),
                  if (_isInitialized) ...[
                    ElevatedButton.icon(
                      onPressed: _getRandomAyah,
                      icon: const Icon(Icons.shuffle),
                      label: const Text('Get Random Ayah'),
                    ),
                    const SizedBox(height: 10),
                    ElevatedButton.icon(
                      onPressed: _searchAyahs,
                      icon: const Icon(Icons.search),
                      label: const Text('Search Ayahs'),
                    ),
                    const SizedBox(height: 10),
                    ElevatedButton.icon(
                      onPressed: _showSurahList,
                      icon: const Icon(Icons.list),
                      label: const Text('Browse Surahs'),
                    ),
                    const SizedBox(height: 10),
                    ElevatedButton.icon(
                      onPressed: _showQuranStatistics,
                      icon: const Icon(Icons.bar_chart),
                      label: const Text('Quran Statistics'),
                    ),
                  ] else ...[
                    ElevatedButton.icon(
                      onPressed: _initializeBindings,
                      icon: const Icon(Icons.refresh),
                      label: const Text('Retry Initialization'),
                    ),
                  ],
                ],
              ),
      ),
    );
  }
}

// Main function for the example app
void main() {
  runApp(const MaterialApp(
    title: 'Hafiz Assistant Demo',
    home: HafizAssistantApp(),
  ));
}
