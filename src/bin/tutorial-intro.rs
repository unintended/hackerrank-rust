use std::io::stdio::stdin;

fn main() {

  let target:i32 = stdin().read_line().ok().unwrap().trim().parse().unwrap();
  stdin().read_line();
  let numbers_str = stdin().read_line().ok().unwrap().trim().to_string();

  let mut idx = 0;
  for i in numbers_str.split(' ') {
    if target == i.parse().unwrap() {
      println!("{}", idx);
      return;
    }
    idx += 1;
  }
}
