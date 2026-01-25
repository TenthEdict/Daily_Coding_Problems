// Daily Coding Problem: 43
// Difficulty: Easy
// Asked by: Amazon
// Authored by: TenthEdict

import java.util.ArrayDeque;

public class easy_43 {
    public static void main(String[] args) {
        MaxStack stack = new MaxStack();

        // Test empty stack edge cases
        System.out.println("max() on empty: " + stack.max());   // null
        System.out.println("pop() on empty: " + stack.pop());   // null

        // Test basic push/max
        stack.push(3);
        stack.push(1);
        stack.push(5);
        stack.push(2);
        System.out.println("\nAfter pushing 3, 1, 5, 2:");
        System.out.println("max() = " + stack.max());  // 5

        // Test pop and max updates
        System.out.println("\nPopping...");
        System.out.println("pop() = " + stack.pop());  // 2
        System.out.println("max() = " + stack.max());  // 5

        System.out.println("pop() = " + stack.pop());  // 5
        System.out.println("max() = " + stack.max());  // 3

        System.out.println("pop() = " + stack.pop());  // 1
        System.out.println("max() = " + stack.max());  // 3

        System.out.println("pop() = " + stack.pop());  // 3
        System.out.println("max() = " + stack.max());  // null (empty)
    }

    static class MaxStack {
        ArrayDeque<Integer> pushed_vals;
        ArrayDeque<Integer> running_max;
        
        public MaxStack() {
            this.pushed_vals = new ArrayDeque<Integer>();
            this.running_max = new ArrayDeque<Integer>();
        }

        public void push(Integer val) {
            this.pushed_vals.push(val);

            if (running_max.isEmpty()) {
                this.running_max.push(val);
            } else {

                Integer new_max = (val > running_max.peek()) ? val : running_max.peek();

                this.running_max.push(new_max);
            }
        }
        
        public Integer pop() {
            if (this.pushed_vals.isEmpty()) {
                return null;
            }
            this.running_max.pop();
            return this.pushed_vals.pop();
        }

        public Integer max() {
            if (this.running_max.isEmpty()) {
                return null;
            }

            return this.running_max.peek();
        }
    }
}      

