#!/usr/bin/env python3
"""
Test script to verify the similar ayah search with specific examples
"""

import json
from pathlib import Path

def test_similar_ayah_bidirectional():
    """Test specific examples of bidirectional similar ayah search"""
    
    # Read the matching-ayah.json file
    data_path = Path("data/matching-ayah.json")
    with open(data_path, 'r', encoding='utf-8') as f:
        matching_data = json.load(f)
    
    print("=== BIDIRECTIONAL SIMILAR AYAH TEST ===")
    
    # Test case 1: Check 1:1 - should find direct matches
    verse_key = "1:1"
    print(f"\nTesting {verse_key}:")
    
    # Direct matches (where 1:1 is source)
    if verse_key in matching_data:
        print(f"  Direct matches: {len(matching_data[verse_key])}")
        for match in matching_data[verse_key][:3]:
            print(f"    -> {match['matched_ayah_key']} (score: {match['score']})")
    
    # Reverse matches (where 1:1 is target)
    reverse_count = 0
    for source_verse, matches in matching_data.items():
        for match in matches:
            if match['matched_ayah_key'] == verse_key:
                reverse_count += 1
                if reverse_count <= 3:
                    print(f"    <- {source_verse} (score: {match['score']})")
    
    print(f"  Reverse matches: {reverse_count}")
    
    # Test case 2: Check 2:255 - should find reverse matches
    verse_key = "2:255"
    print(f"\nTesting {verse_key}:")
    
    # Direct matches (where 2:255 is source)
    direct_count = 0
    if verse_key in matching_data:
        direct_count = len(matching_data[verse_key])
        print(f"  Direct matches: {direct_count}")
        for match in matching_data[verse_key][:3]:
            print(f"    -> {match['matched_ayah_key']} (score: {match['score']})")
    
    # Reverse matches (where 2:255 is target)
    reverse_count = 0
    for source_verse, matches in matching_data.items():
        for match in matches:
            if match['matched_ayah_key'] == verse_key:
                reverse_count += 1
                if reverse_count <= 3:
                    print(f"    <- {source_verse} (score: {match['score']})")
    
    print(f"  Reverse matches: {reverse_count}")
    
    # Test case 3: Check 27:30 - should be connected to 1:1
    verse_key = "27:30"
    print(f"\nTesting {verse_key}:")
    
    # Check if 27:30 connects to 1:1
    found_connection = False
    if verse_key in matching_data:
        for match in matching_data[verse_key]:
            if match['matched_ayah_key'] == "1:1":
                print(f"  Direct connection to 1:1 (score: {match['score']})")
                found_connection = True
    
    # Check reverse connection
    if "1:1" in matching_data:
        for match in matching_data["1:1"]:
            if match['matched_ayah_key'] == verse_key:
                print(f"  Reverse connection from 1:1 (score: {match['score']})")
                found_connection = True
    
    if not found_connection:
        print("  No connection found with 1:1")

if __name__ == "__main__":
    test_similar_ayah_bidirectional()
