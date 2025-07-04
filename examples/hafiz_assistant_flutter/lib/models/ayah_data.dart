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

  @override
  String toString() {
    return 'AyahData(verseKey: $verseKey, arabicText: $arabicText)';
  }

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    return other is AyahData && other.verseKey == verseKey;
  }

  @override
  int get hashCode => verseKey.hashCode;

  /// Get formatted verse display
  String get formattedVerseKey => verseKey;
  
  /// Get short surah name
  String get shortSurahName => surahName.length > 20 ? '${surahName.substring(0, 20)}...' : surahName;
  
  /// Get formatted location info
  String get locationInfo => 'Juz $juzNumber, Page $pageNumber';
  
  /// Check if this is the first verse of a surah
  bool get isFirstVerse => ayahNumber == 1;
}
