/*
Daily Coding Problem: 73
Difficulty: Easy
Asked by: Google
Authored by: TenthEdict
*/

#include <iostream>

struct Node
{
   int val;
   Node* next;

   Node(int v, Node* n = nullptr) : val(v), next(n) {}
};

Node* reverse(Node* curr) 
{
    Node* prev = nullptr;
    
    while (curr) 
    {
        Node* next = curr->next;
        curr->next = prev;
        prev = curr;
        curr = next;
    }
    return prev;
};

void print_list(Node* head)
{
    while (head)
    {
        std::cout << head->val << std::endl;

        head = head->next;
    }
};

int main()
{
    Node* ptr3 = new Node(3);
    Node* ptr2 = new Node(2, ptr3);
    Node* ptr1 = new Node(1, ptr2);

    print_list(reverse(ptr1));

    return 0;
}
