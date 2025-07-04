#!/usr/bin/env python3
"""Debug script untuk mengecek data ayat dan similarity search"""

import json
import os

def load_json_file(filename):
    """Load JSON file"""
    filepath = os.path.join('./data', filename)
    if not os.path.exists(filepath):
        print(f"❌ File {filename} tidak ditemukan!")
        return None
    
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            return json.load(f)
    except Exception as e:
        print(f"❌ Error loading {filename}: {e}")
        return None

def analyze_ayah_data():
    """Analyze ayah data for similarity search"""
    print("🔍 Debug Similarity Search")
    print("=" * 40)
    
    # Load ayah data
    ayah_data = load_json_file('quran-metadata-ayah.json')
    if not ayah_data:
        return
    
    # Check structure
    print(f"📊 Total ayahs in metadata: {len(ayah_data)}")
    
    # Check if 2:255 exists
    verse_key = "2:255"
    if verse_key in ayah_data:
        print(f"✅ Found {verse_key}")
        print(f"   Data: {ayah_data[verse_key]}")
    else:
        print(f"❌ {verse_key} not found")
        # Show some keys
        keys = list(ayah_data.keys())[:10]
        print(f"   Available keys (first 10): {keys}")
    
    # Load Quran text
    simple_data = load_json_file('quran-id-simple.json')
    if not simple_data:
        return
    
    print(f"📊 Quran text structure type: {type(simple_data)}")
    
    if isinstance(simple_data, list):
        print(f"📊 Total surahs: {len(simple_data)}")
        
        # Check Al-Baqarah (surah 2)
        if len(simple_data) >= 2:
            surah_2 = simple_data[1]  # Index 1 for surah 2
            print(f"📖 Surah 2 info:")
            print(f"   Type: {type(surah_2)}")
            if isinstance(surah_2, dict):
                print(f"   Keys: {list(surah_2.keys())}")
                if 'verses' in surah_2:
                    verses = surah_2['verses']
                    print(f"   Total verses: {len(verses)}")
                    if len(verses) >= 255:
                        verse_255 = verses[254]  # Index 254 for verse 255
                        print(f"   Verse 255: {verse_255[:100]}...")
                elif 'ayahs' in surah_2:
                    ayahs = surah_2['ayahs']
                    print(f"   Total ayahs: {len(ayahs)}")
                    if len(ayahs) >= 255:
                        ayah_255 = ayahs[254]  # Index 254 for ayah 255
                        print(f"   Ayah 255: {ayah_255[:100]}...")
    
    # Test simple similarity
    print("\n🔍 Testing simple similarity logic:")
    
    # Sample texts for similarity test
    text1 = "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ"
    text2 = "الْحَمْدُ لِلَّهِ رَبِّ الْعَالَمِينَ"
    text3 = "بِسْمِ اللَّهِ الرَّحْمَٰنِ"  # Similar to text1
    
    def calculate_similarity(words1, words2):
        set1 = set(words1)
        set2 = set(words2)
        intersection = len(set1.intersection(set2))
        union = len(set1.union(set2))
        return intersection / union if union > 0 else 0.0
    
    words1 = text1.split()
    words2 = text2.split()
    words3 = text3.split()
    
    sim1_2 = calculate_similarity(words1, words2)
    sim1_3 = calculate_similarity(words1, words3)
    
    print(f"   Text 1: {text1}")
    print(f"   Text 2: {text2}")
    print(f"   Text 3: {text3}")
    print(f"   Similarity 1-2: {sim1_2:.3f}")
    print(f"   Similarity 1-3: {sim1_3:.3f}")

if __name__ == "__main__":
    analyze_ayah_data()
