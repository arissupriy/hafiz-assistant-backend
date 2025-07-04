#!/usr/bin/env python3
"""
Script untuk menganalisis struktur file qpc-v2-15-lines.db.json
untuk implementasi load data per halaman
"""

import json
from pathlib import Path

def analyze_page_data_structure():
    """Analisis struktur data halaman Quran"""
    
    data_path = Path("data/qpc-v2-15-lines.db.json")
    if not data_path.exists():
        print(f"File {data_path} tidak ditemukan")
        return
    
    print("=== ANALISIS STRUKTUR DATA HALAMAN QURAN ===")
    
    try:
        with open(data_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        print(f"Tipe data: {type(data)}")
        
        if isinstance(data, dict):
            print(f"Jumlah key utama: {len(data)}")
            print(f"Key utama: {list(data.keys())[:10]}...")
            
            # Analisis key pertama
            first_key = list(data.keys())[0]
            first_value = data[first_key]
            print(f"\nContoh key pertama: {first_key}")
            print(f"Tipe value: {type(first_value)}")
            
            if isinstance(first_value, dict):
                print(f"Subkeys: {list(first_value.keys())}")
                # Analisis lebih dalam
                for subkey, subvalue in list(first_value.items())[:5]:
                    print(f"  {subkey}: {type(subvalue)}")
                    if isinstance(subvalue, list) and len(subvalue) > 0:
                        print(f"    List dengan {len(subvalue)} item")
                        print(f"    Item pertama: {subvalue[0]}")
                    elif isinstance(subvalue, dict):
                        print(f"    Dict dengan keys: {list(subvalue.keys())}")
                    else:
                        print(f"    Value: {subvalue}")
                        
        elif isinstance(data, list):
            print(f"Jumlah item: {len(data)}")
            if len(data) > 0:
                print(f"Item pertama: {data[0]}")
                print(f"Tipe item: {type(data[0])}")
                
                if isinstance(data[0], dict):
                    print(f"Keys item pertama: {list(data[0].keys())}")
    
    except Exception as e:
        print(f"Error membaca file: {e}")

def analyze_page_structure_detail():
    """Analisis detail struktur halaman"""
    
    data_path = Path("data/qpc-v2-15-lines.db.json")
    
    try:
        with open(data_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        print("\n=== ANALISIS DETAIL STRUKTUR HALAMAN ===")
        
        # Cari pola untuk page number
        page_keys = []
        for key in list(data.keys())[:20]:
            if key.isdigit():
                page_keys.append(int(key))
        
        if page_keys:
            print(f"Ditemukan page keys numerik: {sorted(page_keys)[:10]}...")
            
            # Analisis halaman pertama
            first_page = str(min(page_keys))
            page_data = data[first_page]
            
            print(f"\nAnalisis halaman {first_page}:")
            print(f"Struktur: {type(page_data)}")
            
            if isinstance(page_data, dict):
                for key, value in page_data.items():
                    print(f"  {key}: {type(value)}")
                    if isinstance(value, list):
                        print(f"    List dengan {len(value)} item")
                        if len(value) > 0:
                            print(f"    Item pertama: {value[0]}")
                            if isinstance(value[0], dict):
                                print(f"    Keys item: {list(value[0].keys())}")
                    elif isinstance(value, dict):
                        print(f"    Dict dengan keys: {list(value.keys())}")
                    else:
                        print(f"    Value: {str(value)[:100]}...")
            
            # Cek apakah ada info tentang lines
            if 'lines' in page_data:
                lines = page_data['lines']
                print(f"\nAnalisis lines (halaman {first_page}):")
                print(f"Jumlah lines: {len(lines)}")
                
                if len(lines) > 0:
                    print(f"Line pertama: {lines[0]}")
                    if isinstance(lines[0], dict):
                        print(f"Keys line: {list(lines[0].keys())}")
                
                # Analisis beberapa lines
                for i, line in enumerate(lines[:3]):
                    print(f"\nLine {i+1}: {line}")
    
    except Exception as e:
        print(f"Error analisis detail: {e}")

def find_page_layout_pattern():
    """Mencari pola layout halaman"""
    
    data_path = Path("data/qpc-v2-15-lines.db.json")
    
    try:
        with open(data_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        print("\n=== MENCARI POLA LAYOUT HALAMAN ===")
        
        # Cari pattern yang mungkin
        patterns = {
            'page_number': [],
            'line_number': [],
            'verse_key': [],
            'text': [],
            'surah_name': [],
            'juz_number': []
        }
        
        # Sampel beberapa halaman
        sample_pages = list(data.keys())[:5]
        
        for page_key in sample_pages:
            page_data = data[page_key]
            print(f"\nHalaman {page_key}:")
            
            # Cari semua keys yang ada di halaman
            all_keys = set()
            if isinstance(page_data, dict):
                all_keys.update(page_data.keys())
                
                # Cari di level yang lebih dalam
                for key, value in page_data.items():
                    if isinstance(value, list):
                        for item in value:
                            if isinstance(item, dict):
                                all_keys.update(item.keys())
                    elif isinstance(value, dict):
                        all_keys.update(value.keys())
            
            print(f"  Semua keys ditemukan: {sorted(all_keys)}")
            
            # Cari pattern yang kita butuhkan
            for pattern_name, pattern_list in patterns.items():
                for key in all_keys:
                    if pattern_name.lower() in key.lower():
                        pattern_list.append(key)
        
        print(f"\nPola yang ditemukan:")
        for pattern_name, found_keys in patterns.items():
            if found_keys:
                print(f"  {pattern_name}: {set(found_keys)}")
    
    except Exception as e:
        print(f"Error mencari pattern: {e}")

if __name__ == "__main__":
    analyze_page_data_structure()
    analyze_page_structure_detail()
    find_page_layout_pattern()
