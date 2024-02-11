fn main() {
    let a = 99;

    if a >= 1000 {
        println!("{a} is a Huge number");
    } else if a < 100 {
        println!("{a} is a Small number");
    } else {
        println!("{a} is a Big number");
    }

    // Ternary Operator does not exists in Rust.
    // But we can write the normal if else statements as ternary operators.
    // Example -
    let is_loading = false;

    let result = if !is_loading {
        "loading"
    } else {
        "Welcome user "
    };
    println!("{}", result)
}
