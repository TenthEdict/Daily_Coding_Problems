# Daily Coding Problem: 189
# Difficulty: Easy
# Asked by: Google
# Authored by: TenthEdict

def longest_distinct_subarray(arr):
    positions = {}
    beginning = 0
    result = 0

    for i in range(len(arr)):
        element = arr[i]

        if element in positions and positions[element] >= beginning:
            beginning = positions[element] + 1

        positions[element] = i

        result = max(result, i - beginning + 1)

    return result

print(longest_distinct_subarray([1,2,3,2,4, 5])) # returns 4
print(longest_distinct_subarray([5, 1, 3, 5, 2, 3, 4, 1])) # returns 5
print(longest_distinct_subarray([1,1,1,1])) # returns 1
print(longest_distinct_subarray([1,2,3,4,5])) # returns 5
print(longest_distinct_subarray([5,4,3,2,1,1])) # returns 5
print(longest_distinct_subarray([])) # returns 0
