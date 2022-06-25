use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const COMP_VERSION: &str = "0.15.5";

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

// -- command list -------------------------------------------------------------

const CMDS: &str = "drop dup swap cls clr roll sa .a a sb .b b sc .c c + +_ - x \
x_ / chs abs round int inv sqrt throot proot ^ exp % mod ! gcd pi e dtor rtod \
sin asin cos acos tan atan log log10 ln";


fn main() {
  let mut args: Vec<String> = env::args().collect();

  let mut ops = Vec::new();

  if args.len() <= 1 {
    args.push("help".to_string());
  }

  if args[1] == "--help" || args[1] == "help" {
    // display command usage information
    println!("usage: comp [version] [help]");
    println!("       comp <list>");
    println!("       comp -f <file>");
    println!();
    println!("where <list> represents a sequence of reverse Polish notion (RPN) \
    postfix operations or <file> is a file containing a similar sequence of \
    operations. Each operation must be either a command (symbol) or value. As \
    examples, 'comp 3 4 +' adds the values 3 and 4 and '3 dup x 4 dup x +' \
    computes the sum of the squares of 3 and 4. The available commands are \
    listed below.");
    println!();
    println!("commands:");
    println!("{}", CMDS);

    return;
  } else if args[1] == "--version" || args[1] == "version" {
    // display version information
    println!("comp {}", COMP_VERSION);
    return;
  } else if args[1] == "mona" {
    println!("{}", MONA);
    return;
  } else if args[1] == "-f" || args[1] == "--file" {
    // read operations list input from file
    print!("reading command input from '{}' file .. ", args[2].to_string()); // debug

    let filename = args[2].to_string();
    let path = Path::new(&filename);
    let display = path.display();

    let mut file = match File::open(&path) {
                     Err(why) => panic!("couldn't open {}: {}", display, why),
                     Ok(file) => file,
                   };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
      Err(why) => panic!("couldn't read {}: {}", display, why),
      Ok(_) => println!("success"),
    };

    let temp_ops: Vec<&str> = contents.split_whitespace().collect();

    // create operations list vector
    for op in temp_ops {
      ops.push(op.to_string());
    }
  } else {
    // read operations list input from command line arguments
    ops = (&args[1..]).to_vec(); //remove
  }
  // println!("{:?}", ops); // debug message

  // create composite computation stack with memory locations
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


fn processnode(cs: &mut CompositeStack, op: &str) {
  match op {
    // stack manipulation
    "drop"   => c_drop(cs),      // drop
    "dup"    => c_dup(cs),       // duplicate
    "swap"   => c_swap(cs),      // swap x and y
    "cls"    => c_cls(cs),       // clear stack
    "clr"    => c_cls(cs),       // clear stack
    "roll"   => c_roll(cs),      // roll stack
    // memory usage
    "sa"     => c_store_a(cs),   // store (pop value off stack and store)
    ".a"     => c_store_a(cs),   // store (pop value off stack and store)
    "a"      => c_push_a(cs),    // retrieve (push stored value onto the stack)
    "sb"     => c_store_b(cs),   // store
    ".b"     => c_store_b(cs),   // store
    "b"      => c_push_b(cs),    // retrieve
    "sc"     => c_store_c(cs),   // store
    ".c"     => c_store_c(cs),   // store
    "c"      => c_push_c(cs),    // retrieve
    // math operations
    "+"      => c_add(cs),       // add
    "+_"     => c_add_all(cs),   // add all
    "-"      => c_sub(cs),       // subtract
    "x"      => c_mult(cs),      // multiply
    "x_"     => c_mult_all(cs),  // multiply all
    "/"      => c_div(cs),       // divide
    "chs"    => c_chs(cs),       // change sign
    "abs"    => c_abs(cs),       // absolute value
    "round"  => c_round(cs),     // round
    "int"    => c_round(cs),
    "inv"    => c_inv(cs),       // invert (1/x)
    "sqrt"   => c_sqrt(cs),      // square root
    "throot" => c_throot(cs),    // nth root
    "proot"  => c_proot(cs),     // find principal roots
    "^"      => c_exp(cs),       // exponenation
    "exp"    => c_exp(cs),
    "%"      => c_mod(cs),       // modulus
    "mod"    => c_mod(cs),
    "!"      => c_fact(cs),      // factorial
    "gcd"    => c_gcd(cs),       // greatest common divisor
    "pi"     => c_pi(cs),        // pi
    "e"      => c_euler(cs),     // Euler's constant
    "dtor"   => c_dtor(cs),      // degrees to radians
    "rtod"   => c_rtod(cs),      // radians to degrees
    "sin"    => c_sin(cs),       // sine
    "asin"   => c_asin(cs),      // arcsine
    "cos"    => c_cos(cs),       // cosine
    "acos"   => c_acos(cs),      // arccosine
    "tan"    => c_tan(cs),       // tangent
    "atan"   => c_atan(cs),      // arctangent
    "log"    => c_log10(cs),     // log (base 10)
    "log10"  => c_log10(cs),
    "ln"     => c_ln(cs),        // natural log
    _ => cs.stack.push(op.parse::<f64>().unwrap()), // push value onto stack
  }
}


