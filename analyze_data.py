#!/usr/bin/env python3
"""
Analisis Data untuk Hafiz Assistant Backend
Script untuk menganalisis struktur file JSON dan membantu debugging
"""

import json
import os
import sys
from pathlib import Path

def analyze_json_file(file_path):
    """Analisis struktur file JSON"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        print(f"\n📄 File: {file_path}")
        print(f"📊 Type: {type(data)}")
        
        if isinstance(data, dict):
            print(f"🔑 Keys: {list(data.keys())[:10]}")
            
            # Analisis setiap key
            for key in list(data.keys())[:5]:
                value = data[key]
                print(f"  • {key}: {type(value)} - {str(value)[:100]}...")
            
            # Cek struktur khusus
            if 'verses' in data:
                print(f"📖 Found 'verses' key")
                verses = data['verses']
                if isinstance(verses, dict):
                    verse_keys = list(verses.keys())[:5]
                    print(f"  First 5 verse keys: {verse_keys}")
                    
                    # Analisis ayat pertama
                    first_verse = verses[verse_keys[0]]
                    print(f"  First verse structure: {type(first_verse)}")
                    if isinstance(first_verse, dict):
                        print(f"  First verse keys: {list(first_verse.keys())}")
        
        elif isinstance(data, list):
            print(f"📏 Length: {len(data)}")
            if data:
                print(f"🔍 First item type: {type(data[0])}")
                print(f"🔍 First item: {str(data[0])[:200]}...")
                
                if isinstance(data[0], dict):
                    print(f"🔑 First item keys: {list(data[0].keys())}")
        
        return data
        
    except Exception as e:
        print(f"❌ Error reading {file_path}: {e}")
        return None

def analyze_all_data_files():
    """Analisis semua file data"""
    data_dir = Path("data")
    
    if not data_dir.exists():
        print("❌ Data directory not found!")
        return
    
    print("🔍 Analyzing all data files...")
    
    json_files = list(data_dir.glob("*.json"))
    
    for json_file in json_files:
        analyze_json_file(json_file)
    
    print(f"\n📊 Total files analyzed: {len(json_files)}")

def check_specific_structure():
    """Cek struktur file yang dibutuhkan oleh loader"""
    
    required_files = [
        "quran-metadata-ayah.json",
        "quran-id-simple.json", 
        "indonesian-mokhtasar.json",
        "transliteration-simple.json",
        "surah-info-id.json",
        "matching-ayah.json",
        "qpc-v2-15-lines.db.json"
    ]
    
    print("🔍 Checking required files structure...")
    
    for file_name in required_files:
        file_path = Path("data") / file_name
        if file_path.exists():
            print(f"\n✅ {file_name} exists")
            
            # Analisis khusus untuk setiap file
            if file_name == "quran-metadata-ayah.json":
                check_ayah_metadata_structure(file_path)
            elif file_name == "quran-id-simple.json":
                check_simple_quran_structure(file_path)
            elif file_name == "matching-ayah.json":
                check_matching_ayah_structure(file_path)
        else:
            print(f"❌ {file_name} missing")

def check_ayah_metadata_structure(file_path):
    """Cek struktur khusus untuk ayah metadata"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        if isinstance(data, dict) and 'verses' in data:
            verses = data['verses']
            print(f"  📖 Found {len(verses)} verses")
            
            # Cek beberapa ayat pertama
            verse_keys = list(verses.keys())[:3]
            for key in verse_keys:
                verse = verses[key]
                print(f"  • {key}: {list(verse.keys()) if isinstance(verse, dict) else type(verse)}")
                
        elif isinstance(data, list):
            print(f"  📏 Array with {len(data)} items")
            if data:
                print(f"  🔍 First item keys: {list(data[0].keys()) if isinstance(data[0], dict) else type(data[0])}")
        
    except Exception as e:
        print(f"  ❌ Error: {e}")

def check_simple_quran_structure(file_path):
    """Cek struktur quran simple"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        if isinstance(data, dict):
            print(f"  🔑 Top level keys: {list(data.keys())}")
            
            # Cek struktur chapters/surahs
            if 'chapters' in data:
                chapters = data['chapters']
                print(f"  📚 Found {len(chapters)} chapters")
            elif 'surahs' in data:
                surahs = data['surahs']
                print(f"  📚 Found {len(surahs)} surahs")
                
    except Exception as e:
        print(f"  ❌ Error: {e}")

def check_matching_ayah_structure(file_path):
    """Cek struktur matching ayah"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        if isinstance(data, dict):
            print(f"  🔗 Matching entries: {len(data)}")
            
            # Cek beberapa entry pertama
            keys = list(data.keys())[:3]
            for key in keys:
                matches = data[key]
                print(f"  • {key}: {len(matches) if isinstance(matches, list) else type(matches)} matches")
                
    except Exception as e:
        print(f"  ❌ Error: {e}")

def generate_rust_mapping():
    """Generate mapping code untuk Rust berdasarkan struktur aktual"""
    print("\n🦀 Generating Rust mapping suggestions...")
    
    # Analisis quran-metadata-ayah.json
    ayah_file = Path("data/quran-metadata-ayah.json")
    if ayah_file.exists():
        try:
            with open(ayah_file, 'r', encoding='utf-8') as f:
                data = json.load(f)
            
            print("\n// Suggested Rust structure for ayah metadata:")
            
            if isinstance(data, dict) and 'verses' in data:
                verses = data['verses']
                first_key = list(verses.keys())[0]
                first_verse = verses[first_key]
                
                if isinstance(first_verse, dict):
                    print("// Based on actual data structure:")
                    print("#[derive(Debug, Serialize, Deserialize)]")
                    print("pub struct AyahMetadata {")
                    
                    for key, value in first_verse.items():
                        rust_type = get_rust_type(value)
                        print(f"    pub {key}: {rust_type},")
                    
                    print("}")
                    
        except Exception as e:
            print(f"❌ Error generating mapping: {e}")

def get_rust_type(value):
    """Convert Python type to Rust type"""
    if isinstance(value, str):
        return "String"
    elif isinstance(value, int):
        return "u32"
    elif isinstance(value, float):
        return "f64"
    elif isinstance(value, bool):
        return "bool"
    elif isinstance(value, list):
        return "Vec<String>"  # Simplified
    elif isinstance(value, dict):
        return "HashMap<String, String>"  # Simplified
    else:
        return "String"

def main():
    """Main function"""
    print("🚀 Hafiz Assistant Backend - Data Analysis Tool")
    print("=" * 50)
    
    if len(sys.argv) > 1:
        command = sys.argv[1]
        
        if command == "all":
            analyze_all_data_files()
        elif command == "check":
            check_specific_structure()
        elif command == "mapping":
            generate_rust_mapping()
        elif command == "file" and len(sys.argv) > 2:
            file_path = sys.argv[2]
            analyze_json_file(file_path)
        else:
            print("❌ Unknown command")
    else:
        print("📋 Available commands:")
        print("  python analyze_data.py all      - Analyze all JSON files")
        print("  python analyze_data.py check    - Check required files structure")
        print("  python analyze_data.py mapping  - Generate Rust mapping")
        print("  python analyze_data.py file <path> - Analyze specific file")
        print()
        print("🔍 Running default analysis...")
        check_specific_structure()

if __name__ == "__main__":
    main()
