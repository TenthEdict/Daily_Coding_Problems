// Daily Coding Problem: 163
// Difficulty: Hard
// Asked by: Jane Street
// Author: TenthEdict

enum Token {
    Num(i32),
    Op(char)
}

fn evaluate_rpn(expression: Vec<Token>) -> Option<i32> {

    let mut ints: Vec<i32> = Vec::new();

    for token in &expression {
        match token {

            Token::Num(value) => ints.push(*value),

            Token::Op(operator) => {
                let first = ints.pop()?;
                let second = ints.pop()?;

                match operator {
                    '*' => ints.push(second * first),
                    '-' => ints.push(second - first),
                    '/' => if first == 0 {return None} else {ints.push(second / first)},
                    '+' => ints.push(second + first),
                    _ => return None
                }
            }
        }
    }
    ints.pop()
}

fn main() {
    // [5, 3, '+'] = 8
    let test1 = vec![Token::Num(5), Token::Num(0), Token::Op('/')];
    
    // [4, 2, '/', 3, '*'] = 6
    let test2 = vec![Token::Num(4), Token::Num(2), Token::Op('/'), Token::Num(3), Token::Op('*')];
    
    // The big example from the problem
    // [15, 7, 1, 1, '+', '-', '/', 3, '*', 2, 1, 1, '+', '+', '-'] = 5
    let test3 = vec![
        Token::Num(15), Token::Num(7), Token::Num(1), Token::Num(1),
        Token::Op('+'), Token::Op('-'), Token::Op('/'),
        Token::Num(3), Token::Op('*'),
        Token::Num(2), Token::Num(1), Token::Num(1),
        Token::Op('+'), Token::Op('+'), Token::Op('-')
    ];
    
    println!("{:?}", evaluate_rpn(test1));
    println!("{:?}", evaluate_rpn(test2));
    println!("{:?}", evaluate_rpn(test3));
}
