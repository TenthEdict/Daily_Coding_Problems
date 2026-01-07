// Daily Coding Problem: 108
// Difficulty: Easy
// Asked by: Google
// Authored by: TenthEdict

class _108 {
    public static void main(String[] args) {
       Boolean result = checkStringRotation("abcde", "cdeab");
       System.out.println(result);
    }

    static Boolean checkStringRotation(String a, String b) {
        if (a.length() != b.length()) {
            return false;
        }
        String doubled = a + a; 

        return doubled.contains(b);
    }
}