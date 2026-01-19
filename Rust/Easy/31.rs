// Daily Coding Problem: 31
// Difficulty: Easy
// Asked by: Google
// Author: TenthEdict

fn edit_distance(a: &str, b: &str) -> usize {
    let rows = a.len() + 1;
    let cols = b.len() + 1;

    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    
    // Create table filled with zeros
    let mut table: Vec<Vec<usize>> = vec![vec![0; cols]; rows];
    
    // Initialize first row (empty string → b[0..j])
    for j in 0..cols {
        table[0][j] = j;
    }
    
    // Initialize first column (a[0..i] → empty string)
    for i in 0..rows {
        table[i][0] = i;
    }
    
    for i in 1..rows {
        for j in 1..cols {
            if a_chars[i-1] == b_chars[j-1] {
                table[i][j] = table[i-1][j-1];
            } else {
                table[i][j] = 1 + table[i-1][j-1]
                    .min(table[i-1][j])
                    .min(table[i][j-1]);
            }
        }
    }
    table[a.len()][b.len()]
}

fn main() {
    println!("{}" , edit_distance("cat", "car"));
    println!("{}" ,edit_distance("kitten", "sitting"));
    println!("{}" ,edit_distance("", "abc"));
    println!("{}" ,edit_distance("same", "same"));
}