// Daily Coding Problem: 73
// Difficulty: Easy
// Asked by: Google
// Authored by: TenthEdict

struct Node {
    val: i32,
    next: Option<Box<Node>> 
}

fn reverse(head: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut prev = None;
    let mut curr = head;

    while let Some(mut node) = curr {
        let next = node.next.take();
        node.next = prev;
        prev = Some(node);
        curr = next;
    }

    prev
}   

fn print_list(head: &Option<Box<Node>>) {
    let mut current = head;
    while let Some(node) = current {
        print!("{}", node.val);
        if node.next.is_some() {
            print!(" -> ");
        }
        current = &node.next;
    }
    println!(" -> None");
}

fn main() {
    let node3 = Node { val: 3,  next: None };
    let node2 = Node { val: 2, next: Some(Box::new(node3)) };
    let node1 = Node { val: 1, next: Some(Box::new(node2)) };

    let head = Some(Box::new(node1));

    print_list(&head);

    let reversed = reverse(head);

    print_list(&reversed);
}
