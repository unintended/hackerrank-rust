#![feature(slicing_syntax)]
use std::io::stdio::stdin;
use std::num::SignedInt;

fn main() {

  let n:u32 = stdin().read_line().ok().unwrap().trim().parse().unwrap();
  let k:u32 = stdin().read_line().ok().unwrap().trim().parse().unwrap();

  let mut a:Vec<u32> = std::io::stdin().lock().lines().take(n as uint).map(
    |line| line.unwrap().trim().parse().unwrap()
  ).collect();

  a.sort();
  let res = a.iter().zip(a.iter().skip(k as uint - 1)).fold(
    std::u32::MAX,
    |min, (a, b)| std::cmp::min(min, *b - *a)
  );

  println!("{}", res);
}
