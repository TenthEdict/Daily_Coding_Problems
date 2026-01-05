// Daily Coding Problem: 108
// Difficulty: Easy
// Asked by: Google
// Authored by: TenthEdict

function checkStringRotation(a: string, b: string): boolean {
    if (a.length != b.length) { return false }
    return (a+a).includes(b);
}

console.log(checkStringRotation("abcde", "cdeab"));
console.log(checkStringRotation("abcde", "abcde"));
console.log(checkStringRotation("abc", "acb"));
console.log(checkStringRotation("abcde", "abcd"));
console.log(checkStringRotation("", ""));
console.log(checkStringRotation("a", "a"));