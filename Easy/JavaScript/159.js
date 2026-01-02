/**
 * Daily Coding Problem: 159
 * Difficulty: Easy
 * Asked By: Google
 * Authored By: TenthEdict
 * 
 * Question: Given a string, return the first recurring character in it, or null if there is no recurring character. For example, given the string "acbbac", return "b". Given the string "abcdef", return null.
 * 
 */

function first_recurring_character(str) {
    let seen = {}
    for (i in str) {
        if (str[i] in seen) {
            return str[i]
        } else {
            seen[str[i]] = true
        }
    }
    return null
}

console.log(first_recurring_character("acbbac")) // prints b
console.log(first_recurring_character("abcdef")) // prints null
console.log(first_recurring_character("abcdef")) // prints null
