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

  @override
  String toString() {
    return 'SurahInfo(id: $id, nameEnglish: $nameEnglish, versesCount: $versesCount)';
  }

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    return other is SurahInfo && other.id == id;
  }

  @override
  int get hashCode => id.hashCode;

  /// Get formatted surah title
  String get formattedTitle => '$id. $nameEnglish';
  
  /// Get revelation type display
  String get revelationType => revelationPlace == 'Makkah' ? 'Makkiyah' : 'Madaniyah';
  
  /// Get verses count display
  String get versesDisplay => '$versesCount verses';
  
  /// Get comprehensive name display
  String get fullNameDisplay => '$nameEnglish ($nameArabic)';
}
