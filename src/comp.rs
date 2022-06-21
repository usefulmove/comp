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
  ops.iter().map(|op| processnode(&mut cstack, &op)).collect::<Vec<_>>();

  // display updated stack
  for e in cstack {
    println!("{}", e);
  }
}

fn processnode(s: &mut Vec<f64>, cmdval: &String) {
  //println!("original stack contents: {:?}", s); // debug
  //println!("op = {}", cmdval); // debug

  if cmdval == "+" {
    //println!("add"); // debug
    let ssize: usize = s.len(); // initial stack size
    let val: f64 = s.pop().unwrap();
    s[ssize-2] += val;
  } else {
    s.push(cmdval.parse::<f64>().unwrap()); // TODO this temporarily just pushes values onto the stack
  }

  //println!("final stack contents: {:?}", s); // debug
}
