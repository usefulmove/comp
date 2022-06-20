use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{}", fib(args[1].parse::<i64>().unwrap()));
}

fn fib(n: i64) -> i64 {
  if n <= 0 {
    return 0;
  } else if n < 3 {
    return 1;
  } else {
    return fib(n-1) + fib(n-2);
  }
}
