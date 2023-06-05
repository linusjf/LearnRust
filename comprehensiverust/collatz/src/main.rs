// Program entry point
fn main() {
    // Mutable variable binding
    let mut x: i32 = 6;
    // Macro for printing, like printf
    print!("{x}");
    while x != 1 {
        // No parenthesis around expression
        if x % 2 == 0 {
            // Math like in other languages
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        print!(" -> {x}");
    }
    println!();
}
