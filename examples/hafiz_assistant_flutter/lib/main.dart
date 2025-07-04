import 'package:flutter/material.dart';
import 'screens/home_screen.dart';
import 'services/hafiz_assistant_service.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  
  // Initialize the Hafiz Assistant Service
  final service = HafizAssistantService.instance;
  await service.initialize();
  
  runApp(const HafizAssistantApp());
}

class HafizAssistantApp extends StatelessWidget {
  const HafizAssistantApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Hafiz Assistant',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: Colors.teal,
          brightness: Brightness.light,
        ),
        useMaterial3: true,
        appBarTheme: const AppBarTheme(
          centerTitle: true,
          elevation: 0,
        ),
      ),
      home: const HomeScreen(),
    );
  }
}