// -- command functions --------------------------------------------------------

// ---- stack manipulation -----------------------------------------------------

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


// ---- memory usage -----------------------------------------------------------

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

fn c_chs(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  cs.stack[end] *= -1.0;
}

fn c_abs(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  cs.stack[end] = f64::abs(cs.stack[end]);
}

fn c_round(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  cs.stack[end] = cs.stack[end].round();
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

fn c_proot(cs: &mut CompositeStack) {
  let c: f64 = cs.stack.pop().unwrap();
  let b: f64 = cs.stack.pop().unwrap();
  let a: f64 = cs.stack.pop().unwrap();

  if (b*b - 4.0*a*c) < 0.0 {
    cs.stack.push(-1.0*b/(2.0*a)); // root1 real
    cs.stack.push(f64::sqrt(4.0*a*c-b*b)/(2.0*a)); // root1 imag
    cs.stack.push(-1.0*b/(2.0*a)); // root2 real
    cs.stack.push(-1.0*f64::sqrt(4.0*a*c-b*b)/(2.0*a)); // root2 imag
  } else {
    cs.stack.push(-1.0*b+f64::sqrt(b*b-4.0*a*c)/(2.0*a)); // root1 real
    cs.stack.push(0.0); // root1 imag
    cs.stack.push(-1.0*b-f64::sqrt(b*b-4.0*a*c)/(2.0*a)); // root2 real
    cs.stack.push(0.0); // root2 imag
  }
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

fn c_gcd(cs: &mut CompositeStack) {
  let a: u64 = cs.stack.pop().unwrap() as u64;
  let b: u64 = cs.stack.pop().unwrap() as u64;
  let g: f64 = gcd(a,b) as f64;
  cs.stack.push(g);
}

fn c_pi(cs: &mut CompositeStack) {
  cs.stack.push(std::f64::consts::PI);
}

fn c_euler(cs: &mut CompositeStack) {
  cs.stack.push(std::f64::consts::E);
}

fn c_dtor(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  cs.stack[end] = cs.stack[end].to_radians();
}

fn c_rtod(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  cs.stack[end] = cs.stack[end].to_degrees();
}

fn c_sin(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  cs.stack[end] = cs.stack[end].sin();
}

fn c_asin(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  cs.stack[end] = cs.stack[end].asin();
}

fn c_cos(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  cs.stack[end] = cs.stack[end].cos();
}

fn c_acos(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  cs.stack[end] = cs.stack[end].acos();
}

fn c_tan(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  cs.stack[end] = cs.stack[end].tan();
}

fn c_atan(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  cs.stack[end] = cs.stack[end].atan();
}

fn c_log10(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  cs.stack[end] = cs.stack[end].log10();
}

fn c_ln(cs: &mut CompositeStack) {
  let end: usize = cs.stack.len() - 1;
  cs.stack[end] = cs.stack[end].ln();
}


// -- support functions --------------------------------------------------------

fn factorial(n: u64) -> u64 {
  if n < 2 {
    return 1;
  } else {
    return n * factorial(n-1);
  }
}

fn gcd(a: u64, b: u64) -> u64 {
  if b != 0 {
    return gcd(b, a % b)
  } else {
    return a
  }
}


// -- mona ---------------------------------------------------------------------

const MONA: &str = "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!>''''''<!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n\
       !!!!!!!!!!!!!!!!!!!!!!!!!!!!'''''`             ``'!!!!!!!!!!!!!!!!!!!!!!!!\n\
       !!!!!!!!!!!!!!!!!!!!!!!!''`          .....         `'!!!!!!!!!!!!!!!!!!!!!\n\
       !!!!!!!!!!!!!!!!!!!!!'`      .      :::::'            `'!!!!!!!!!!!!!!!!!!\n\
       !!!!!!!!!!!!!!!!!!!'     .   '     .::::'                `!!!!!!!!!!!!!!!!\n\
       !!!!!!!!!!!!!!!!!'      :          `````                   `!!!!!!!!!!!!!!\n\
       !!!!!!!!!!!!!!!!        .,cchcccccc,,.                       `!!!!!!!!!!!!\n\
       !!!!!!!!!!!!!!!     .-\"?$$$$$$$$$$$$$$c,                      `!!!!!!!!!!!\n\
       !!!!!!!!!!!!!!    ,ccc$$$$$$$$$$$$$$$$$$$,                     `!!!!!!!!!!\n\
       !!!!!!!!!!!!!    z$$$$$$$$$$$$$$$$$$$$$$$$;.                    `!!!!!!!!!\n\
       !!!!!!!!!!!!    <$$$$$$$$$$$$$$$$$$$$$$$$$$:.                    `!!!!!!!!\n\
       !!!!!!!!!!!     $$$$$$$$$$$$$$$$$$$$$$$$$$$h;:.                   !!!!!!!!\n\
       !!!!!!!!!!'     $$$$$$$$$$$$$$$$$$$$$$$$$$$$$h;.                   !!!!!!!\n\
       !!!!!!!!!'     <$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$                   !!!!!!!\n\
       !!!!!!!!'      `$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$F                   `!!!!!!\n\
       !!!!!!!!        c$$$$???$$$$$$$P\"\"  \"\"\"??????\"                      !!!!!!\n\
       !!!!!!!         `\"\" .,.. \"$$$$F    .,zcr                            !!!!!!\n\
       !!!!!!!         .  dL    .?$$$   .,cc,      .,z$h.                  !!!!!!\n\
       !!!!!!!!        <. $$c= <$d$$$   <$$$$=-=+\"$$$$$$$                  !!!!!!\n\
       !!!!!!!         d$$$hcccd$$$$$   d$$$hcccd$$$$$$$F                  `!!!!!\n\
       !!!!!!         ,$$$$$$$$$$$$$$h d$$$$$$$$$$$$$$$$                   `!!!!!\n\
       !!!!!          `$$$$$$$$$$$$$$$<$$$$$$$$$$$$$$$$'                    !!!!!\n\
       !!!!!          `$$$$$$$$$$$$$$$$\"$$$$$$$$$$$$$P>                     !!!!!\n\
       !!!!!           ?$$$$$$$$$$$$??$c`$$$$$$$$$$$?>'                     `!!!!\n\
       !!!!!           `?$$$$$$I7?\"\"    ,$$$$$$$$$?>>'                       !!!!\n\
       !!!!!.           <<?$$$$$$c.    ,d$$?$$$$$F>>''                       `!!!\n\
       !!!!!!            <i?$P\"??$$r--\"?\"\"  ,$$$$h;>''                       `!!!\n\
       !!!!!!             $$$hccccccccc= cc$$$$$$$>>'                         !!!\n\
       !!!!!              `?$$$$$$F\"\"\"\"  `\"$$$$$>>>''                         `!!\n\
       !!!!!                \"?$$$$$cccccc$$$$??>>>>'                           !!\n\
       !!!!>                  \"$$$$$$$$$$$$$F>>>>''                            `!\n\
       !!!!!                    \"$$$$$$$$???>'''                                !\n\
       !!!!!>                     `\"\"\"\"\"                                        `\n\
       !!!!!!;                       .                                          `\n\
       !!!!!!!                       ?h.\n\
       !!!!!!!!                       $$c,\n\
       !!!!!!!!>                      ?$$$h.              .,c\n\
       !!!!!!!!!                       $$$$$$$$$hc,.,,cc$$$$$\n\
       !!!!!!!!!                  .,zcc$$$$$$$$$$$$$$$$$$$$$$\n\
       !!!!!!!!!               .z$$$$$$$$$$$$$$$$$$$$$$$$$$$$\n\
       !!!!!!!!!             ,d$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$          .\n\
       !!!!!!!!!           ,d$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$         !!\n\
       !!!!!!!!!         ,d$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$        ,!'\n\
       !!!!!!!!>        c$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$.\n\
       !!!!!!''       ,d$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$      allen mullen";


// -- unit tests ---------------------------------------------------------------

#[cfg(test)]

mod comp_tests {

  #[test]
  fn test_base() {
    let mut testcs = super::CompositeStack{
                       stack: Vec::new(),
                       mem_a: 20.0,
                       mem_b: 6.18,
                       mem_c: -123.45,
                     };
                             
    // TODO
  }

  #[test]
  fn test_support() {
    assert!(super::gcd(55, 10) == 5);
    assert!(super::factorial(10) == 3628800);
  }

}
