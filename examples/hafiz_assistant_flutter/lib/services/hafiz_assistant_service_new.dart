import '../models/ayah_data.dart';
import '../models/surah_info.dart';
import '../models/quran_statistics.dart';
import '../ffi/hafiz_assistant_ffi.dart';
import '../ffi/ffi_helper.dart';

class HafizAssistantService {
  static HafizAssistantService? _instance;
  final HafizAssistantFFI _ffi = HafizAssistantFFI.instance;
  bool _isInitialized = false;

  HafizAssistantService._();

  static HafizAssistantService get instance {
    _instance ??= HafizAssistantService._();
    return _instance!;
  }

  /// Initialize the service and FFI
  Future<bool> initialize() async {
    if (_isInitialized) return true;
    
    _isInitialized = _ffi.initialize();
    return _isInitialized;
  }

  /// Get a random ayah from the Quran
  Future<AyahData?> getRandomAyah() async {
    if (!_isInitialized) return null;
    
    final nativePtr = _ffi.getRandomAyah();
    final ayah = FFIHelper.convertAyahData(nativePtr);
    
    if (nativePtr != null) {
      _ffi.freeAyahData(nativePtr);
    }
    
    return ayah;
  }

  /// Get a specific ayah by surah and ayah number
  Future<AyahData?> getAyah(int surahNumber, int ayahNumber) async {
    if (!_isInitialized) return null;
    
    final nativePtr = _ffi.getAyah(surahNumber, ayahNumber);
    final ayah = FFIHelper.convertAyahData(nativePtr);
    
    if (nativePtr != null) {
      _ffi.freeAyahData(nativePtr);
    }
    
    return ayah;
  }

  /// Get information about a specific surah
  Future<SurahInfo?> getSurahInfo(int surahNumber) async {
    if (!_isInitialized) return null;
    
    final nativePtr = _ffi.getSurahInfo(surahNumber);
    final surahInfo = FFIHelper.convertSurahInfo(nativePtr);
    
    if (nativePtr != null) {
      _ffi.freeSurahInfo(nativePtr);
    }
    
    return surahInfo;
  }

  /// Get all surahs information
  Future<List<SurahInfo>> getAllSurahs() async {
    if (!_isInitialized) return [];
    
    List<SurahInfo> surahs = [];
    
    // Get all 114 surahs
    for (int i = 1; i <= 114; i++) {
      final surahInfo = await getSurahInfo(i);
      if (surahInfo != null) {
        surahs.add(surahInfo);
      }
    }
    
    return surahs;
  }

  /// Search ayahs by text
  Future<List<AyahData>> searchAyahsByText(String query) async {
    if (!_isInitialized || query.trim().isEmpty) return [];
    
    final nativePtrs = _ffi.searchAyahs(query);
    final results = FFIHelper.convertSearchResults(nativePtrs);
    
    // Free native memory
    _ffi.freeSearchResults(nativePtrs);
    
    return results;
  }

  /// Get Quran statistics
  Future<QuranStatistics?> getQuranStatistics() async {
    if (!_isInitialized) return null;
    
    final nativePtr = _ffi.getQuranStatistics();
    final statistics = FFIHelper.convertQuranStatistics(nativePtr);
    
    if (nativePtr != null) {
      _ffi.freeStatistics(nativePtr);
    }
    
    return statistics;
  }

  /// Get ayahs for a specific surah
  Future<List<AyahData>> getSurahAyahs(int surahNumber) async {
    if (!_isInitialized) return [];
    
    final surahInfo = await getSurahInfo(surahNumber);
    if (surahInfo == null) return [];
    
    List<AyahData> ayahs = [];
    
    for (int i = 1; i <= surahInfo.versesCount; i++) {
      final ayah = await getAyah(surahNumber, i);
      if (ayah != null) {
        ayahs.add(ayah);
      }
    }
    
    return ayahs;
  }

  /// Get ayah data by verse key (compatibility method)
  Future<AyahData?> getAyahData(String verseKey) async {
    final parts = verseKey.split(':');
    if (parts.length != 2) return null;
    
    final surahNumber = int.tryParse(parts[0]);
    final ayahNumber = int.tryParse(parts[1]);
    
    if (surahNumber == null || ayahNumber == null) return null;
    
    return await getAyah(surahNumber, ayahNumber);
  }

  /// Check if service is initialized
  bool get isInitialized => _isInitialized;
}
