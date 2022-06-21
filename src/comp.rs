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

  // create computation stack and memory
  let mut cstack = CompositeStack{
                     stack: Vec::new(),
                     mem_a: 0.0,
                     mem_b: 0.0,
                     mem_c: 0.0,
                   };

  // map process node function over operations list
  ops.iter().map(|op| processnode(&mut cstack, &op)).for_each(drop);

  // display updated stack
  for e in cstack.stack {
    println!("{}", e);
  }
}

struct CompositeStack {
  stack: Vec<f64>,
  mem_a: f64,
  mem_b: f64,
  mem_c: f64,
}

fn processnode(cs: &mut CompositeStack, cmdval: &str) {
  match cmdval {
    // stack manipulatgion
    "drop"   => c_drop(cs), // drop
    "dup"    => c_dup(cs),  // duplicate
    "swap"   => c_swap(cs), // swap x and y
    "cls"    => c_cls(cs),  // clear stack
    "roll"   => c_roll(cs), // roll stack
    // memory usage
    "sa"     => c_store_a(cs), // store (pop value off stack and store)
    "a"      => c_push_a(cs),  // retrieve (push stored value onto the stack)
    "sb"     => c_store_b(cs), // store
    "b"      => c_push_b(cs),  // retrieve
    "sc"     => c_store_c(cs), // store
    "c"      => c_push_c(cs),  // retrieve
    // math operations
    "+"      => c_add(cs),
    "+_"     => c_add_all(cs),
    "-"      => c_sub(cs),
    "x"      => c_mult(cs),
    "x_"     => c_mult_all(cs),
    "/"      => c_div(cs),
    "chs"    => c_chs(cs),
    "abs"    => c_abs(cs),
    "inv"    => c_inv(cs),
    "sqrt"   => c_sqrt(cs),
    "throot" => c_throot(cs),
    "^"      => c_exp(cs),
    "%"      => c_mod(cs),
    "!"      => c_fact(cs),
    _ => cs.stack.push(cmdval.parse::<f64>().unwrap()), // push value onto stack
  }
}

// -- Commands -----------------------------------------------------------------

// -- stack manipulation -------------------------------------------------------

fn c_drop(cs: &mut CompositeStack) {
  cs.stack.pop().unwrap();
}

fn c_dup(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  cs.stack.push(cs.stack[end]);
}

fn c_swap(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  let o: f64 = cs.stack[end];
  cs.stack[end] = cs.stack[end-1];
  cs.stack[end-1] = o;
}

fn c_cls(cs: &mut CompositeStack) {
  cs.stack.clear();
}

fn c_roll(cs: &mut CompositeStack) {
  let o: f64 = cs.stack.pop().unwrap();
  cs.stack.splice(0..0, [o]);
}

// -- memory usage -------------------------------------------------------------

fn c_store_a(cs: &mut CompositeStack) {
  cs.mem_a = cs.stack.pop().unwrap();
}

fn c_push_a(cs: &mut CompositeStack) {
  cs.stack.push(cs.mem_a);
}

fn c_store_b(cs: &mut CompositeStack) {
  cs.mem_b = cs.stack.pop().unwrap();
}

fn c_push_b(cs: &mut CompositeStack) {
  cs.stack.push(cs.mem_b);
}

fn c_store_c(cs: &mut CompositeStack) {
  cs.mem_c = cs.stack.pop().unwrap();
}

fn c_push_c(cs: &mut CompositeStack) {
  cs.stack.push(cs.mem_c);
}


// -- math operations ----------------------------------------------------------

fn c_add(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  cs.stack[end-1] += cs.stack.pop().unwrap();
}

fn c_add_all(cs: &mut CompositeStack) {
  while cs.stack.len() > 1 {
    let end: usize = cs.stack.len() - 1;
    cs.stack[end-1] += cs.stack.pop().unwrap();
  }
}

fn c_sub(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  cs.stack[end-1] -= cs.stack.pop().unwrap();
}

fn c_mult(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  cs.stack[end-1] *= cs.stack.pop().unwrap();
}

fn c_mult_all(cs: &mut CompositeStack) {
  while cs.stack.len() > 1 {
    let end: usize = cs.stack.len() - 1;
    cs.stack[end-1] *= cs.stack.pop().unwrap();
  }
}

fn c_div(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  cs.stack[end-1] /= cs.stack.pop().unwrap();
}

fn c_abs(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  cs.stack[end] = f64::abs(cs.stack[end]);
}

fn c_chs(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  cs.stack[end] = -1.0 * cs.stack[end];
}

fn c_inv(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  cs.stack[end] = 1.0 / cs.stack[end];
}

fn c_sqrt(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  cs.stack[end] = f64::sqrt(cs.stack[end]);
}

fn c_throot(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  let o: f64 = cs.stack.pop().unwrap();
  cs.stack[end-1] = cs.stack[end-1].powf(1.0/o);
}

fn c_exp(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  let o: f64 = cs.stack.pop().unwrap();
  cs.stack[end-1] = cs.stack[end-1].powf(o);
}

fn c_mod(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  let o: f64 = cs.stack.pop().unwrap();
  cs.stack[end-1] = cs.stack[end-1] % o;
}

fn c_fact(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  cs.stack[end] = factorial(cs.stack[end] as u64) as f64;
}

fn factorial(n: u64) -> u64 {
  if n < 2 {
    return 1;
  } else {
    return n * factorial(n-1);
  }
}
