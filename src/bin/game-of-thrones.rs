#![feature(slicing_syntax)]

// extern crate itertools;

use std::io::stdio::stdin;
use std::num::SignedInt;
// use itertools::Itertools;

fn main() {

  let s:String = stdin().read_line().ok().unwrap().trim().to_string();

  let mut cs:Vec<char> = s.chars().collect();
  cs.sort();

  // functional solution doesn't work at hakerrank (itertools crate is missing)
  // let res = cs.iter().group_by(|&i| *i).filter(|t| t.1.len() % 2 != 0).count() <= 1;

  let mut odd_count = 0;
  let mut prev = &cs[0];
  let mut n = 0;

  for c in cs.iter() {
    if c != prev {
      odd_count += n % 2;
      n = 0;
    }
    prev = c;
    n += 1;
  }

  odd_count += n % 2;

  println!("{}", if odd_count <= 1 { "YES" } else { "NO" });

}
