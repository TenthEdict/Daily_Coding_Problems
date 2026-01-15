// Daily Coding Problem: 47
// Difficulty: Easy
// Asked by: Amazon
// Authored by: TenthEdict

#include <stdio.h>

int best_profit(int *prices, int length) {
    if (length < 2) {
        return 0;
    }

    int profit = 0;
    int min_price = prices[0];

    for (int i = 1; i < length; i++) {
        if (prices[i] < min_price) {
            min_price = prices[i];
        }
        profit = (profit > prices[i] - min_price) ? profit : prices[i] - min_price;
    }
    return profit;
}

int main() {
    int case1[6] = {9, 11, 8, 5, 7, 10};
    printf("%d\n", best_profit(case1, 6));
    return 0;
}
