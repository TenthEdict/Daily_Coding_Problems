/*
 * Daily Coding Problem: 73
 * Difficulty: Easy
 * Asked By: Google
 * Authored By: TenthEdict
 */

interface ListNode {
    val: number;
    next: ListNode | null;
}

function reverse(head: ListNode | null): ListNode | null {
    let prev = null; 
    let curr = head; 

    while (curr) {
        let next = curr.next;
        curr.next = prev;
        prev = curr;
        curr = next
    }
    return prev;
}

// Helper to visualize the list
function printList(head: ListNode | null): void {
    while (head) {
        console.log(head.val);
        head = head.next;
    }
}

// Build: 1 -> 2 -> 3 -> null
const node3: ListNode = {
    val: 3,
    next: null,
}
const node2: ListNode = {
    val: 2,
    next: node3,
}

const node1: ListNode = {
    val: 1,
    next: node2,
}

// Then reverse it and print both

console.log(printList(reverse(node1)));