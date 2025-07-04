#!/usr/bin/env python3
"""
Script untuk menganalisis struktur file qpc-v2-15-lines.db.json lebih detail
"""

import json
from pathlib import Path

def deep_analyze_page_data():
    """Analisis mendalam struktur data halaman"""
    
    data_path = Path("data/qpc-v2-15-lines.db.json")
    
    try:
        with open(data_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        print("=== ANALISIS MENDALAM STRUKTUR DATA HALAMAN ===")
        print(f"Root keys: {list(data.keys())}")
        print(f"Type: {data.get('type')}")
        print(f"Name: {data.get('name')}")
        
        # Analisis objects
        if 'objects' in data:
            objects = data['objects']
            print(f"\nObjects type: {type(objects)}")
            print(f"Objects length: {len(objects) if isinstance(objects, (list, dict)) else 'N/A'}")
            
            if isinstance(objects, dict):
                print(f"Objects keys: {list(objects.keys())[:10]}...")
                
                # Cari keys yang seperti page number
                page_keys = []
                for key in objects.keys():
                    if isinstance(key, str) and key.isdigit():
                        page_keys.append(int(key))
                
                if page_keys:
                    print(f"Page numbers found: {sorted(page_keys)[:10]}...")
                    
                    # Analisis halaman pertama
                    first_page = str(min(page_keys))
                    page_data = objects[first_page]
                    
                    print(f"\nAnalisis halaman {first_page}:")
                    print(f"Type: {type(page_data)}")
                    
                    if isinstance(page_data, dict):
                        print(f"Keys: {list(page_data.keys())}")
                        
                        # Analisis setiap key
                        for key, value in page_data.items():
                            print(f"\n  {key}:")
                            print(f"    Type: {type(value)}")
                            
                            if isinstance(value, list):
                                print(f"    Length: {len(value)}")
                                if len(value) > 0:
                                    print(f"    First item: {value[0]}")
                                    if isinstance(value[0], dict):
                                        print(f"    First item keys: {list(value[0].keys())}")
                            elif isinstance(value, dict):
                                print(f"    Dict keys: {list(value.keys())}")
                            else:
                                print(f"    Value: {str(value)[:100]}...")
                
                # Cek apakah ada pattern lain
                non_numeric_keys = [k for k in objects.keys() if not (isinstance(k, str) and k.isdigit())]
                if non_numeric_keys:
                    print(f"\nNon-numeric keys: {non_numeric_keys[:10]}...")
                    
                    # Analisis key non-numerik pertama
                    first_non_numeric = non_numeric_keys[0]
                    print(f"Analisis key '{first_non_numeric}': {objects[first_non_numeric]}")
            
            elif isinstance(objects, list):
                print(f"Objects is list with {len(objects)} items")
                if len(objects) > 0:
                    print(f"First item: {objects[0]}")
                    if isinstance(objects[0], dict):
                        print(f"First item keys: {list(objects[0].keys())}")
        
        # Cek apakah ada structure lain
        for key in data.keys():
            if key not in ['type', 'name', 'objects']:
                print(f"\nUnexpected key '{key}': {data[key]}")
    
    except Exception as e:
        print(f"Error: {e}")

def sample_page_content():
    """Sampel konten halaman"""
    
    data_path = Path("data/qpc-v2-15-lines.db.json")
    
    try:
        with open(data_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        print("\n=== SAMPEL KONTEN HALAMAN ===")
        
        if 'objects' in data and isinstance(data['objects'], dict):
            objects = data['objects']
            
            # Cari halaman pertama
            page_keys = [k for k in objects.keys() if isinstance(k, str) and k.isdigit()]
            if page_keys:
                page_keys = sorted([int(k) for k in page_keys])
                first_page = str(page_keys[0])
                
                print(f"Konten halaman {first_page}:")
                page_data = objects[first_page]
                
                # Print dengan format yang bagus
                print(json.dumps(page_data, indent=2, ensure_ascii=False)[:1000] + "...")
                
                # Cek apakah ada lines atau similar
                if isinstance(page_data, dict):
                    for key, value in page_data.items():
                        if 'line' in key.lower() or 'text' in key.lower():
                            print(f"\n{key} detail:")
                            if isinstance(value, list):
                                for i, item in enumerate(value[:3]):
                                    print(f"  {i+1}: {item}")
                            else:
                                print(f"  {value}")
    
    except Exception as e:
        print(f"Error sampling: {e}")

if __name__ == "__main__":
    deep_analyze_page_data()
    sample_page_content()
