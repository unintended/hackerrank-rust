#![feature(slicing_syntax)]

use std::io::stdio::stdin;
use std::slice::SliceConcatExt;

fn join_array(a: &Vec<i32>) -> String {
  let sa:Vec<String> = a.iter().map(|e| e.to_string()).collect();
  sa.connect(" ")
}

fn main() {

  stdin().read_line();
  let mut list:Vec<i32> = stdin().read_line().ok().unwrap().trim().to_string().split(' ').map(|s| s.parse().unwrap()).collect();

  let mut last = list.len() - 1;
  let mut n = list[last];

  for i in (1..list.len()).rev() {
    if list[i - 1] > n {
      list[i] = list[i - 1];
      println!("{}", join_array(&list));
      last = i - 1;
      continue;
    }
    last = i;
    break;
  }
  list[last] = n;
  println!("{}", join_array(&list));
}
