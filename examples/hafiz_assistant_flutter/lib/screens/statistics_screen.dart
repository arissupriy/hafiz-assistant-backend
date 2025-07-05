import 'package:flutter/material.dart';
import '../models/quran_statistics.dart';
import '../services/hafiz_assistant_service.dart';

class StatisticsScreen extends StatefulWidget {
  const StatisticsScreen({super.key});

  @override
  State<StatisticsScreen> createState() => _StatisticsScreenState();
}

class _StatisticsScreenState extends State<StatisticsScreen> {
  QuranStatistics? _statistics;
  bool _isLoading = true;

  @override
  void initState() {
    super.initState();
    _loadStatistics();
  }

  void _loadStatistics() async {
    try {
      final service = HafizAssistantService.instance;
      final stats = await service.getQuranStatistics();
      setState(() {
        _statistics = stats;
        _isLoading = false;
      });
    } catch (e) {
      setState(() {
        _isLoading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Quran Statistics'),
        backgroundColor: Theme.of(context).colorScheme.surfaceVariant,
      ),
      body: _isLoading
          ? const Center(child: CircularProgressIndicator())
          : _statistics == null
              ? const Center(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Icon(
                        Icons.error_outline,
                        size: 64,
                        color: Colors.grey,
                      ),
                      SizedBox(height: 16),
                      Text(
                        'Failed to load statistics',
                        style: TextStyle(
                          fontSize: 18,
                          color: Colors.grey,
                        ),
                      ),
                    ],
                  ),
                )
              : _buildStatistics(),
    );
  }

  Widget _buildStatistics() {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Main statistics
          _buildStatisticCard(
            title: 'General Statistics',
            icon: Icons.analytics,
            children: [
              _buildStatisticRow('Total Surahs', _statistics!.totalSurahs),
              _buildStatisticRow('Total Ayahs', _statistics!.totalAyahs),
              _buildStatisticRow('Total Words', _statistics!.totalWords),
              _buildStatisticRow('Total Letters', _statistics!.totalLetters),
            ],
          ),
          
          const SizedBox(height: 16),
          
          // Divisions
          _buildStatisticCard(
            title: 'Divisions',
            icon: Icons.format_list_numbered,
            children: [
              _buildStatisticRow('Total Juz', _statistics!.totalJuz),
              _buildStatisticRow('Total Hizb', _statistics!.totalHizb),
              _buildStatisticRow('Total Ruku', _statistics!.totalRuku),
              _buildStatisticRow('Total Manzil', _statistics!.totalManzil),
            ],
          ),
          
          const SizedBox(height: 16),
          
          // Revelation details
          _buildStatisticCard(
            title: 'Additional Details',
            icon: Icons.place,
            children: [
              _buildStatisticRow('Total Pages', _statistics!.totalPages),
              _buildStatisticRow('Total Sajda', _statistics!.totalSajda),
            ],
          ),
          
          const SizedBox(height: 16),
          
          // Additional info
          Card(
            child: Padding(
              padding: const EdgeInsets.all(16.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Icon(
                        Icons.info_outline,
                        color: Theme.of(context).colorScheme.primary,
                      ),
                      const SizedBox(width: 8),
                      Text(
                        'Additional Information',
                        style: Theme.of(context).textTheme.titleMedium?.copyWith(
                              fontWeight: FontWeight.bold,
                            ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 12),
                  const Text(
                    'The Holy Quran is the central religious text of Islam, '
                    'believed to be a revelation from Allah (God) to the Prophet Muhammad. '
                    'It is divided into chapters called Surahs, which are further divided into verses called Ayahs.',
                    style: TextStyle(height: 1.5),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildStatisticCard({
    required String title,
    required IconData icon,
    required List<Widget> children,
  }) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(
                  icon,
                  color: Theme.of(context).colorScheme.primary,
                ),
                const SizedBox(width: 8),
                Text(
                  title,
                  style: Theme.of(context).textTheme.titleMedium?.copyWith(
                        fontWeight: FontWeight.bold,
                      ),
                ),
              ],
            ),
            const SizedBox(height: 12),
            ...children,
          ],
        ),
      ),
    );
  }

  Widget _buildStatisticRow(String label, int value) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4.0),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Text(
            label,
            style: Theme.of(context).textTheme.bodyMedium,
          ),
          Text(
            value.toString(),
            style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                  fontWeight: FontWeight.bold,
                  color: Theme.of(context).colorScheme.primary,
                ),
          ),
        ],
      ),
    );
  }
}
