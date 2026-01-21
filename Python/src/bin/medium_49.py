# Daily Coding Problem: 49
# Difficulty: Medium
# Asked by: Amazon
# Authored by: TenthEdict

def max_subarray_sum(arr):
    if len(arr) == 0:
        return None
    
    current_sum = 0
    max_sum = 0

    for i in arr:
        current_sum = current_sum + i
        max_sum = max(max_sum, current_sum)
        if current_sum < 0:
            current_sum = 0

    return max_sum

def main():
    print(max_subarray_sum([34,-50,42,14,-5,86]))
    print(max_subarray_sum([-5,-1,-8,-9]))
    print(max_subarray_sum([1,2,3,4]))
    print(max_subarray_sum([]))
    print(max_subarray_sum([-2,1,-2,2,-1,2]))

main()
