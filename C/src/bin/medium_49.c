// Daily Coding Problem: 49
// Difficulty: Medium
// Asked by: Amazon
// Authored by: TenthEdict

#include <stdio.h>
#include <stdbool.h>

bool max_subarray_sum(int *arr, int len, int *result) {
    if (len <= 0) {
        *result = 0;
        return false;
    }

    int current_sum = 0;
    int max_sum = 0;

    for (int i = 0; i < len; i++)
    {
        current_sum += arr[i]; // issue 1
        max_sum = (max_sum > current_sum) ? max_sum : current_sum;
        current_sum = (current_sum < 0) ? 0 : current_sum;
    }

    *result = max_sum; // issue 3

    printf("%d\n", *result); // issue 2

    return true;
}

int main() {
    int test[] = {34, -50, 42, 14, -5, 86};
    int result;
    
    max_subarray_sum(test, sizeof(test) / sizeof(*test), &result);
    
    return 0;
}
