/*
 * Daily Coding Problem: 108
 * Difficulty: Easy
 * Asked By: Google
 * Authored By: TenthEdict
 */

function check_string_rotation(a, b) {
    if (a.length != b.length) { return false };
    return (a + a).includes(b);
}

console.log(check_string_rotation("abcde", "cdeab"));
console.log(check_string_rotation("abcde", "abcde"));
console.log(check_string_rotation("abc", "acb"));
console.log(check_string_rotation("abcde", "abcd"));
console.log(check_string_rotation("", ""));
console.log(check_string_rotation("a", "a"));
