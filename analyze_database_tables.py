#!/usr/bin/env python3
"""
Script untuk menganalisis tabel database dalam qpc-v2-15-lines.db.json
"""

import json
from pathlib import Path

def analyze_database_tables():
    """Analisis tabel-tabel dalam database"""
    
    data_path = Path("data/qpc-v2-15-lines.db.json")
    
    try:
        with open(data_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        print("=== ANALISIS TABEL DATABASE ===")
        
        if 'objects' in data and isinstance(data['objects'], list):
            objects = data['objects']
            
            for i, table in enumerate(objects):
                print(f"\n=== TABEL {i+1}: {table.get('name', 'Unknown')} ===")
                print(f"Type: {table.get('type')}")
                print(f"DDL: {table.get('ddl')}")
                
                if 'columns' in table:
                    columns = table['columns']
                    print(f"Columns ({len(columns)}):")
                    for col in columns:
                        print(f"  - {col['name']}: {col['type']}")
                
                if 'rows' in table:
                    rows = table['rows']
                    print(f"Rows: {len(rows)}")
                    
                    # Tampilkan beberapa row pertama
                    for j, row in enumerate(rows[:5]):
                        print(f"  Row {j+1}: {row}")
                    
                    if len(rows) > 5:
                        print(f"  ... dan {len(rows) - 5} row lainnya")
                
                # Jika ini tabel halaman, analisis lebih detail
                if 'name' in table and 'line' in table['name'].lower():
                    print(f"\n=== ANALISIS DETAIL TABEL {table['name']} ===")
                    
                    if 'columns' in table and 'rows' in table:
                        columns = table['columns']
                        rows = table['rows']
                        
                        # Buat mapping column name -> index
                        col_map = {col['name']: i for i, col in enumerate(columns)}
                        
                        # Analisis data halaman
                        page_data = {}
                        for row in rows:
                            if 'page' in col_map:
                                page_num = row[col_map['page']]
                                if page_num not in page_data:
                                    page_data[page_num] = []
                                page_data[page_num].append(row)
                        
                        print(f"Halaman ditemukan: {sorted(page_data.keys())[:10]}...")
                        
                        # Analisis halaman pertama
                        first_page = min(page_data.keys())
                        print(f"\nAnalisis halaman {first_page}:")
                        print(f"Jumlah baris: {len(page_data[first_page])}")
                        
                        for row in page_data[first_page][:3]:
                            print(f"  {row}")
    
    except Exception as e:
        print(f"Error: {e}")

def create_page_structure_map():
    """Buat mapping struktur halaman"""
    
    data_path = Path("data/qpc-v2-15-lines.db.json")
    
    try:
        with open(data_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        print("\n=== STRUKTUR HALAMAN YANG DIBUTUHKAN ===")
        
        if 'objects' in data and isinstance(data['objects'], list):
            objects = data['objects']
            
            # Cari tabel yang berisi data halaman
            page_table = None
            for table in objects:
                if 'name' in table and ('line' in table['name'].lower() or 'page' in table['name'].lower()):
                    page_table = table
                    break
            
            if page_table:
                print(f"Tabel halaman: {page_table['name']}")
                columns = page_table['columns']
                rows = page_table['rows']
                
                # Buat mapping column
                col_map = {col['name']: i for i, col in enumerate(columns)}
                print(f"Kolom tersedia: {list(col_map.keys())}")
                
                # Buat struktur RenderedPage
                print("\n=== MAPPING KE RENDERED PAGE ===")
                print("Struktur yang dibutuhkan:")
                print("- page_number: u16")
                print("- lines: Vec<PageLine>")
                print("- surah_headers: Vec<SurahHeader>")
                
                print("\nPageLine struktur:")
                print("- line_number: u8")
                print("- line_type: String")
                print("- text: String")
                print("- is_centered: bool")
                print("- verse_keys: Vec<String>")
                
                # Cek mapping yang mungkin
                possible_mappings = {
                    'page_number': [col for col in col_map.keys() if 'page' in col.lower()],
                    'line_number': [col for col in col_map.keys() if 'line' in col.lower()],
                    'text': [col for col in col_map.keys() if 'text' in col.lower()],
                    'verse_key': [col for col in col_map.keys() if 'verse' in col.lower() or 'ayah' in col.lower()],
                    'surah': [col for col in col_map.keys() if 'surah' in col.lower()],
                }
                
                print("\nPossible mappings:")
                for field, candidates in possible_mappings.items():
                    if candidates:
                        print(f"  {field}: {candidates}")
                
                # Sampel data untuk melihat pattern
                print(f"\nSampel data (3 row pertama):")
                for i, row in enumerate(rows[:3]):
                    print(f"  Row {i+1}:")
                    for j, (col_name, col_idx) in enumerate(col_map.items()):
                        if j < len(row):
                            print(f"    {col_name}: {row[col_idx]}")
    
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    analyze_database_tables()
    create_page_structure_map()
