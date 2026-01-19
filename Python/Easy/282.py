# Daily Coding Problem: 282
# Difficulty: Easy
# Asked by: Netflix
# Authored by: TenthEdict

# Question: Given an array of integers, determine whether it contains a Pythagorean triplet. Recall that a Pythogorean triplet (a, b, c) is defined by the equation a2+ b2= c 2.

def contains_pythagorean_triplet(arr):
    squared = set(x ** 2 for x in arr)

    for a in squared:
        for b in squared: 
            if a + b in squared:
                return True
    return False

print(contains_pythagorean_triplet([1,2,3])) # False
print(contains_pythagorean_triplet([5,12,13])) # True
print(contains_pythagorean_triplet([])) # False

