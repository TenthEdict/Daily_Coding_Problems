# Daily Coding Problem: 47
# Difficulty: Easy
# Asked by: Amazon
# Author: TenthEdict

def best_profit(arr):
    if len(arr) < 2:
        return 0

    min_price = arr[0]
    profit = 0

    for price in arr:
        if price < min_price:
            min_price = price
        profit = max(profit, price - min_price)
    
    return profit

def main():
    print(best_profit([9, 11, 8, 5, 7, 10]))
    print(best_profit([10, 2, 11, 1, 5]))
    print(best_profit([10, 8, 5, 2]))
    print(best_profit([5]))
    print(best_profit([]))

main()