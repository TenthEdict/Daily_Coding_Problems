# Daily Coding Problem: 225
# Difficulty: Easy
# Asked by: Bloomberg
# Authored by: TenthEdict

# Time Complexity: O(n**2)
def josephus_On2(n, k):
    prisoners = list(range(1, n+1))
    index = k-1
    while len(prisoners) > 1:
        prisoners.pop(index)
        index = (index + k - 1) % len(prisoners)
    return prisoners[0]

print(josephus_On2(5,2)) # prints 3
print(josephus_On2(1,5)) # prints 1
print(josephus_On2(7,1)) # prints 7

# Time Complexity: O(log N)

def josephus_Ologn(n):
    p = 1
    while p * 2 <= n:
        p = p * 2
    index = 2 * (n - p) + 1
    return index 

print(josephus_Ologn(1)) # prints 1
print(josephus_Ologn(16)) # prints 1
print(josephus_Ologn(8)) # prints 1

for n in range(1, 17):
    slow = josephus_On2(n, 2)
    fast = josephus_Ologn(n)
    match = "✓" if slow == fast else "✗"
    print(f"n={n}: O(n²)={slow}, O(log n)={fast} {match}")

