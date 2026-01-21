# Daily Coding Problem: 73
# Difficulty: Easy
# Asked by: Google
# Authored by: TenthEdict

class Node:
    def __init__(self, val, next=None):
        self.val = val
        self.next = next

def reverse(head):
    prev = None
    curr = head

    while curr:
        next = curr.next
        curr.next = prev
        prev = curr
        curr = next

    return prev

def print_list(head):
    while(head):
        print(head.val)
        head = head.next


def main():
    node3 = Node(3)
    node2 = Node(2, node3)
    node1 = Node(1,node2)

    print_list(reverse(node1))

main()
