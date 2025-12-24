# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

This is a **Daily Coding Problem** practice repository for solving algorithm and data structure problems. Problems are sourced from the Daily Coding Problem email newsletter and organized by difficulty level.

## Directory Structure

```
Daily Coding Problems/
├── Easy/           # Easy difficulty solutions (e.g., 69.py, 1110.py)
├── Emails/Import/  # mbox files from Daily Coding Problem emails
├── parse_mbox_to_excel.py  # Utility to extract problems from email archive
├── daily_coding_problems.xlsx  # Generated spreadsheet tracking all problems
└── requirements.txt  # Python dependencies (openpyxl)
```

## Commands

### Run a solution
```bash
python3 Easy/69.py
```

### Parse email archive to Excel
```bash
python3 parse_mbox_to_excel.py
```

### Install dependencies
```bash
pip install -r requirements.txt
```

## Solution File Format

Each solution follows this header pattern:
```python
# Daily Coding Problem: <number>
# Difficulty: Easy|Medium|Hard
# Asked by: <Company>
# Authored by: TenthEdict

# Question: <problem description>

def solution_function(...):
    ...

# Test cases at bottom with expected output comments
print(solution_function(...))  # Expected: ...
```

## Conventions

- Solutions are organized in folders by difficulty: `Easy/`, `Medium/`, `Hard/`
- File names are the problem number (e.g., `1110.py` for Problem #1110)
- Each file is self-contained with test cases included
- Multiple solution approaches may exist in one file (e.g., O(n log n) vs O(n) implementations)

## Email Parsing

The `parse_mbox_to_excel.py` script extracts problems from exported Gmail mbox files:
- Input: `Emails/Import/DCP.mbox/mbox`
- Output: `daily_coding_problems.xlsx`
- Extracts: problem number, difficulty, company, description, email date
- Auto-categorizes: Data Structures and Algorithm Types based on keyword matching
- Tracks: status (Not Started by default), date attempted, date solved

**Incremental updates:** Run the script anytime to add new problems. It will:
1. Load existing spreadsheet and preserve all user-modified data (status, dates, etc.)
2. Parse mbox and skip problems already in the spreadsheet
3. Auto-categorize new problems and append them
4. Re-sort by problem number
