// Daily Coding Problem: 73
// Difficulty: Easy
// Asked by: Google
// Authored by: TenthEdict

class Easy73 {

    public static void main(String[] args) {
        Node node3 = new Node(3, null);
        Node node2 = new Node(2, node3);
        Node node1 = new Node(1, node2);

    printList(node1);
    System.out.println("---");
    printList(reverse(node1));
    }

    static class Node {
        int val;
        Node next;

        Node(int val, Node next) {
            this.val = val;
            this.next = next;
        }
    }

    static Node reverse(Node head) {
        Node prev = null;
        Node curr = head;

        while (curr != null) {
            Node next = curr.next;
            curr.next = prev;
            prev = curr;
            curr = next;
        }
        return prev;
}

    static void printList(Node head) {
        while (head != null) {
            System.out.println(head.val);
            head = head.next;
        }
    }
}
