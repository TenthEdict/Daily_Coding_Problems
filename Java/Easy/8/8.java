// Daily Coding Problem: 8
// Difficulty: Easy
// Asked by: Google
// Authored by: TenthEdict

class Node {
    int val;
    Node left_child;
    Node right_child;

    Node(int val, Node left_child, Node right_child) {
        this.val = val;
        this.left_child = left_child;
        this.right_child = right_child;
    }
}

class Result {
    int count;
    boolean result;

    Result(int count, boolean result) {
        this.count = count;
        this.result = result;
    }
}

class Solution {
    public static void main(String[] args) {
        Node leaf_1a = new Node(1, null, null);
        Node leaf_1b = new Node(1, null, null);
        Node leaf_1c = new Node(1, null, null);
        Node sub_sub_node_0a = new Node(0, null, null);
        Node sub_sub_node_1a = new Node(1, leaf_1b, leaf_1c);
        Node sub_node_0a = new Node(0, sub_sub_node_1a, sub_sub_node_0a);
        Node root = new Node(0, leaf_1a, sub_node_0a);

        Result result = countUnivalSubtrees(root);
        System.out.println(result.count);
        System.out.println(result.result);
        }
    static Result countUnivalSubtrees(Node tree) {
        if (tree == null) {
            return new Result(0, true);
        }

        Result left_result = countUnivalSubtrees(tree.left_child);
        Result right_result = countUnivalSubtrees(tree.right_child);

        if (left_result.result && right_result.result) {
            if ((tree.left_child == null || tree.left_child.val == tree.val) && (tree.right_child == null || tree.right_child.val == tree.val)) {
                return new Result(left_result.count+ right_result.count + 1, true);
            }
        }
        return new Result(left_result.count + right_result.count, false);
    }
}   