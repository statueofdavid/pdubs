use rand::prelude::*;

/*
 * reference: https://rust-random.github.io/book/guide-start.html
 */

fn main() {
    let pw: char = random();
    println!("{}", pw)
}
