# Daily Coding Problem: 37
# Difficulty: Easy
# Asked by: Google
# Author: TenthEdict

def power_set(arr):
    n = len(arr)
    result = []

    for i in range(1 << n):
        subset = []

        for j in range(n):
            if i & (1 << j) != 0 : 
                subset.append(arr[j])
       
        result.append(subset)
    
    return result

print(power_set([1, 2, 3]))
print(power_set([1, 2]))
# print(power_set([]))