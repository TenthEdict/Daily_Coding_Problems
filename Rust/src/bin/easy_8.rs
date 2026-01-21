// Daily Coding Problem: 8
// Difficulty: Easy
// Asked by: Google
// Authored by: TenthEdict

struct Node {
    val: i32,
    left_child: Option<Box<Node>>, 
    right_child: Option<Box<Node>>
}

fn count_unival_subtrees(tree: Option<Box<Node>>) -> (bool, i32) {
    match tree {
        None => (true, 0),
        Some(node) => {
            let left_child = node.left_child;
            let left_val = left_child.as_ref().map(|n| n.val);
            let right_child = node.right_child;
            let right_val = right_child.as_ref().map(|n| n.val);


            let (left_result, left_count) = count_unival_subtrees(left_child);
            let (right_result, right_count) = count_unival_subtrees(right_child);

            if left_result && right_result {
                if (left_val.is_none() || left_val == Some(node.val)) && (right_val.is_none() || right_val == Some(node.val)) {
                    return (true, left_count + right_count + 1)
                }
            }
            return (false, left_count + right_count)
        }
    }
}

fn main() {
    let leaf_1a = Node { val: 1, left_child: None, right_child: None };
    let leaf_1b = Node { val: 1, left_child: None, right_child: None };
    let leaf_1c = Node { val: 1, left_child: None, right_child: None };
    let sub_sub_node_1a = Node { val: 1, left_child: Some(Box::new(leaf_1b)), right_child: Some(Box::new(leaf_1c)) };
    let sub_sub_node_0a = Node { val: 0, left_child: None, right_child: None };
    let sub_node_0a = Node { val: 0, left_child: Some(Box::new(sub_sub_node_1a)), right_child: Some(Box::new(sub_sub_node_0a)) };
    let root = Node { val: 0, left_child: Some(Box::new(leaf_1a)), right_child: Some(Box::new(sub_node_0a)) };

    let result = count_unival_subtrees(Some(Box::new(root)));
    println!("{:?}", result);
}
