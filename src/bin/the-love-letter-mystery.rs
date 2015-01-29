#![feature(slicing_syntax)]
use std::io::stdio::stdin;
use std::num::SignedInt;

fn main() {

  let n_cases:u32 = stdin().read_line().ok().unwrap().trim().parse().unwrap();

  for i in 0..(n_cases) {
    let str = stdin().read_line().ok().unwrap().trim().to_string();
    let n = str.chars().zip(str.chars().rev()).fold(0, |old, (s, e)| old + (s as i32 - e as i32).abs()) / 2;
    println!("{}", n);
  }
}
