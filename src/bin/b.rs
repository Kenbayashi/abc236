use std::{collections::HashMap, io::Read};

fn main() {
    let n = read();

    let cards = (1..(4 * n)).into_iter()
                            .map(|_| read())
                            .collect::<Vec<i32>>();

    let mut map = HashMap::<i32, i32>::new();

    cards.into_iter()
         .for_each(|num| *map.entry(num).or_default() += 1);

    let (ans, _) = map.into_iter()
                      .find(|&(_, count)| count != 4)
                      .unwrap();

    println!("{}", ans);
}

fn read() -> i32 {
    std::io::stdin().bytes()
                    .map(|c| c.unwrap() as char)
                    .skip_while(|c| c.is_whitespace())
                    .take_while(|c| !c.is_whitespace())
                    .collect::<String>()
                    .parse::<i32>()
                    .ok()
                    .unwrap()
}
