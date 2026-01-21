# Daily Coding Problem: 108
# Difficulty: Easy
# Asked by: Google
# Authored by: TenthEdict

def check_string_rotation(a, b):
    if len(a) != len(b):
        return False
    return b in a + a
    
print(check_string_rotation("abcde", "cdeab")) # True
print(check_string_rotation("abcde", "abcde")) # True
print(check_string_rotation("abc", "acb")) # False
print(check_string_rotation("abc", "abcd")) # False
print(check_string_rotation("","")) # True
print(check_string_rotation("a", "a")) # True
