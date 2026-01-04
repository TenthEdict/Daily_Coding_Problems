// Daily Coding Problem: 118
// Difficulty: Easy
// Asked By: Google
// Author: TenthEdict

function sortedSquares(nums: number[]): number[] {
    let left: number = 0;
    let right: number = nums.length - 1;
    let result: number[] = new Array(nums.length);
    let position: number = nums.length - 1;

    while (left <= right) {
        if (nums[left]**2 > nums[right]**2) {
            result[position] = nums[left]**2;
            left++;
            position--;
        } else {
            result[position] = nums[right]**2;
            right--;
            position--;
        }

    }
    return result;
}

let arr: number[] = [-9,-2,0,2,3];
console.log(sortedSquares(arr));
