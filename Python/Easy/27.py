# Daily Coding Problem: 27
# Difficulty: Easy
# Asked by: Google
# Authored By: TenthEdict

def is_balanced(string):
    stack = list()
    expected = ''

    for c in string:
        if c in ['[', '{', '(']:
            stack.append(c)
            continue
        else:
            if len(stack) == 0:
                return False
            match c:
                case ']':
                    expected = '['
                case ')':
                    expected = '('
                case '}':
                    expected = '{'
                case _:
                    return False
        
            popped = stack.pop()

            if popped == expected:
                continue
            else:
                return False
    
    if len(stack) == 0:
        return True
    else:
        return False
    
case1 = "([])[]({})"
case2 = "([)]"
case3 = "((()"
case4 = ")"
case5 = "("
case6 = ""
case7 = "(a)"
case8 = "(0"

def main():
    print(is_balanced(case1))
    print(is_balanced(case2))
    print(is_balanced(case3))
    print(is_balanced(case4))
    print(is_balanced(case5))
    print(is_balanced(case6))
    print(is_balanced(case7))
    print(is_balanced(case8))

main()