/*
Daily Coding Problem: 184
Difficulty: Easy
Asked by: Amazon
Authored by: TenthEdict
*/

#include <iostream>


int gcd(int a, int b)
{
    int r = a % b;

    if (r == 0)
    {
        return b;
    } else  {
        return gcd(b, r);
    }
    
}

int gcd_of_array(int arr[], int len)
{
    int result = arr[0];

    for (int i = 1; i < len; i++)
    {
        result = gcd(result, arr[i]);
    }
    return result;
}

int main()
{
    int numbers[] = {42, 56, 14};

    std::cout << gcd_of_array(numbers, 3) << std::endl;

    return 0;
}
