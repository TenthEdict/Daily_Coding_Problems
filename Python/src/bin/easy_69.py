# Daily Coding Problem: 69
# Difficulty: Easy
# Asked by: Facebook
# Authored by: TenthEdict

# Question: Given a list of integers, return the largest product that can be made by multiplying any three integers.

def largest_product_from_three_ints_Onlogn(arr):
    # step 1
    arr.sort()
    two_smallest = arr[:2]
    three_largest = arr[-3:]

    # step 2
    two_smallest_product = two_smallest[0] * two_smallest[1] * arr[-1]
    three_largest_product = three_largest[0] * three_largest[1] * three_largest[2]

    # return the largest product with max of the two products
    if two_smallest_product > three_largest_product:
        return two_smallest_product
    else:
        return three_largest_product
    
print(largest_product_from_three_ints_Onlogn([-10,5,2,-10,3]))        # Expected: 500
print(largest_product_from_three_ints_Onlogn([1, 2, 3, 4, 5]))        # Expected: 60
print(largest_product_from_three_ints_Onlogn([-5, -4, -3, -2, -1]))   # Expected: -6
print(largest_product_from_three_ints_Onlogn([-100, 1, 2, 3]))        # Expected: 6
print(largest_product_from_three_ints_Onlogn([-5, -4, 0, 3, 2]))      # Expected: 60

def largest_product_from_three_ints_On(arr):
    if len(arr) < 3:
        print("Array not long enough!")
        return None
    
    # Initialize tracking variables
    small_1, small_2 = float('inf'), float('inf')
    large_1, large_2, large_3 = float('-inf'), float('-inf'), float('-inf')

    for num in arr:
        if num > large_1:
            large_3 = large_2
            large_2 = large_1
            large_1 = num
        elif num > large_2:
            large_3 = large_2
            large_2 = num
        elif num > large_3:
            large_3 = num

        if num < small_1:
            small_2 = small_1
            small_1 = num
        elif num < small_2:
            small_2 = num
    
    # Compute both candidates and return max
    two_smallest_product = small_1 * small_2 * large_1
    three_largest_product = large_1 * large_2 * large_3
    if two_smallest_product > three_largest_product:
       return two_smallest_product
    else:
      return three_largest_product
    
print(largest_product_from_three_ints_On([1, 2, 3, 4, 5]))        # Expected: 60
print(largest_product_from_three_ints_On([-5, -4, -3, -2, -1]))   # Expected: -6
print(largest_product_from_three_ints_On([-100, 1, 2, 3]))        # Expected: 6
print(largest_product_from_three_ints_On([-5, -4, 0, 3, 2]))      # Expected: 60
print(largest_product_from_three_ints_On([-10, -10, 2, 3, 5]))    # Expected: 500
print(largest_product_from_three_ints_On([1, 2]))                 # Expected: None
