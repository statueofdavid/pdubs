use rand::prelude::*;

/*
 * reference: https://rust-random.github.io/book/guide-start.html
 */
fn gen_rand_char_not_space() -> char {
    let mut rng = rand::thread_rng();
    let _random_symbol: char = rng.gen_range('\u{0020}'..'\u{007E}');
    let is_space = _random_symbol.is_ascii_whitespace();
    
    if !is_space {
        return _random_symbol;
    } else {
        gen_rand_char_not_space()
    }
}

fn main() {
    let mut pw = Vec::new();

    for _ in 0..12 {
        pw.push(gen_rand_char_not_space());
    }

    let s_pw = pw.into_iter().collect::<String>();
    println!("{}", s_pw.trim());
}
