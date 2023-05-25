use rand::prelude::*;

/*
 * reference: https://rust-random.github.io/book/guide-start.html
 */

fn main() {
    let mut pw = Vec::new();
    
    for _ in 0..12 {
        let mut rng = rand::thread_rng();
        let random_symbol: char = rng.gen_range('\u{0020}'..'\u{007E}');

        while random_symbol == '\u{0020}' {
            let random_symbol: char = rng.gen_range('\u{0020}'..'\u{007E}');
        }
        pw.push(random_symbol);
    }

    let s_pw = pw.into_iter().collect::<String>();

    println!("{}", s_pw.trim());
}
