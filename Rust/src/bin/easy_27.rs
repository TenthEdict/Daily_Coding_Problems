// Daily Coding Problem: 27
// Difficulty: Easy
// Asked by: Google
// Authored by: TenthEdict

fn is_balanced(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        if matches!(c, '(' | '{' | '[') {
            stack.push(c);
            continue;
        } else {
            if stack.is_empty() {
                return false;
            }

            let expected = match c {
                ')' => '(',
                ']' => '[',
                '}' => '{',
                _ => return false
            };

            let popped = stack.pop();
        
            if popped == Some(expected) {
                continue;
            } else {
                return false;
            }
        } 
    }
    stack.is_empty()
}

fn main() {
    let case1 = "([])[]({})";
    let case2 = "([)]";
    let case3 = "((()";
    let case4 = ")";
    let case5 = "(";
    let case6 = "";
    let case7 = "(a)";
    let case8 = "(0";

    println!("{} -> {}", case1, is_balanced(case1));
    println!("{} -> {}", case2, is_balanced(case2));
    println!("{} -> {}", case3, is_balanced(case3));
    println!("{} -> {}", case4, is_balanced(case4));
    println!("{} -> {}", case5, is_balanced(case5));
    println!("{} -> {}", case6, is_balanced(case6));
    println!("{} -> {}", case7, is_balanced(case7));
    println!("{} -> {}", case8, is_balanced(case8));
}
