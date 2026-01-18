# Daily Coding Problem: 50
# Difficulty: Easy
# Asked by: Microsoft
# Author: TenthEdict

class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        

def evaluate(node):
    if isinstance(node.val, int):
        return node.val
    else: 
        operator = node.val
        match operator:
            case '+':
                return evaluate(node.left) + evaluate(node.right)
            case '-':
                return evaluate(node.left) - evaluate(node.right)
            case '*':
                return evaluate(node.left) * evaluate(node.right)
            case '/':
                return evaluate(node.left) / evaluate(node.right)
            

left_subtree = Node('+', Node(3), Node(2))
right_subtree = Node('+', Node(4), Node(5))
root = Node('*', left_subtree, right_subtree)

print(evaluate(root))