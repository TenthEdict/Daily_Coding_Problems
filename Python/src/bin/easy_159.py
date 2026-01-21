# Daily Coding Problem: 159
# Difficulty: Easy
# Asked by: Google
# Authored by: TenthEdict

def first_recurring_character(s):
    seen = set()
    for char in s:
        if char in seen:
            return char
        seen.add(char)
    return None

print(first_recurring_character("acbbac"))  # Output: 'b'
print(first_recurring_character("abcdef"))  # Output: None  
