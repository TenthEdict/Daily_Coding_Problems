// Daily Coding Problem: 108
// Difficulty: Easy
// Asked by: Google
// Authored by: TenthEdict

package main

import (
	"fmt"
	"strings"
)

func checkStringRotation(a string, b string) bool {
	if len(a) != len(b) { return false}
	return strings.Contains(a+a, b)
}

func main() {
	fmt.Println(checkStringRotation("abcde", "cdeab"))
	fmt.Println(checkStringRotation("abc", "acb"))    // false
	fmt.Println(checkStringRotation("", ""))          // true
	fmt.Println(checkStringRotation("abc", "abcd"))   // false
}