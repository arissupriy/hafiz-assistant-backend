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

  @override
  String toString() {
    return 'QuranStatistics(totalSurahs: $totalSurahs, totalAyahs: $totalAyahs)';
  }

  /// Get formatted statistics as a map for display
  Map<String, String> get formattedStats => {
    'Total Surahs': totalSurahs.toString(),
    'Total Ayahs': totalAyahs.toString(),
    'Total Words': totalWords.toString(),
    'Total Letters': totalLetters.toString(),
    'Total Pages': totalPages.toString(),
    'Total Juz': totalJuz.toString(),
    'Total Hizb': totalHizb.toString(),
    'Total Ruku': totalRuku.toString(),
    'Total Manzil': totalManzil.toString(),
    'Total Sajda': totalSajda.toString(),
  };

  /// Get essential statistics for quick view
  Map<String, String> get essentialStats => {
    'Surahs': totalSurahs.toString(),
    'Ayahs': totalAyahs.toString(),
    'Pages': totalPages.toString(),
    'Juz': totalJuz.toString(),
  };
}
