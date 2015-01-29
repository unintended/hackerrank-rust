use std::io::stdio::stdin;

fn main() {

  let n_cases:u32 = stdin().read_line().ok().unwrap().trim().parse().unwrap();
  let numbers_str = stdin().read_line().ok().unwrap().trim().to_string();

  let n = numbers_str.split(' ').fold(0, |a, n| a ^ n.parse().unwrap());

  println!("{}", n);
}
