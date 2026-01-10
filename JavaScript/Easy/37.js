// Daily Coding Problem: 37
// Difficulty: Easy
// Asked By: Google
// Author: TenthEdict

function power_set(arr) {
    let n = arr.length;
    let result = [];

    for (let i = 0; i < (1 << n); i++) {
        let subset = [];

        for (let j = 0; j < n; j++) {
            if ((i & (1 << j)) != 0) {
                subset.push(arr[j]);
            }
        }
        result.push(subset);
    }
    return result;
}