/*
 * Daily Coding Problem: 73
 * Difficulty: Easy
 * Asked By: Google
 * Authored By: TenthEdict
 */

class Node {
    constructor(val, next=null) {
        this.val = val;
        this.next = next;
    }
}

function reverse(head) {
    let prev = null;
    let curr = head;

    while (curr) {
        let next = curr.next;
        curr.next = prev;
        prev = curr;
        curr = next;
    }
    return prev;
}

function print_list(head) {
    while (head) {
        console.log(head.val);
        head = head.next
    }
}

let node3 = new Node(3);
let node2 = new Node(2, node3);
let node1 = new Node(1, node2);

console.log(print_list(reverse(node1)))
