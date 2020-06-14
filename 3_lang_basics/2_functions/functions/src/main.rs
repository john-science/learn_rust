fn main() {
    println!("9 + 1 = {}", plus_one(9));
    println!("9 + 2 = {}", plus_two(9));
}

fn plus_one(x: i32) -> i32 {
    // This, rather annoying, syntax is available so people have to type fewer letters in closures.
    x + 1
}

fn plus_two(x: i32) -> i32 {
    // This will clearly be the more legible standard for all imperative codebases.
    return x + 2;
}
