# Daily Coding Problem: 81
# Difficulty: Easy
# Asked by: Yelp
# Author: TenthEdict

mapping = {
    "2": "abc", "3": "def", "4": "ghi", "5": "jkl",
    "6": "mno", "7": "pqrs", "8": "tuv", "9": "wxyz"
}


def letter_combinations(digits):
    result = []

    def helper(current, digits):
    # Base case: no more digits to process
        if not digits:
        # append current to result
            return result.append(current)
    
    # Recursive case:
        first_digit = digits[0]
        for letter in mapping[first_digit]:
            # recurse with updated current and remaining digits
            helper(current + letter, digits[1:])

    helper("", digits)

    return result

print(letter_combinations("23"))
# Expected: ['ad', 'ae', 'af', 'bd', 'be', 'bf', 'cd', 'ce', 'cf']

print(letter_combinations("7"))
# Expected: ['p', 'q', 'r', 's']

print(letter_combinations(""))
# What do you expect here?