use std::env;
use std::collections::HashMap;

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
  //println!("original stack contents: {:?}", stack); // debug
  //println!("op = {}", cmdval); // debug

  match cmdval {
    "+"    => c_add(stack),
    "-"    => c_sub(stack),
    "x"    => c_mult(stack),
    "/"    => c_div(stack),
    "sqrt" => c_sqrt(stack),
    _ => stack.push(cmdval.parse::<f64>().unwrap()), // push value onto stack
  }

  //println!("final stack contents: {:?}", stack); // debug
}

// -- Commands -----------------------------------------------------------------

// -- stack manipulation -------------------------------------------------------

//TODO

// -- math operations ----------------------------------------------------------

fn c_add(s: &mut Vec<f64>) {
  let ssize: usize = s.len(); // initial stack size
  let val: f64 = s.pop().unwrap();
  s[ssize-2] += val;
}

fn c_sub(s: &mut Vec<f64>) {
  let ssize: usize = s.len(); // initial stack size
  let val: f64 = s.pop().unwrap();
  s[ssize-2] -= val;
}

fn c_mult(s: &mut Vec<f64>) {
  let ssize: usize = s.len(); // initial stack size
  let val: f64 = s.pop().unwrap();
  s[ssize-2] *= val;
}

fn c_div(s: &mut Vec<f64>) {
  let ssize: usize = s.len(); // initial stack size
  let val: f64 = s.pop().unwrap();
  s[ssize-2] /= val;
}

fn c_sqrt(s: &mut Vec<f64>) {
  let ssize: usize = s.len(); // initial stack size
  s[ssize-1] = f64::sqrt(s[ssize-1]);
}
