use std::io;
use std::io::Read;
use std::str::FromStr;

fn read<T: FromStr>() -> T where T: Default {
    io::stdin().bytes()
               .map(|c| c.unwrap() as char)
               .skip_while(|&c| c.is_whitespace())
               .take_while(|&c| !c.is_whitespace())
               .collect::<String>()
               .parse::<T>()
               .unwrap_or_default()
}

fn swap_char(s: String, a: usize, b: usize) -> String {
    let char_a = s.chars().nth(a - 1).unwrap();
    let char_b = s.chars().nth(b - 1).unwrap();

    let mut array = s.chars().collect::<Vec<char>>();
    array[a - 1] = char_b;
    array[b - 1] = char_a;

    array.iter().collect::<String>()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();

    let a: usize = read();
    let b: usize = read();

    println!("{}", swap_char(input, a, b));
}
