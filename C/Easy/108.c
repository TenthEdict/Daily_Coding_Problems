// Daily Coding Problem: 108
// Difficulty: Easy
// Asked by: Google
// Authored by: TenthEdict

#include <stdio.h>
#include <string.h>
#include <stdbool.h>

bool checkStringRotation(const char* a, const char* b) {

    if (strlen(a) != strlen(b)) {
        return false;
    }

    char doubled[256];

    sprintf(doubled, "%s%s", a, a);

    if (strstr(doubled, b) == NULL) {
        return false;
    } else {
        return true;
    }
}

int main() {
    printf("%d\n", checkStringRotation("abcde", "cdeab"));  // 1 (true)
    printf("%d\n", checkStringRotation("abc", "acb"));      // 0 (false)
    printf("%d\n", checkStringRotation("", ""));            // 1 (true)
    printf("%d\n", checkStringRotation("abc", "abcd"));     // 0 (false)
    return 0;
}