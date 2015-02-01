use std::io::stdio::stdin;

fn main() {

  let n:u32 = stdin().read_line().ok().unwrap().trim().parse().unwrap();

  let mut a:Vec<u32> = std::io::stdin().lock().lines().take(n as uint).map(
    |line| line.unwrap().trim().parse().unwrap()
  ).collect();

  for i in a.iter() {
    println!("{}", *i ^ !0);
  }

}
