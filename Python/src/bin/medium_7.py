# Daily Coding Problem: 7
# Difficulty: Medium
# Asked by: Facebook
# Authored by: TenthEdict

def count_decodable(s):
    n = len(s)
    dp = [0] * (n + 1)
    
    dp[0] = 1
    
    for i in range(1, n + 1):
        # Check single digit
        if s[i-1] != '0':
            dp[i] += dp[i-1]
        
        # Check two digits
        if i >= 2:
            two_digit = int(s[i-2:i])
            if two_digit >= 10 and two_digit <= 26:
                dp[i] += dp[i-2]
    return dp[n]

print(count_decodable("111"))   # expect 3
print(count_decodable("12"))    # expect 2
print(count_decodable("27"))    # expect 1
print(count_decodable("1111"))  # expect 5
