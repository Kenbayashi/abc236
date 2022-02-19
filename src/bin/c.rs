use std::io;
use std::io::Read;

fn main() {
    let _n = read();
    let _m = read();

    let mut input = String::new();

    io::stdin().read_line(&mut input).ok().unwrap();
    let st_slow = input.split_whitespace()
                       .map(|s| String::from(s))
                       .collect::<Vec<String>>();

    input.clear();
    io::stdin().read_line(&mut input).ok().unwrap();
    let st_fast = input.split_whitespace()
                       .map(|s| String::from(s))
                       .collect::<Vec<String>>();

    st_slow.into_iter()
           .map(|st| st_fast.contains(&st))
           .map(|ans| if ans {String::from("Yes")} else {String::from("No")})
           .for_each(|message| println!("{}", message));
}

fn read() -> isize {
    io::stdin().bytes()
               .map(|c| c.unwrap() as char)
               .skip_while(|c| c.is_whitespace())
               .take_while(|c| !c.is_whitespace())
               .collect::<String>()
               .parse::<isize>()
               .ok()
               .unwrap()
}
