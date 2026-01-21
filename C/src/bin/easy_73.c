// Daily Coding Problem: 73
// Difficulty: Easy
// Asked by: Google
// Authored by: TenthEdict

#include <stdio.h>
#include <stdlib.h>

typedef struct Node
{
    int val;
    struct Node* next;
} Node;

Node* reverse(Node* head)
{
    Node* prev = NULL;
    Node* curr = head;

    while (curr)
    {
        Node* next = curr->next;
        curr->next = prev;
        prev = curr;
        curr = next;
    }
    return prev;
}

void print_list(Node* head)
{
    while (head)
    {
        printf("%d\n", head->val);

        head = head->next;
    }
}

int main()
{
    Node* ptr3 = malloc(sizeof(Node));
    ptr3->val = 3;
    ptr3->next = NULL;

    Node* ptr2 = malloc(sizeof(Node));
    ptr2->val = 2;
    ptr2->next = ptr3;

    Node* ptr1 = malloc(sizeof(Node));
    ptr1->val = 1;
    ptr1->next = ptr2;

    print_list(ptr1);
    Node* reversed = reverse(ptr1);
    print_list(reversed);

    return 0;
}
