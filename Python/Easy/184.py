# Daily Coding Problem: 184
# Difficulty: Easy
# Asked by: Amazon
# Authored by: TenthEdict

def gcd(a, b):
    r = a % b
    if r == 0:
        return b
    else:
        return gcd(b, r)

def gcd_of_array(arr):
    result = arr[0]
    for i in range(len(arr)):
        result = gcd(result, arr[i])
    return result

def main():
    numbers = [42,56,14]
    print(gcd_of_array(numbers))

main()