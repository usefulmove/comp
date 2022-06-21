use std::env;

/*

    note: base data structure is a vector (linked
    list) used as a stack. atoms on the list are
    either be symbols (commands) or values. each
    calculation is a list of operations that are
    processed in order of occurrence. this is an
    implementation of a lisp interpreter for rev-
    erse polish notation s-expressions (sexp).

      operations list structure
        (object : command or value)
        "5"
        "sqrt"
        "1
        "-"
        "2"
        "/"

    a list evaluation engine takes the list of
    strings and executes the corresponding oper-
    ations then returns the resulting mutated
    stack.

*/

fn main() {
  const COMP_VERSION: &str = "0.15.0";

  let args: Vec<String> = env::args().collect();

  let ops = &args[1..]; // operations list
  //println!("{:?}", ops); // debug

  // create computation stack
  let mut cstack: Vec<f64> = Vec::new();

  // map process node function over operations list
  ops.iter().map(|op| processnode(&mut cstack, &op)).for_each(drop);

  // display updated stack
  for e in cstack {
    println!("{}", e);
  }
}

fn processnode(stack: &mut Vec<f64>, cmdval: &str) {
  match cmdval {
    // stack manipulatgion
    "drop"   => c_drop(stack), // drop
    "dup"    => c_dup(stack),  // duplicate
    "swap"   => c_swap(stack), // swap x and y
    "cls"    => c_cls(stack),  // clear stack
    "roll"   => c_roll(stack), // roll stack
    // memory usage
    //TODO"sa"   => c_store_a(stack), // store
    //TODO"a"    => c_push_a(stack),  // retrieve
    //TODO"sb"   => c_store_b(stack), // store
    //TODO"b"    => c_push_b(stack),  // retrieve
    //TODO"sc"   => c_store_c(stack), // store
    //TODO"c"    => c_push_c(stack),  // retrieve
    // math operations
    "+"      => c_add(stack),
    "+_"     => c_add_all(stack),
    "-"      => c_sub(stack),
    "x"      => c_mult(stack),
    "x_"     => c_mult_all(stack),
    "/"      => c_div(stack),
    "chs"    => c_chs(stack),
    "abs"    => c_abs(stack),
    "inv"    => c_inv(stack),
    "sqrt"   => c_sqrt(stack),
    "throot" => c_throot(stack),
    "^"      => c_exp(stack),
    "%"      => c_mod(stack),
    "!"      => c_fact(stack),
    _ => stack.push(cmdval.parse::<f64>().unwrap()), // push value onto stack
  }
}

// -- Commands -----------------------------------------------------------------

// -- stack manipulation -------------------------------------------------------

fn c_drop(s: &mut Vec<f64>) {
  s.pop().unwrap();
}

fn c_dup(s: &mut Vec<f64>) {
  let end: usize = s.len() - 1;
  s.push(s[end]);
}

fn c_swap(s: &mut Vec<f64>) {
  let end: usize = s.len() - 1;
  let o: f64 = s[end];
  s[end] = s[end-1];
  s[end-1] = o;
}

fn c_cls(s: &mut Vec<f64>) {
  s.clear();
}

fn c_roll(s: &mut Vec<f64>) {
  let o: f64 = s.pop().unwrap();
  s.splice(0..0, [o]);
}

// -- memory usage -------------------------------------------------------------

//TODO

// -- math operations ----------------------------------------------------------

fn c_add(s: &mut Vec<f64>) {
  let end: usize = s.len() - 1;
  s[end-1] += s.pop().unwrap();
}

fn c_add_all(s: &mut Vec<f64>) {
  while s.len() > 1 {
    let end: usize = s.len() - 1;
    s[end-1] += s.pop().unwrap();
  }
}

fn c_sub(s: &mut Vec<f64>) {
  let end: usize = s.len() - 1;
  s[end-1] -= s.pop().unwrap();
}

fn c_mult(s: &mut Vec<f64>) {
  let end: usize = s.len() - 1;
  s[end-1] *= s.pop().unwrap();
}

fn c_mult_all(s: &mut Vec<f64>) {
  while s.len() > 1 {
    let end: usize = s.len() - 1;
    s[end-1] *= s.pop().unwrap();
  }
}

fn c_div(s: &mut Vec<f64>) {
  let end: usize = s.len() - 1;
  s[end-1] /= s.pop().unwrap();
}

fn c_abs(s: &mut Vec<f64>) {
  let end: usize = s.len() - 1;
  s[end] = f64::abs(s[end]);
}

fn c_chs(s: &mut Vec<f64>) {
  let end: usize = s.len() - 1;
  s[end] = -1.0 * s[end];
}

fn c_inv(s: &mut Vec<f64>) {
  let end: usize = s.len() - 1;
  s[end] = 1.0 / s[end];
}

fn c_sqrt(s: &mut Vec<f64>) {
  let end: usize = s.len() - 1;
  s[end] = f64::sqrt(s[end]);
}

fn c_throot(s: &mut Vec<f64>) {
  let end: usize = s.len() - 1;
  let o: f64 = s.pop().unwrap();
  s[end-1] = s[end-1].powf(1.0/o);
}

fn c_exp(s: &mut Vec<f64>) {
  let end: usize = s.len() - 1;
  let o: f64 = s.pop().unwrap();
  s[end-1] = s[end-1].powf(o);
}

fn c_mod(s: &mut Vec<f64>) {
  let end: usize = s.len() - 1;
  let o: f64 = s.pop().unwrap();
  s[end-1] = s[end-1] % o;
}

fn c_fact(s: &mut Vec<f64>) {
  let end: usize = s.len() - 1;
  s[end] = factorial(s[end] as u64) as f64;
}

fn factorial(n: u64) -> u64 {
  if n < 2 {
    return 1;
  } else {
    return n * factorial(n-1);
  }
}