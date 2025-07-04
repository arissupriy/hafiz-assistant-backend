#!/usr/bin/env python3
"""
Test loading data untuk memastikan file bisa dibaca
"""

import json
import os

def test_load_files():
    """Test loading semua file yang diperlukan"""
    files_to_test = [
        "quran-metadata-ayah.json",
        "quran-id-simple.json", 
        "indonesian-mokhtasar.json",
        "transliteration-simple.json",
        "surah-info-id.json",
        "matching-ayah.json",
        "qpc-v2-15-lines.db.json"
    ]
    
    for filename in files_to_test:
        filepath = f"data/{filename}"
        print(f"🔍 Testing {filename}...")
        
        if not os.path.exists(filepath):
            print(f"  ❌ File not found: {filepath}")
            continue
        
        try:
            with open(filepath, 'r', encoding='utf-8') as f:
                data = json.load(f)
            
            print(f"  ✅ Successfully loaded")
            print(f"     Type: {type(data)}")
            
            if isinstance(data, dict):
                print(f"     Keys: {len(data)} items")
                if data:
                    first_key = list(data.keys())[0]
                    print(f"     First key: {first_key}")
            elif isinstance(data, list):
                print(f"     Length: {len(data)} items")
                
        except json.JSONDecodeError as e:
            print(f"  ❌ JSON parse error: {e}")
        except Exception as e:
            print(f"  ❌ Error: {e}")

def test_ayah_metadata_parsing():
    """Test parsing struktur ayah metadata sesuai dengan Rust"""
    filepath = "data/quran-metadata-ayah.json"
    
    if not os.path.exists(filepath):
        print(f"❌ File {filepath} not found")
        return
    
    print(f"\n🦀 Testing Rust-compatible parsing for {filepath}")
    
    with open(filepath, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    if isinstance(data, dict):
        # Test parsing beberapa entries
        count = 0
        success_count = 0
        
        for key, verse_data in data.items():
            count += 1
            if count > 5:  # Test 5 entries saja
                break
                
            print(f"\n  Testing entry {key}:")
            
            try:
                # Simulasi parsing sesuai dengan Rust struct
                ayah = {
                    'id': verse_data.get('id'),
                    'surah_number': verse_data.get('surah_number'),
                    'ayah_number': verse_data.get('ayah_number'),
                    'verse_key': verse_data.get('verse_key'),
                    'text': verse_data.get('text')
                }
                
                # Validasi semua field ada
                missing_fields = [k for k, v in ayah.items() if v is None]
                if missing_fields:
                    print(f"    ❌ Missing fields: {missing_fields}")
                else:
                    print(f"    ✅ All fields present")
                    print(f"       verse_key: {ayah['verse_key']}")
                    print(f"       text: {ayah['text'][:50]}...")
                    success_count += 1
                    
            except Exception as e:
                print(f"    ❌ Error: {e}")
        
        print(f"\n📊 Results: {success_count}/{count} entries parsed successfully")

if __name__ == "__main__":
    print("🧪 Testing Data Loading")
    print("=" * 30)
    
    test_load_files()
    test_ayah_metadata_parsing()
