import 'dart:ffi';
import 'package:ffi/ffi.dart';
import '../models/ayah_data.dart';
import '../models/surah_info.dart';
import '../models/quran_statistics.dart';
import 'hafiz_assistant_ffi.dart';

class FFIHelper {
  static AyahData? convertAyahData(Pointer<AyahDataNative>? nativePtr) {
    if (nativePtr == null || nativePtr == nullptr) return null;
    
    final native = nativePtr.ref;
    
    return AyahData(
      verseKey: '${native.surahNumber}:${native.ayahNumber}',
      surahNumber: native.surahNumber,
      ayahNumber: native.ayahNumber,
      arabicText: native.arabicText.toDartString(),
      transliteration: native.transliteration.toDartString(),
      translation: native.translation.toDartString(),
      surahName: native.surahNameTransliteration.toDartString(),
      surahNameArabic: native.surahNameArabic.toDartString(),
      juzNumber: 1, // Default values - would need to be calculated or stored
      hizbNumber: 1,
      rubNumber: 1,
      rukuNumber: 1,
      manzilNumber: 1,
      pageNumber: 1,
    );
  }

  static SurahInfo? convertSurahInfo(Pointer<SurahInfoNative>? nativePtr) {
    if (nativePtr == null || nativePtr == nullptr) return null;
    
    final native = nativePtr.ref;
    
    return SurahInfo(
      id: native.number,
      nameSimple: native.nameTransliteration.toDartString(),
      nameArabic: native.nameArabic.toDartString(),
      nameEnglish: native.nameTranslation.toDartString(),
      revelationOrder: native.number, // Placeholder - would need actual revelation order
      revelationPlace: native.revelationType == 0 ? 'Makkah' : 'Madinah',
      versesCount: native.ayahCount,
      bismillahPre: native.number != 1 && native.number != 9, // All except Al-Fatihah and At-Tawbah
    );
  }

  static QuranStatistics? convertQuranStatistics(Pointer<QuranStatisticsNative>? nativePtr) {
    if (nativePtr == null || nativePtr == nullptr) return null;
    
    final native = nativePtr.ref;
    
    return QuranStatistics(
      totalSurahs: native.totalSurahs,
      totalAyahs: native.totalAyahs,
      totalWords: 77800, // Default values - would need to be added to Rust struct
      totalLetters: 330000,
      totalPages: 604,
      totalJuz: 30,
      totalHizb: 60,
      totalRuku: 540,
      totalManzil: 7,
      totalSajda: 15,
    );
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
}
