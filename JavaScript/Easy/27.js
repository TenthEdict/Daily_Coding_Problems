/*
 * Daily Coding Problem: 27
 * Difficulty: Easy
 * Asked By: Google
 * Authored By: TenthEdict
 */

function is_balanced(str) {
    let stack = [];
    let expected = "";

    for (let c of str) {
        if (['[', '{', '('].includes(c)) {
            stack.push(c);
            continue;
        } else {
            if (stack.length == 0) {
                return false;
            }
            switch (c) {
                case ']': 
                    expected = '[';
                    break;
                case ')':  
                    expected = '(';
                    break;
                case '}': 
                    expected = '{';
                    break;
                default: 
                    return false;
            }

            let popped = stack.pop()

            if (popped == expected) {
                continue;
            } else {
                return false;
            }
        }
    }
    if (stack.length == 0) {
        return true;
    } else {
        return false;
    }
}

console.log(is_balanced("([])[]({})"))
console.log(is_balanced("([)]"))
console.log(is_balanced(""))
console.log(is_balanced("((()"))