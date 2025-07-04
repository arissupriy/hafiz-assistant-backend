#!/usr/bin/env python3
"""
Analisis file surah-info-id.json
"""

import json
import os

def analyze_surah_info():
    filepath = "data/surah-info-id.json"
    
    if not os.path.exists(filepath):
        print(f"❌ File {filepath} not found")
        return
    
    with open(filepath, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    print(f"📄 File: {filepath}")
    print(f"📊 Type: {type(data)}")
    
    if isinstance(data, dict):
        print(f"🔑 Keys: {len(data)} items")
        first_keys = list(data.keys())[:3]
        print(f"🔍 First 3 keys: {first_keys}")
        
        for key in first_keys:
            item = data[key]
            print(f"\n📋 Key '{key}':")
            print(f"   Type: {type(item)}")
            if isinstance(item, dict):
                print(f"   Fields: {list(item.keys())}")
                for field, value in item.items():
                    print(f"     {field}: {type(value).__name__} = {str(value)[:80]}...")
    
    elif isinstance(data, list):
        print(f"📏 Length: {len(data)} items")
        
        if data:
            first_item = data[0]
            print(f"\n📋 First item:")
            print(f"   Type: {type(first_item)}")
            if isinstance(first_item, dict):
                print(f"   Fields: {list(first_item.keys())}")
                for field, value in first_item.items():
                    print(f"     {field}: {type(value).__name__} = {str(value)[:80]}...")
    
    # Generate Rust struct suggestion
    print(f"\n🦀 Suggested Rust SurahInfo struct:")
    
    if isinstance(data, list) and data:
        sample = data[0]
    elif isinstance(data, dict) and data:
        sample = list(data.values())[0]
    else:
        print("   No data to analyze")
        return
    
    print("#[derive(Debug, Clone, Serialize, Deserialize)]")
    print("pub struct SurahInfo {")
    
    for field, value in sample.items():
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
        return "serde_json::Value"
    else:
        return "String"

if __name__ == "__main__":
    analyze_surah_info()
