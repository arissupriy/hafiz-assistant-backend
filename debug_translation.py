#!/usr/bin/env python3
"""
Debug translation integration in the Rust backend
"""

import json
from pathlib import Path

def debug_translation_integration():
    """Debug why translation isn't showing in the output"""
    
    # Check translation file
    translation_file = Path("data/indonesian-mokhtasar.json")
    if translation_file.exists():
        print("✅ Translation file exists")
        
        with open(translation_file, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        print(f"Translation data structure: {type(data)}")
        print(f"Total verses in translation: {len(data)}")
        
        # Check if we have translations for the verses we're testing
        test_verses = ["1:1", "2:255", "3:2", "42:4"]
        print(f"\n=== TRANSLATIONS FOR TEST VERSES ===")
        
        for verse_key in test_verses:
            if verse_key in data:
                text = data[verse_key]['text'] if isinstance(data[verse_key], dict) else str(data[verse_key])
                print(f"✅ {verse_key}: {text[:100]}...")
            else:
                print(f"❌ {verse_key}: NOT FOUND")
    else:
        print("❌ Translation file not found!")
    
    # Check transliteration file
    print("\n" + "="*50)
    transliteration_file = Path("data/transliteration-simple.json")
    if transliteration_file.exists():
        print("✅ Transliteration file exists")
        
        with open(transliteration_file, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        print(f"Transliteration data structure: {type(data)}")
        
        if 'verses' in data:
            verses = data['verses']
            print(f"Total verses in transliteration: {len(verses)}")
            
            # Check structure of first few verses
            print("\n=== FIRST 3 TRANSLITERATION ENTRIES ===")
            for i, verse in enumerate(verses[:3]):
                print(f"Verse {i+1}: {verse}")
            
            # Check if we have transliterations for the verses we're testing
            test_verses = ["1:1", "2:255", "3:2", "42:4"]
            print(f"\n=== TRANSLITERATIONS FOR TEST VERSES ===")
            
            verse_dict = {v['verse_key']: v['text'] for v in verses}
            
            for verse_key in test_verses:
                if verse_key in verse_dict:
                    print(f"✅ {verse_key}: {verse_dict[verse_key][:100]}...")
                else:
                    print(f"❌ {verse_key}: NOT FOUND")
        else:
            print("❌ No 'verses' key found in transliteration data")
            print(f"Available keys: {list(data.keys())[:10]}...")
    else:
        print("❌ Transliteration file not found!")

if __name__ == "__main__":
    debug_translation_integration()
