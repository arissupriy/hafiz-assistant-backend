#!/usr/bin/env python3
"""
Debug script untuk struktur data JSON Quran
"""

import json
import os

def debug_quran_metadata():
    """Debug file quran-metadata-ayah.json"""
    file_path = "data/quran-metadata-ayah.json"
    
    if not os.path.exists(file_path):
        print(f"❌ File {file_path} tidak ditemukan")
        return
    
    with open(file_path, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    print(f"📄 File: {file_path}")
    print(f"📊 Type: {type(data)}")
    
    if isinstance(data, dict):
        print(f"🔑 Total keys: {len(data)}")
        
        # Ambil 3 entry pertama untuk analisis
        first_keys = list(data.keys())[:3]
        print(f"🔍 First 3 keys: {first_keys}")
        
        for key in first_keys:
            item = data[key]
            print(f"\n📋 Key '{key}':")
            print(f"   Type: {type(item)}")
            
            if isinstance(item, dict):
                print(f"   Fields: {list(item.keys())}")
                for field, value in item.items():
                    print(f"     {field}: {type(value).__name__} = {str(value)[:50]}...")

def debug_quran_simple():
    """Debug file quran-id-simple.json"""
    file_path = "data/quran-id-simple.json"
    
    if not os.path.exists(file_path):
        print(f"❌ File {file_path} tidak ditemukan")
        return
    
    with open(file_path, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    print(f"\n📄 File: {file_path}")
    print(f"📊 Type: {type(data)}")
    
    if isinstance(data, dict):
        print(f"🔑 Total keys: {len(data)}")
        
        # Ambil 5 entry pertama
        first_keys = list(data.keys())[:5]
        print(f"🔍 First 5 keys: {first_keys}")
        
        for key in first_keys:
            value = data[key]
            print(f"   {key}: {type(value).__name__} = {str(value)[:100]}...")

def generate_rust_ayah_metadata():
    """Generate Rust struct berdasarkan struktur aktual"""
    file_path = "data/quran-metadata-ayah.json"
    
    if not os.path.exists(file_path):
        print(f"❌ File {file_path} tidak ditemukan")
        return
    
    with open(file_path, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    if isinstance(data, dict) and data:
        first_key = list(data.keys())[0]
        first_item = data[first_key]
        
        if isinstance(first_item, dict):
            print(f"\n🦀 Suggested Rust AyahMetadata struct:")
            print("#[derive(Debug, Clone, Serialize, Deserialize)]")
            print("pub struct AyahMetadata {")
            
            for field, value in first_item.items():
                rust_type = get_rust_type(value)
                print(f"    pub {field}: {rust_type},")
            
            print("}")

def get_rust_type(value):
    """Convert Python type to appropriate Rust type"""
    if value is None:
        return "Option<String>"
    elif isinstance(value, str):
        return "String"
    elif isinstance(value, int):
        if value < 256:
            return "u8"
        elif value < 65536:
            return "u16"
        else:
            return "u32"
    elif isinstance(value, float):
        return "f64"
    elif isinstance(value, bool):
        return "bool"
    elif isinstance(value, list):
        return "Vec<String>"
    elif isinstance(value, dict):
        return "serde_json::Value"  # For complex nested objects
    else:
        return "String"

def main():
    print("🔍 Debugging Quran Data Structure")
    print("=" * 40)
    
    debug_quran_metadata()
    debug_quran_simple()
    generate_rust_ayah_metadata()

if __name__ == "__main__":
    main()
