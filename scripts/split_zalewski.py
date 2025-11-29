#!/usr/bin/env python3
"""
Split pdfrip.txt (Zalewski's Enochian Chess) into organized markdown files.
"""

import re
import os

INPUT_FILE = "/home/tom/Code/enoch/pdfrip.txt"
OUTPUT_DIR = "/home/tom/Code/enoch/docs/zalewski"

# Section definitions: (start_line, output_path, title)
# Lines are 1-indexed to match grep output
SECTIONS = [
    # Front matter
    (457, "00-front-matter/foreword.md", "Foreword"),
    (540, "00-front-matter/preface.md", "Preface"),
    (773, "00-front-matter/how-to-use.md", "How to Use This Book"),

    # Part I - Book of Earth (lines 825-2499)
    (825, "part1-earth/00-introduction.md", "Part I: Book of Earth - Introduction"),
    (998, "part1-earth/01-chaturanga.md", "Chaturanga"),
    (1284, "part1-earth/02-constructing-boards.md", "Constructing Your Chessboards"),
    (1705, "part1-earth/03-alternative-method.md", "Alternative Method of Constructing Your Chess Set"),
    (1912, "part1-earth/04-coloring.md", "Coloring and Descriptions of the Chess Pieces"),
    (2375, "part1-earth/05-ptah.md", "Ptah"),

    # Part II - Book of Air (lines 2500-6611)
    (2500, "part2-air/00-rules-intro.md", "Part II: Book of Air - The Game"),
    (2859, "part2-air/01-official-rules.md", "Official Rules for Enochian Chess"),
    (3100, "part2-air/02-positioning.md", "Positioning of the Chess Pieces"),
    (3255, "part2-air/03-which-board.md", "Which Board to Use?"),
    (3567, "part2-air/04-strategy.md", "Game Strategy"),
    (3811, "part2-air/05-openings.md", "Openings"),
    (3835, "part2-air/06-earth-fire-settings.md", "Earth and Fire Board Play Settings"),
    (4591, "part2-air/07-air-water-settings.md", "Air and Water Board Play Settings"),
    (5319, "part2-air/08-middle-endgame.md", "Middle-Game and Endgame"),

    # Part III - Book of Water (lines 6612-10888)
    (6612, "part3-water/00-divination-intro.md", "Part III: Book of Water - Divination"),
    (6812, "part3-water/01-divination-basics.md", "Divination Basics"),
    (7081, "part3-water/02-elemental-boards.md", "The Elemental Boards"),
    (7464, "part3-water/03-piece-positions.md", "Piece Positions"),
    (7844, "part3-water/04-divinatory-meanings.md", "Divinatory Meanings of the Chess Pieces"),
    (8504, "part3-water/05-analyzing.md", "Analyzing a Divination Game"),
    (8914, "part3-water/06-timing.md", "Timing and Relationships"),
    (9605, "part3-water/07-example.md", "Divination Example"),

    # Part IV - Book of Fire (lines 10889-12411)
    (10889, "part4-fire/00-training-intro.md", "Part IV: Book of Fire - Training of the Adept"),
    (11207, "part4-fire/01-beginning.md", "And in the Beginning"),
    (11664, "part4-fire/02-ritual-magic.md", "Ritual Magic and the Chess Game"),
    (11800, "part4-fire/03-methodology.md", "Experimental Methodology"),
    (11876, "part4-fire/04-transformations.md", "Formulas of Transformation"),
    (12043, "part4-fire/05-evocation.md", "Formula of Evocation"),
    (12313, "part4-fire/06-visitors.md", "Interesting Visitors"),

    # Appendix & Bibliography
    (12412, "appendix/bibliography.md", "Bibliography"),
]

def clean_text(text):
    """Clean up PDF extraction artifacts."""
    # Remove code block markers from PDF extraction
    text = re.sub(r'^```\n?', '', text, flags=re.MULTILINE)
    text = re.sub(r'\n```$', '', text, flags=re.MULTILINE)
    text = re.sub(r'^```$', '', text, flags=re.MULTILINE)

    # Remove page headers like "96 Part II-Book of Air" or "Official Rules for Enochian Chess 91"
    text = re.sub(r'^\d+\s+Part\s+[IVX]+-Book of (Earth|Air|Water|Fire)\s*$', '', text, flags=re.MULTILINE)
    text = re.sub(r'^Part\s+[IVX]+-Book of (Earth|Air|Water|Fire)\s*$', '', text, flags=re.MULTILINE)
    text = re.sub(r'^Part\s+[IVX]+-Book of (Wafer)\s*$', '', text, flags=re.MULTILINE)  # OCR typo

    # Remove standalone page numbers
    text = re.sub(r'^\d{1,3}\s*$', '', text, flags=re.MULTILINE)

    # Remove repeated section headers with page numbers
    text = re.sub(r'^(Official Rules for Enochian Chess|Constructing Your Chessboards|The Enochian System of Magic|Middle-Game and Endgame|Game Strategy|Openings|Divination|Ritual Magic and the Chess Game)\s*\d*\s*$', '', text, flags=re.MULTILINE)

    # Remove page numbers at end of lines (like "Chess Pieces 45")
    text = re.sub(r'\s+\d{1,3}\s*$', '', text, flags=re.MULTILINE)

    # Fix common OCR issues
    text = text.replace('ofthe', 'of the')
    text = text.replace('tothe', 'to the')
    text = text.replace('leR', 'left')
    text = text.replace('Kzng', 'King')

    # Remove excessive blank lines
    text = re.sub(r'\n{4,}', '\n\n\n', text)
    text = re.sub(r'\n{3,}', '\n\n', text)

    return text.strip()

def main():
    # Read the entire file
    with open(INPUT_FILE, 'r', encoding='utf-8', errors='replace') as f:
        lines = f.readlines()

    total_lines = len(lines)
    print(f"Read {total_lines} lines from {INPUT_FILE}")

    # Sort sections by start line
    sorted_sections = sorted(SECTIONS, key=lambda x: x[0])

    # Process each section
    for i, (start_line, output_path, title) in enumerate(sorted_sections):
        # Determine end line (start of next section or end of file)
        if i + 1 < len(sorted_sections):
            end_line = sorted_sections[i + 1][0] - 1
        else:
            end_line = total_lines

        # Adjust for 0-indexing
        start_idx = start_line - 1
        end_idx = end_line

        # Extract content
        content_lines = lines[start_idx:end_idx]
        content = ''.join(content_lines)

        # Clean up the content
        content = clean_text(content)

        # Create markdown with frontmatter
        markdown = f"""---
title: "{title}"
---

{content}
"""

        # Write to file
        output_file = os.path.join(OUTPUT_DIR, output_path)
        os.makedirs(os.path.dirname(output_file), exist_ok=True)

        with open(output_file, 'w', encoding='utf-8') as f:
            f.write(markdown)

        print(f"Created: {output_path} ({end_line - start_line + 1} lines)")

    print(f"\nDone! Created {len(SECTIONS)} files in {OUTPUT_DIR}")

if __name__ == "__main__":
    main()
