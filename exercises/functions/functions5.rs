// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(mut num: i32) -> i32 {
    num * &num         // remove the return statement and allow it to just return the value
}
