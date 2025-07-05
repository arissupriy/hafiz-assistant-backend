import 'dart:ffi';
import 'package:ffi/ffi.dart';
import '../models/ayah_data.dart';
import '../models/surah_info.dart';
import '../models/quran_statistics.dart';
import 'hafiz_assistant_ffi_corrected.dart';

class FFIHelper {
  static AyahData? convertAyahData(Pointer<AyahDataNative>? nativePtr) {
    if (nativePtr == null || nativePtr == nullptr) return null;
    
    try {
      final native = nativePtr.ref;
      
      return AyahData(
        id: 0, // Will be set based on surah/ayah numbers
        surahNumber: native.surahNumber,
        ayahNumber: native.ayahNumber,
        verseKey: _safeStringFromPointer(native.verseKey) ?? '${native.surahNumber}:${native.ayahNumber}',
        textArab: _safeStringFromPointer(native.arabicText) ?? '',
        textUthmani: _safeStringFromPointer(native.arabicText) ?? '',
        translationId: _safeStringFromPointer(native.translation) ?? '',
        transliteration: _safeStringFromPointer(native.transliteration) ?? '',
        surahName: _safeStringFromPointer(native.surahName) ?? '',
        surahNameArabic: _safeStringFromPointer(native.surahNameArabic) ?? '',
        juzNumber: native.juzNumber,
        hizbNumber: native.hizbNumber,
        rubNumber: native.rubNumber,
        rukuNumber: native.rukuNumber,
        manzilNumber: native.manzilNumber,
        pageNumber: native.pageNumber,
        lineNumber: 1, // Default value
        sajda: null, // Will be populated if needed
      );
    } catch (e) {
      print('Error converting AyahData: $e');
      return null;
    }
  }

  static SurahInfo? convertSurahInfo(Pointer<SurahInfoNative>? nativePtr) {
    if (nativePtr == null || nativePtr == nullptr) return null;
    
    try {
      final native = nativePtr.ref;
      
      return SurahInfo(
        id: native.id,
        nameSimple: _safeStringFromPointer(native.nameSimple) ?? '',
        nameArabic: _safeStringFromPointer(native.nameArabic) ?? '',
        nameEnglish: _safeStringFromPointer(native.nameEnglish) ?? '',
        revelationOrder: native.revelationOrder,
        revelationPlace: _safeStringFromPointer(native.revelationPlace) ?? '',
        versesCount: native.versesCount,
        bismillahPre: native.bismillahPre,
      );
    } catch (e) {
      print('Error converting SurahInfo: $e');
      return null;
    }
  }

  static QuranStatistics? convertQuranStatistics(Pointer<QuranStatisticsNative>? nativePtr) {
    if (nativePtr == null || nativePtr == nullptr) return null;
    
    try {
      final native = nativePtr.ref;
      
      return QuranStatistics(
        totalSurahs: native.totalSurahs,
        totalAyahs: native.totalAyahs,
        totalWords: native.totalWords,
        totalLetters: native.totalLetters,
        totalPages: native.totalPages,
        totalJuz: native.totalJuz,
        totalHizb: native.totalHizb,
        totalRuku: native.totalRuku,
        totalManzil: native.totalManzil,
        totalSajda: native.totalSajda,
      );
    } catch (e) {
      print('Error converting QuranStatistics: $e');
      return null;
    }
  }

  static List<AyahData> convertSearchResults(List<Pointer<AyahDataNative>> nativePtrs) {
    List<AyahData> results = [];
    
    for (final ptr in nativePtrs) {
      final ayah = convertAyahData(ptr);
      if (ayah != null) {
        results.add(ayah);
      }
    }
    
    return results;
  }

  static String? _safeStringFromPointer(Pointer<Utf8> pointer) {
    try {
      if (pointer == nullptr) return null;
      return pointer.toDartString();
    } catch (e) {
      print('Error converting string from pointer: $e');
      return null;
    }
  }
}
