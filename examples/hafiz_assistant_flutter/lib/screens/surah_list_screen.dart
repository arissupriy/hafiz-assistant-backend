import 'package:flutter/material.dart';
import '../models/surah_info.dart';
import '../services/hafiz_assistant_service.dart';
import '../widgets/surah_card.dart';

class SurahListScreen extends StatefulWidget {
  const SurahListScreen({super.key});

  @override
  State<SurahListScreen> createState() => _SurahListScreenState();
}

class _SurahListScreenState extends State<SurahListScreen> {
  List<SurahInfo> _surahs = [];
  List<SurahInfo> _filteredSurahs = [];
  final TextEditingController _searchController = TextEditingController();
  bool _isLoading = true;

  @override
  void initState() {
    super.initState();
    _loadSurahs();
  }

  @override
  void dispose() {
    _searchController.dispose();
    super.dispose();
  }

  void _loadSurahs() async {
    try {
      final service = HafizAssistantService.instance;
      final surahs = await service.getAllSurahs();
      setState(() {
        _surahs = surahs;
        _filteredSurahs = surahs;
        _isLoading = false;
      });
    } catch (e) {
      setState(() {
        _isLoading = false;
      });
      // Handle error
    }
  }

  void _filterSurahs(String query) {
    if (query.isEmpty) {
      setState(() {
        _filteredSurahs = _surahs;
      });
      return;
    }

    setState(() {
      _filteredSurahs = _surahs.where((surah) {
        return surah.nameSimple.toLowerCase().contains(query.toLowerCase()) ||
            surah.nameEnglish.toLowerCase().contains(query.toLowerCase()) ||
            surah.nameArabic.contains(query) ||
            surah.id.toString().contains(query);
      }).toList();
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Surahs'),
        backgroundColor: Theme.of(context).colorScheme.surfaceVariant,
      ),
      body: Column(
        children: [
          // Search bar
          Padding(
            padding: const EdgeInsets.all(16.0),
            child: TextField(
              controller: _searchController,
              decoration: InputDecoration(
                hintText: 'Search surahs...',
                prefixIcon: const Icon(Icons.search),
                suffixIcon: _searchController.text.isNotEmpty
                    ? IconButton(
                        icon: const Icon(Icons.clear),
                        onPressed: () {
                          _searchController.clear();
                          _filterSurahs('');
                        },
                      )
                    : null,
                border: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(12),
                ),
                filled: true,
                fillColor: Theme.of(context).colorScheme.surfaceVariant,
              ),
              onChanged: _filterSurahs,
            ),
          ),
          
          // Surah list
          Expanded(
            child: _isLoading
                ? const Center(child: CircularProgressIndicator())
                : _filteredSurahs.isEmpty
                    ? const Center(
                        child: Column(
                          mainAxisAlignment: MainAxisAlignment.center,
                          children: [
                            Icon(
                              Icons.search_off,
                              size: 64,
                              color: Colors.grey,
                            ),
                            SizedBox(height: 16),
                            Text(
                              'No surahs found',
                              style: TextStyle(
                                fontSize: 18,
                                color: Colors.grey,
                              ),
                            ),
                          ],
                        ),
                      )
                    : ListView.builder(
                        itemCount: _filteredSurahs.length,
                        itemBuilder: (context, index) {
                          final surah = _filteredSurahs[index];
                          return SurahCard(
                            surahInfo: surah,
                            onTap: () {
                              _navigateToSurahDetail(surah);
                            },
                          );
                        },
                      ),
          ),
        ],
      ),
    );
  }

  void _navigateToSurahDetail(SurahInfo surah) {
    // For now, just show a dialog with surah info
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: Text(surah.nameSimple),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('English: ${surah.nameEnglish}'),
            Text('Arabic: ${surah.nameArabic}'),
            Text('Number: ${surah.id}'),
            Text('Verses: ${surah.versesCount}'),
            Text('Revelation: ${surah.revelationPlace}'),
            Text('Order: ${surah.revelationOrder}'),
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
  }
}
