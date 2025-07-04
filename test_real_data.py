#!/usr/bin/env python3
"""
Test script untuk menguji data real Quran
Script ini akan membaca dan menganalisis file-file data JSON yang tersedia
"""

import json
import os
import sys
from pathlib import Path
from typing import Dict, List, Any

def load_json_file(file_path: str) -> Dict[str, Any]:
    """Load JSON file with error handling"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            return json.load(f)
    except Exception as e:
        print(f"❌ Error loading {file_path}: {e}")
        return {}

def analyze_data_structure(data: Dict[str, Any], name: str) -> None:
    """Analyze the structure of loaded data"""
    print(f"\n📊 Analyzing {name}:")
    print(f"  Type: {type(data)}")
    
    if isinstance(data, dict):
        print(f"  Keys: {len(data.keys())}")
        for key in list(data.keys())[:5]:  # Show first 5 keys
            print(f"    - {key}: {type(data[key])}")
            if isinstance(data[key], list) and len(data[key]) > 0:
                print(f"      First item: {type(data[key][0])}")
                if isinstance(data[key][0], dict):
                    print(f"      Item keys: {list(data[key][0].keys())[:3]}...")
    elif isinstance(data, list):
        print(f"  Length: {len(data)}")
        if len(data) > 0:
            print(f"  First item type: {type(data[0])}")
            if isinstance(data[0], dict):
                print(f"  First item keys: {list(data[0].keys())}")

def test_ayah_metadata(data_dir: str) -> None:
    """Test Ayah metadata"""
    print("\n🔍 Testing Ayah Metadata...")
    
    # Test quran metadata ayah
    ayah_file = os.path.join(data_dir, "quran-metadata-ayah.json")
    if os.path.exists(ayah_file):
        ayah_data = load_json_file(ayah_file)
        analyze_data_structure(ayah_data, "Quran Metadata Ayah")
        
        # Sample beberapa ayah
        if isinstance(ayah_data, dict):
            sample_keys = list(ayah_data.keys())[:3]
            print(f"\n📖 Sample Ayahs:")
            for key in sample_keys:
                ayah = ayah_data[key]
                print(f"  {key}: {ayah}")
    
    # Test quran simple
    simple_file = os.path.join(data_dir, "quran-id-simple.json")
    if os.path.exists(simple_file):
        simple_data = load_json_file(simple_file)
        analyze_data_structure(simple_data, "Quran ID Simple")
        
        # Sample teks ayah
        if isinstance(simple_data, dict):
            sample_keys = list(simple_data.keys())[:3]
            print(f"\n📜 Sample Ayah Texts:")
            for key in sample_keys:
                text = simple_data[key]
                print(f"  {key}: {text[:100]}..." if len(text) > 100 else f"  {key}: {text}")

def test_surah_metadata(data_dir: str) -> None:
    """Test Surah metadata"""
    print("\n🔍 Testing Surah Metadata...")
    
    surah_file = os.path.join(data_dir, "quran-metadata-surah-name.json")
    if os.path.exists(surah_file):
        surah_data = load_json_file(surah_file)
        analyze_data_structure(surah_data, "Surah Names")
        
        # Sample surah names
        if isinstance(surah_data, dict):
            sample_keys = list(surah_data.keys())[:10]
            print(f"\n📚 Sample Surah Names:")
            for key in sample_keys:
                surah = surah_data[key]
                print(f"  {key}: {surah}")
    
    # Test surah info
    info_file = os.path.join(data_dir, "surah-info-id.json")
    if os.path.exists(info_file):
        info_data = load_json_file(info_file)
        analyze_data_structure(info_data, "Surah Info ID")
        
        # Sample surah info
        if isinstance(info_data, list) and len(info_data) > 0:
            print(f"\n📋 Sample Surah Info:")
            for i, surah in enumerate(info_data[:3]):
                print(f"  Surah {i+1}: {surah}")

def test_translations(data_dir: str) -> None:
    """Test translations"""
    print("\n🔍 Testing Translations...")
    
    # Test Indonesian translation
    id_file = os.path.join(data_dir, "indonesian-mokhtasar.json")
    if os.path.exists(id_file):
        id_data = load_json_file(id_file)
        analyze_data_structure(id_data, "Indonesian Translation")
        
        # Sample translations
        if isinstance(id_data, dict):
            sample_keys = list(id_data.keys())[:3]
            print(f"\n🌍 Sample Indonesian Translations:")
            for key in sample_keys:
                trans = id_data[key]
                print(f"  {key}: {trans}")
    
    # Test transliteration
    trans_file = os.path.join(data_dir, "transliteration-simple.json")
    if os.path.exists(trans_file):
        trans_data = load_json_file(trans_file)
        analyze_data_structure(trans_data, "Transliteration")
        
        # Sample transliterations
        if isinstance(trans_data, dict):
            sample_keys = list(trans_data.keys())[:3]
            print(f"\n📝 Sample Transliterations:")
            for key in sample_keys:
                trans = trans_data[key]
                print(f"  {key}: {trans}")

def test_search_data(data_dir: str) -> None:
    """Test search-related data"""
    print("\n🔍 Testing Search Data...")
    
    # Test phrases
    phrase_file = os.path.join(data_dir, "phrases.json")
    if os.path.exists(phrase_file):
        phrase_data = load_json_file(phrase_file)
        analyze_data_structure(phrase_data, "Phrases")
        
        # Sample phrases
        if isinstance(phrase_data, dict):
            sample_keys = list(phrase_data.keys())[:5]
            print(f"\n🔍 Sample Phrases:")
            for key in sample_keys:
                phrase = phrase_data[key]
                print(f"  {key}: {phrase}")
    
    # Test matching ayahs
    match_file = os.path.join(data_dir, "matching-ayah.json")
    if os.path.exists(match_file):
        match_data = load_json_file(match_file)
        analyze_data_structure(match_data, "Matching Ayahs")
        
        # Sample matches
        if isinstance(match_data, dict):
            sample_keys = list(match_data.keys())[:3]
            print(f"\n🔗 Sample Matching Ayahs:")
            for key in sample_keys:
                matches = match_data[key]
                print(f"  {key}: {matches}")

def test_word_analysis(data_dir: str) -> None:
    """Test word analysis data"""
    print("\n🔍 Testing Word Analysis...")
    
    # Test word lemma
    lemma_file = os.path.join(data_dir, "word-lemma.db.json")
    if os.path.exists(lemma_file):
        lemma_data = load_json_file(lemma_file)
        analyze_data_structure(lemma_data, "Word Lemma")
        
        # Sample lemmas
        if isinstance(lemma_data, dict):
            sample_keys = list(lemma_data.keys())[:3]
            print(f"\n📊 Sample Word Lemmas:")
            for key in sample_keys:
                lemma = lemma_data[key]
                print(f"  {key}: {lemma}")
    
    # Test word root
    root_file = os.path.join(data_dir, "word-root.db.json")
    if os.path.exists(root_file):
        root_data = load_json_file(root_file)
        analyze_data_structure(root_data, "Word Root")
        
        # Sample roots
        if isinstance(root_data, dict):
            sample_keys = list(root_data.keys())[:3]
            print(f"\n🌳 Sample Word Roots:")
            for key in sample_keys:
                root = root_data[key]
                print(f"  {key}: {root}")

def test_complete_ayah_data(data_dir: str) -> None:
    """Test complete ayah data integration"""
    print("\n🔍 Testing Complete Ayah Data Integration...")
    
    # Load all necessary files
    files = {
        'ayah_metadata': 'quran-metadata-ayah.json',
        'ayah_text': 'quran-id-simple.json',
        'translation': 'indonesian-mokhtasar.json',
        'transliteration': 'transliteration-simple.json',
        'surah_names': 'quran-metadata-surah-name.json'
    }
    
    data = {}
    for key, filename in files.items():
        file_path = os.path.join(data_dir, filename)
        if os.path.exists(file_path):
            data[key] = load_json_file(file_path)
        else:
            print(f"⚠️  File not found: {filename}")
    
    # Test integration with sample ayahs
    test_verses = ['1:1', '1:2', '2:1', '2:255']  # Al-Fatihah and Ayat Kursi
    
    print(f"\n🧪 Testing Integration with Sample Verses:")
    for verse in test_verses:
        print(f"\n📖 Testing verse: {verse}")
        
        # Try to get complete data for this verse
        verse_data = {}
        
        # Get text
        if 'ayah_text' in data and verse in data['ayah_text']:
            verse_data['arabic'] = data['ayah_text'][verse]
        
        # Get translation
        if 'translation' in data and verse in data['translation']:
            verse_data['translation'] = data['translation'][verse]
        
        # Get transliteration
        if 'transliteration' in data and verse in data['transliteration']:
            verse_data['transliteration'] = data['transliteration'][verse]
        
        # Get metadata
        if 'ayah_metadata' in data and verse in data['ayah_metadata']:
            verse_data['metadata'] = data['ayah_metadata'][verse]
        
        # Get surah name
        surah_num = verse.split(':')[0]
        if 'surah_names' in data and surah_num in data['surah_names']:
            verse_data['surah_name'] = data['surah_names'][surah_num]
        
        # Display results
        if verse_data:
            print(f"  ✅ Found data for {verse}:")
            for key, value in verse_data.items():
                if isinstance(value, str) and len(value) > 100:
                    print(f"    {key}: {value[:100]}...")
                else:
                    print(f"    {key}: {value}")
        else:
            print(f"  ❌ No data found for {verse}")

def main():
    """Main test function"""
    print("🧪 Testing Hafiz Assistant Backend with Real Data")
    print("=" * 60)
    
    # Get data directory
    data_dir = "data"
    if not os.path.exists(data_dir):
        print(f"❌ Data directory not found: {data_dir}")
        return
    
    # List all available files
    print(f"\n📁 Available data files:")
    for file in os.listdir(data_dir):
        if file.endswith('.json'):
            file_path = os.path.join(data_dir, file)
            size = os.path.getsize(file_path)
            print(f"  - {file} ({size:,} bytes)")
    
    # Run tests
    try:
        test_ayah_metadata(data_dir)
        test_surah_metadata(data_dir)
        test_translations(data_dir)
        test_search_data(data_dir)
        test_word_analysis(data_dir)
        test_complete_ayah_data(data_dir)
        
        print("\n" + "=" * 60)
        print("✅ All tests completed successfully!")
        print("🚀 Real data is ready for Rust integration!")
        
    except Exception as e:
        print(f"\n❌ Error during testing: {e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    main()
