#!/usr/bin/env python3
"""
Script to analyze matching-ayah.json structure and find bidirectional patterns
"""

import json
import sys
from pathlib import Path

def analyze_matching_ayah_structure():
    """Analyze the structure of matching-ayah.json"""
    try:
        # Read the matching-ayah.json file
        data_path = Path("data/matching-ayah.json")
        if not data_path.exists():
            print(f"Error: {data_path} not found")
            return
        
        with open(data_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        print("=== MATCHING AYAH STRUCTURE ANALYSIS ===")
        print(f"Total entries: {len(data)}")
        
        # Check if it's a list or dict
        if isinstance(data, list):
            print("Data structure: List")
            if len(data) > 0:
                print(f"First entry structure: {list(data[0].keys())}")
                print(f"First entry example: {data[0]}")
                
                # Check for bidirectional patterns
                print("\n=== BIDIRECTIONAL ANALYSIS ===")
                verse_pairs = set()
                for entry in data:
                    if 'verse_key' in entry and 'matched_ayah_key' in entry:
                        verse_pairs.add((entry['verse_key'], entry['matched_ayah_key']))
                
                print(f"Total unique verse pairs: {len(verse_pairs)}")
                
                # Check if we have both directions
                bidirectional_count = 0
                for v1, v2 in verse_pairs:
                    if (v2, v1) in verse_pairs:
                        bidirectional_count += 1
                
                print(f"Bidirectional pairs: {bidirectional_count}")
                print(f"Unidirectional pairs: {len(verse_pairs) - bidirectional_count}")
                
                # Sample analysis
                print("\n=== SAMPLE ENTRIES ===")
                for i, entry in enumerate(data[:5]):
                    print(f"Entry {i+1}: {entry}")
                
        elif isinstance(data, dict):
            print("Data structure: Dictionary")
            print(f"Top-level keys: {list(data.keys())}")
            
            # If it's a nested structure, analyze further
            for key, value in list(data.items())[:3]:
                print(f"Key '{key}': {type(value)}")
                if isinstance(value, dict):
                    print(f"  Subkeys: {list(value.keys())}")
                elif isinstance(value, list) and len(value) > 0:
                    print(f"  List length: {len(value)}")
                    print(f"  First item: {value[0]}")
        
        return data
        
    except Exception as e:
        print(f"Error analyzing matching-ayah.json: {e}")
        return None

def find_similar_ayahs_for_verse(data, verse_key):
    """Find all similar ayahs for a given verse key (bidirectional)"""
    similar_ayahs = []
    
    if isinstance(data, dict):
        # Direct matches where verse_key is the source
        if verse_key in data:
            for match in data[verse_key]:
                similar_ayahs.append({
                    'verse_key': verse_key,
                    'matched_ayah_key': match['matched_ayah_key'],
                    'similarity_score': match.get('score', 0.0),
                    'matching_type': 'direct',
                    'matched_words_count': match.get('matched_words_count', 0),
                    'coverage': match.get('coverage', 0)
                })
        
        # Reverse matches where verse_key is the target
        for source_verse, matches in data.items():
            for match in matches:
                if match['matched_ayah_key'] == verse_key:
                    similar_ayahs.append({
                        'verse_key': verse_key,
                        'matched_ayah_key': source_verse,
                        'similarity_score': match.get('score', 0.0),
                        'matching_type': 'reverse',
                        'matched_words_count': match.get('matched_words_count', 0),
                        'coverage': match.get('coverage', 0)
                    })
    
    return similar_ayahs

def test_bidirectional_search():
    """Test bidirectional search functionality"""
    data = analyze_matching_ayah_structure()
    if not data:
        return
    
    print("\n=== BIDIRECTIONAL SEARCH TEST ===")
    
    # Test with a few verse keys
    test_verses = ["1:1", "1:2", "2:1", "2:255"]
    
    for verse_key in test_verses:
        similar = find_similar_ayahs_for_verse(data, verse_key)
        print(f"Verse {verse_key}: Found {len(similar)} similar ayahs")
        for entry in similar[:3]:  # Show first 3
            print(f"  -> {entry['matched_ayah_key']} (score: {entry.get('similarity_score', 'N/A')})")

if __name__ == "__main__":
    test_bidirectional_search()
