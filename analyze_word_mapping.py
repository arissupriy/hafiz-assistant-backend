#!/usr/bin/env python3
"""
Script untuk membuat mapping word ID ke verse key
berdasarkan analisis data yang ada
"""

import json
from pathlib import Path

def analyze_word_to_verse_mapping():
    """Analisis mapping word ID ke verse"""
    
    # Baca data pages
    pages_path = Path("data/qpc-v2-15-lines.db.json")
    ayah_path = Path("data/quran-metadata-ayah.json")
    
    try:
        # Load page data
        with open(pages_path, 'r', encoding='utf-8') as f:
            page_data = json.load(f)
        
        # Load ayah data
        with open(ayah_path, 'r', encoding='utf-8') as f:
            ayah_data = json.load(f)
        
        print("=== ANALISIS WORD TO VERSE MAPPING ===")
        
        # Extract page rows
        page_rows = []
        if 'objects' in page_data:
            for table in page_data['objects']:
                if table.get('name') == 'pages':
                    page_rows = table.get('rows', [])
                    break
        
        print(f"Total page rows: {len(page_rows)}")
        
        # Analisis pola word ID
        ayah_rows = [row for row in page_rows if row[2] == 'ayah' and row[4] and row[5]]
        print(f"Ayah rows: {len(ayah_rows)}")
        
        # Analisis beberapa row pertama untuk melihat pola
        print("\nSampel ayah rows:")
        for i, row in enumerate(ayah_rows[:10]):
            page_num, line_num, line_type, is_centered, first_word, last_word, surah_num = row
            word_count = int(last_word) - int(first_word) + 1 if first_word and last_word else 0
            print(f"  {i+1}: Page {page_num}, Line {line_num}, Words {first_word}-{last_word} ({word_count} words)")
        
        # Hitung total kata dan perkiraan mapping
        total_words = 0
        if ayah_rows:
            last_row = max(ayah_rows, key=lambda x: int(x[5]) if x[5] else 0)
            total_words = int(last_row[5]) if last_row[5] else 0
        
        print(f"\nTotal words estimated: {total_words}")
        
        # Buat estimasi sederhana berdasarkan urutan
        # Asumsi: word ID berurutan sesuai urutan ayat dalam Quran
        total_ayahs = len(ayah_data)
        print(f"Total ayahs: {total_ayahs}")
        
        if total_words > 0:
            avg_words_per_ayah = total_words / total_ayahs
            print(f"Average words per ayah: {avg_words_per_ayah:.2f}")
        
        # Analisis distribusi kata per halaman
        page_word_ranges = {}
        for row in ayah_rows:
            page_num = row[0]
            first_word = int(row[4]) if row[4] else 0
            last_word = int(row[5]) if row[5] else 0
            
            if page_num not in page_word_ranges:
                page_word_ranges[page_num] = []
            page_word_ranges[page_num].extend([first_word, last_word])
        
        # Analisis beberapa halaman pertama
        print(f"\nAnalisis word ranges per halaman:")
        for page_num in sorted(page_word_ranges.keys())[:5]:
            word_range = page_word_ranges[page_num]
            min_word = min(word_range)
            max_word = max(word_range)
            print(f"  Page {page_num}: Words {min_word}-{max_word}")
    
    except Exception as e:
        print(f"Error: {e}")

def create_simple_word_mapping_strategy():
    """Buat strategi mapping sederhana"""
    
    ayah_path = Path("data/quran-metadata-ayah.json")
    
    try:
        with open(ayah_path, 'r', encoding='utf-8') as f:
            ayah_data = json.load(f)
        
        print("\n=== STRATEGI MAPPING SEDERHANA ===")
        
        # Buat mapping berdasarkan asumsi urutan
        ayah_list = []
        for ayah_id, ayah_info in ayah_data.items():
            if isinstance(ayah_info, dict):
                ayah_list.append({
                    'id': ayah_info.get('id'),
                    'verse_key': ayah_info.get('verse_key'),
                    'text': ayah_info.get('text', ''),
                    'word_count': len(ayah_info.get('text', '').split())
                })
        
        # Sort by ID
        ayah_list.sort(key=lambda x: x['id'])
        
        # Assign word ranges
        current_word_id = 1
        word_to_verse_map = {}
        
        for ayah in ayah_list[:10]:  # Hanya 10 ayat pertama untuk contoh
            start_word = current_word_id
            end_word = current_word_id + ayah['word_count'] - 1
            
            print(f"Ayah {ayah['verse_key']}: Words {start_word}-{end_word} ({ayah['word_count']} words)")
            print(f"  Text: {ayah['text'][:50]}...")
            
            # Map each word ID to this verse
            for word_id in range(start_word, end_word + 1):
                word_to_verse_map[word_id] = ayah['verse_key']
            
            current_word_id = end_word + 1
        
        print(f"\nSample word to verse mapping:")
        for word_id in range(1, 21):
            if word_id in word_to_verse_map:
                print(f"  Word {word_id}: {word_to_verse_map[word_id]}")
    
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    analyze_word_to_verse_mapping()
    create_simple_word_mapping_strategy()
