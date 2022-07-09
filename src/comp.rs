use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::num::ParseFloatError;
use std::path::Path;
use std::path::Display;
use std::collections::HashMap;
use colored::*;

const RELEASE_STATUS: &str = "b";

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
const CMDS: &str = "drop dup swap cls clr roll rot + +_ - x x_ / chs abs round \
int inv sqrt throot proot ^ exp % mod ! gcd pi e dtor rtod sin asin cos acos \
tan atan log log2 log10 ln logn sa .a a sb .b b sc .c c";


fn main() {
  // enable or disable backtrace on error
  env::set_var("RUST_BACKTRACE", "0");

  // construct command interpreter
  let mut cinter = Interpreter::new();

  // get command line arguments and collect into a vector
  let mut args: Vec<String> = env::args().collect();

  // if no arguments are passed, behave as if help flag was passed
  if args.len() <= 1 {
    args.push("help".to_string());
  }

  if args[1] == "--help" || args[1] == "help" {
    // display command usage information
    show_help();
    std::process::exit(0);

  } else if args[1] == "--version" || args[1] == "version" {
    // display version information
    show_version();
    std::process::exit(0);

  } else if args[1] == "mona" {
    println!("{MONA}");
    std::process::exit(0);

  } else if args[1] == "-f" || args[1] == "--file" {
    // read operations list input from file
    if args.len() > 2 {
      // read file path
      let filename: String = args[2].to_string();
      let path: &Path = Path::new(&filename);
      let display: Display = path.display();

      // open file
      let mut file: File = match File::open(&path) {
        Ok(file) => file,
        Err(error) => {
          eprintln!("{}: could not open file [{}]: {error}", "error".bright_red(), display.to_string().cyan());
          std::process::exit(99);
        },
      };

      // read file contents
      let mut file_contents: String = String::new();
      match file.read_to_string(&mut file_contents) {
        Ok(_) => (),
        Err(error) => {
          eprintln!("{}: could not read [{}]: {error}", "error".bright_red(), display.to_string().cyan());
          std::process::exit(99);
        },
      };

      // split individual list elements
      let temp_ops: Vec<&str> = file_contents.split_whitespace().collect();

      // create operations list vector from file contents
      for op in temp_ops {
        cinter.ops.push(op.to_string());
      }

    } else {
      eprintln!("{}: no file path provided", "error".bright_red());
      std::process::exit(99);

    }

  } else {
    // read operations list input from arguments
    cinter.ops = (&args[1..]).to_vec();

  }

  // process operations list
  cinter.process_ops();

  // display resulting computation stack
  for element in cinter.stack {
    println!("  {}", element.to_string().bright_green());
  }

  std::process::exit(0);
}

struct Function {
  name: String,
  fops: Vec<String>,
}

struct Interpreter {
  stack: Vec<f64>,
  mem_a: f64,
  mem_b: f64,
  mem_c: f64,
  ops: Vec<String>,
  fns: Vec<Function>,
  cmap: HashMap<String, fn(&mut Interpreter, &str)>,
}

impl Interpreter {
  // constructor
  fn new() -> Interpreter {
    let mut cint = Interpreter {
      stack: Vec::new(),
      mem_a: 0.0,
      mem_b: 0.0,
      mem_c: 0.0,
      ops: Vec::new(),
      fns: Vec::new(),
      cmap: HashMap::new(),
    };
    cint.init();

    cint
  }

  // process operations method
  fn process_ops(&mut self) {
    while !self.ops.is_empty() {
      let operation: String = self.ops.remove(0); // pop first operation
      self.process_node(operation.as_str());
    }
  }

  // add native command to interpreter
  fn compose_native(&mut self, name: &str, func: fn(&mut Interpreter, &str)) {
    self.cmap.insert(name.to_string(), func);
  }

  fn init(&mut self) {
    // stack manipulation
    self.compose_native("drop",   Interpreter::c_drop);     // drop
    self.compose_native("dup",    Interpreter::c_dup);      // duplicate
    self.compose_native("swap",   Interpreter::c_swap);     // swap x and y
    self.compose_native("cls",    Interpreter::c_cls);      // clear stack
    self.compose_native("clr",    Interpreter::c_cls);      // clear stack
    self.compose_native("roll",   Interpreter::c_roll);     // roll stack
    self.compose_native("rot",    Interpreter::c_rot);      // rotate stack (reverse direction from roll)
    // memory usage
    self.compose_native("sa",     Interpreter::c_store_a);  // store (pop value off stack and store)
    self.compose_native(".a",     Interpreter::c_store_a);  // store (pop value off stack and store)
    self.compose_native("a",      Interpreter::c_push_a);   // retrieve (push stored value onto the stack)
    self.compose_native("sb",     Interpreter::c_store_b);  // store
    self.compose_native(".b",     Interpreter::c_store_b);  // store
    self.compose_native("b",      Interpreter::c_push_b);   // retrieve
    self.compose_native("sc",     Interpreter::c_store_c);  // store
    self.compose_native(".c",     Interpreter::c_store_c);  // store
    self.compose_native("c",      Interpreter::c_push_c);   // retrieve
    // math operations
    self.compose_native("+",      Interpreter::c_add);      // add
    self.compose_native("+_",     Interpreter::c_add_all);  // add all
    self.compose_native("-",      Interpreter::c_sub);      // subtract
    self.compose_native("x",      Interpreter::c_mult);     // multiply
    self.compose_native("x_",     Interpreter::c_mult_all); // multiply all
    self.compose_native("/",      Interpreter::c_div);      // divide
    self.compose_native("chs",    Interpreter::c_chs);      // change sign
    self.compose_native("abs",    Interpreter::c_abs);      // absolute value
    self.compose_native("round",  Interpreter::c_round);    // round
    self.compose_native("int",    Interpreter::c_round);
    self.compose_native("inv",    Interpreter::c_inv);      // invert (1/x)
    self.compose_native("sqrt",   Interpreter::c_sqrt);     // square root
    self.compose_native("throot", Interpreter::c_throot);   // nth root
    self.compose_native("proot",  Interpreter::c_proot);    // find principal roots
    self.compose_native("^",      Interpreter::c_exp);      // exponentiation
    self.compose_native("exp",    Interpreter::c_exp);
    self.compose_native("%",      Interpreter::c_mod);      // modulus
    self.compose_native("mod",    Interpreter::c_mod);
    self.compose_native("!",      Interpreter::c_fact);     // factorial
    self.compose_native("gcd",    Interpreter::c_gcd);      // greatest common divisor
    self.compose_native("pi",     Interpreter::c_pi);       // pi
    self.compose_native("e",      Interpreter::c_euler);    // Euler's constant
    self.compose_native("dtor",   Interpreter::c_dtor);     // degrees to radians
    self.compose_native("rtod",   Interpreter::c_rtod);     // radians to degrees
    self.compose_native("sin",    Interpreter::c_sin);      // sine
    self.compose_native("asin",   Interpreter::c_asin);     // arcsine
    self.compose_native("cos",    Interpreter::c_cos);      // cosine
    self.compose_native("acos",   Interpreter::c_acos);     // arccosine
    self.compose_native("tan",    Interpreter::c_tan);      // tangent
    self.compose_native("atan",   Interpreter::c_atan);     // arctangent
    self.compose_native("log2",   Interpreter::c_log2);     // log (base 2)
    self.compose_native("log",    Interpreter::c_log10);    // log (base 10)
    self.compose_native("log10",  Interpreter::c_log10);
    self.compose_native("logn",   Interpreter::c_logn);     // log (base n)
    self.compose_native("ln",     Interpreter::c_ln);       // natural log
    // control flow
    self.compose_native("fn",     Interpreter::c_fn);       // function definition
    self.compose_native("(",      Interpreter::c_comment);  // function definition
  }

  fn process_node(&mut self, op: &str) {
    if self.cmap.contains_key(op) { // native comp command?
      let f = self.cmap[op];
      f(self, op);
    } else {
      let result: Option<usize> = self.is_user_function(op); // user-defined function?

      match result {
        Some(index) => { // user-defined function
          // copy user function ops (fops) into main ops
          for i in (0..self.fns[index].fops.len()).rev() {
            let fop: String = self.fns[index].fops[i].clone();
            self.ops.insert(0, fop);
          }
        }
        None => { // neither native command nor user-defined function
          // attempt to parse as value
          let res: Result<f64, ParseFloatError> = self.parse_value(op);

          let val = match res {
            Ok(val) => val, // parsed successfully
            Err(_error) => { // parse failed
              eprintln!("{}: unknown expression: [{}] is not a recognized operation \
                         or value", "error".bright_red(), op.to_string().cyan());
              std::process::exit(99);
            },
          };

          // push value onto stack
          self.stack.push(val);
        }
      }
    }
  }

  fn parse_value(&self, op: &str) -> Result<f64, ParseFloatError> {
    let value: f64 = op.parse::<f64>()?;
    Ok(value)
  }

  // check stack depth
  fn check_stack_error(&self, min_depth: usize, command: &str) {
    if self.stack.len() < min_depth {
      eprintln!("{}: [{}] operation called without at least {min_depth} element(s) on stack", "error".bright_red(), command.to_string().cyan());
      std::process::exit(99);
    }
  }


  // command functions ---------------------------------------------------------
  // ---- stack manipulation ---------------------------------------------------

  fn c_drop(&mut self, op: &str) {
    if !self.stack.is_empty() {
      self.stack.pop().unwrap();
    } else {
      println!("{}: [{}] operation called on empty stack", "warning".bright_yellow(), op.to_string().cyan());
    }
  }

  fn c_dup(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    let end: usize = self.stack.len() - 1;
    self.stack.push(self.stack[end]);
  }

  fn c_swap(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 2, op);

    let end: usize = self.stack.len() - 1;
    self.stack.swap(end, end-1);
  }

  fn c_cls(&mut self, _op: &str) {
    self.stack.clear();
  }

  fn c_roll(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    let o: f64 = self.stack.pop().unwrap(); // remove last
    self.stack.splice(0..0, [o]);           // add as first
  }

  fn c_rot(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    let o: f64 = self.stack.remove(0); // remove first
    self.stack.push(o);                // add as last
  }


  // ---- memory usage ---------------------------------------------------------

  fn c_store_a(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    self.mem_a = self.stack.pop().unwrap();
  }

  fn c_push_a(&mut self, _op: &str) {
    self.stack.push(self.mem_a);
  }

  fn c_store_b(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    self.mem_b = self.stack.pop().unwrap();
  }

  fn c_push_b(&mut self, _op: &str) {
    self.stack.push(self.mem_b);
  }

  fn c_store_c(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    self.mem_c = self.stack.pop().unwrap();
  }

  fn c_push_c(&mut self, _op: &str) {
    self.stack.push(self.mem_c);
  }


  // ---- math operations ------------------------------------------------------

  fn c_add(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 2, op);

    let end: usize = self.stack.len() - 1;
    self.stack[end-1] += self.stack.pop().unwrap();
  }

  fn c_add_all(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 2, op);

    while self.stack.len() > 1 {
      let end: usize = self.stack.len() - 1;
      self.stack[end-1] += self.stack.pop().unwrap();
    }
  }

  fn c_sub(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 2, op);

    let end: usize = self.stack.len() - 1;
    self.stack[end-1] -= self.stack.pop().unwrap();
  }

  fn c_mult(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 2, op);

    let end: usize = self.stack.len() - 1;
    self.stack[end-1] *= self.stack.pop().unwrap();
  }

  fn c_mult_all(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 2, op);

    while self.stack.len() > 1 {
      let end: usize = self.stack.len() - 1;
      self.stack[end-1] *= self.stack.pop().unwrap();
    }
  }

  fn c_div(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 2, op);

    let end: usize = self.stack.len() - 1;
    self.stack[end-1] /= self.stack.pop().unwrap();
  }

  fn c_chs(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    let end: usize = self.stack.len() - 1;
    self.stack[end] *= -1.0;
  }

  fn c_abs(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    let end: usize = self.stack.len() - 1;
    self.stack[end] = f64::abs(self.stack[end]);
  }

  fn c_round(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    let end: usize = self.stack.len() - 1;
    self.stack[end] = self.stack[end].round();
  }

  fn c_inv(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    let end: usize = self.stack.len() - 1;
    self.stack[end] = 1.0 / self.stack[end];
  }

  fn c_sqrt(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    let end: usize = self.stack.len() - 1;
    self.stack[end] = f64::sqrt(self.stack[end]);
  }

  fn c_throot(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    let end: usize = self.stack.len() - 1;
    let o: f64 = self.stack.pop().unwrap();
    self.stack[end-1] = self.stack[end-1].powf(1.0/o);
  }

  fn c_proot(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 3, op);

    let c: f64 = self.stack.pop().unwrap();
    let b: f64 = self.stack.pop().unwrap();
    let a: f64 = self.stack.pop().unwrap();

    if (b*b - 4.0*a*c) < 0.0 {
      self.stack.push(-1.0*b/(2.0*a)); // root_1 real
      self.stack.push(f64::sqrt(4.0*a*c-b*b)/(2.0*a)); // root_1 imag
      self.stack.push(-1.0*b/(2.0*a)); // root_2 real
      self.stack.push(-1.0*f64::sqrt(4.0*a*c-b*b)/(2.0*a)); // root_2 imag
    } else {
      self.stack.push(-1.0*b+f64::sqrt(b*b-4.0*a*c)/(2.0*a)); // root_1 real
      self.stack.push(0.0); // root_1 imag
      self.stack.push(-1.0*b-f64::sqrt(b*b-4.0*a*c)/(2.0*a)); // root_2 real
      self.stack.push(0.0); // root_2 imag
    }
  }

  fn c_exp(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 2, op);

    let end: usize = self.stack.len() - 1;
    let o: f64 = self.stack.pop().unwrap();
    self.stack[end-1] = self.stack[end-1].powf(o);
  }

  fn c_mod(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 2, op);

    let end: usize = self.stack.len() - 1;
    let o: f64 = self.stack.pop().unwrap();
    self.stack[end-1] %= o;
  }

  fn c_fact(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    let end: usize = self.stack.len() - 1;
    self.stack[end] = Interpreter::factorial(self.stack[end].round());
  }

  fn c_gcd(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 2, op);

    let a: u64 = self.stack.pop().unwrap() as u64;
    let b: u64 = self.stack.pop().unwrap() as u64;
    let g: f64 = Interpreter::gcd(a,b) as f64;
    self.stack.push(g);
  }

  fn c_pi(&mut self, _op: &str) {
    self.stack.push(std::f64::consts::PI);
  }

  fn c_euler(&mut self, _op: &str) {
    self.stack.push(std::f64::consts::E);
  }

  fn c_dtor(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    let end: usize = self.stack.len() - 1;
    self.stack[end] = self.stack[end].to_radians();
  }

  fn c_rtod(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    let end: usize = self.stack.len() - 1;
    self.stack[end] = self.stack[end].to_degrees();
  }

  fn c_sin(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    let end: usize = self.stack.len() - 1;
    self.stack[end] = self.stack[end].sin();
  }

  fn c_asin(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    let end: usize = self.stack.len() - 1;
    self.stack[end] = self.stack[end].asin();
  }

  fn c_cos(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    let end: usize = self.stack.len() - 1;
    self.stack[end] = self.stack[end].cos();
  }

  fn c_acos(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    let end: usize = self.stack.len() - 1;
    self.stack[end] = self.stack[end].acos();
  }

  fn c_tan(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    let end: usize = self.stack.len() - 1;
    self.stack[end] = self.stack[end].tan();
  }

  fn c_atan(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    let end: usize = self.stack.len() - 1;
    self.stack[end] = self.stack[end].atan();
  }

  fn c_log10(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    let end: usize = self.stack.len() - 1;
    self.stack[end] = self.stack[end].log10();
  }

  fn c_log2(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    let end: usize = self.stack.len() - 1;
    self.stack[end] = self.stack[end].log2();
  }

  fn c_logn(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    let end: usize = self.stack.len() - 1;
    let n: f64 = self.stack.pop().unwrap();
    self.stack[end-1] = self.stack[end-1].log(n);
  }

  fn c_ln(&mut self, op: &str) {
    Interpreter::check_stack_error(self, 1, op);

    let end: usize = self.stack.len() - 1;
    self.stack[end] = self.stack[end].ln();
  }


  // -- control flow -----------------------------------------------------------

  fn c_fn(&mut self, _op: &str) {
    // get function name
    let fn_name: String = self.ops.remove(0);

    // create new function instance and assign function name
    self.fns.push(Function { name: fn_name,
                             fops: Vec::new(),
                           });
    let fpos: usize = self.fns.len() - 1; // added function position in function vector

    // build out function operations my reading from interpreter ops
    while self.ops[0] != "end" {
      self.fns[fpos].fops.push(self.ops.remove(0));
    }
    self.ops.remove(0); // remove "end" op
  }

  // is operator a user defined function?
  fn is_user_function(&self, op: &str) -> Option<usize> {
    if !self.fns.is_empty() {
      for i in 0..self.fns.len() {
        if self.fns[i].name == op {
          return Some(i);
        }
      }
    }
    None
  }

  fn c_comment(&mut self, _op: &str) {
    let mut nested: usize = 0;

    while !self.ops.is_empty() {
      let op = self.ops.remove(0);
      match &op[..] {
        "(" => {
          nested += 1;
        },
        ")" => {
          if nested == 0 {
            return;
          } else {
            nested -= 1;
          }
        },
        _ => (),
      }
    }
  }


  // support functions ---------------------------------------------------------
  
  // factorial
  fn factorial(n: f64) -> f64 {
    if n < 2.0 {
      1.0
    } else {
      n * Interpreter::factorial(n - 1.0)
    }
  }
  
  // greatest common divisor
  fn gcd(a: u64, b: u64) -> u64 {
    if b != 0 {
      Interpreter::gcd(b, a % b)
    } else {
      a
    }
  }

}


fn show_help() {
  println!("Command interpreter");
  println!();
  println!("Usage:");
  println!("    comp [version] [help]");
  println!("    comp <list>");
  println!("    comp -f <file>");
  println!();
  println!("Options:");
  println!("        --version          show version");
  println!("    -f, --file             used to specify a path to a file");
  println!("        --help             display help and usage information");
  println!();
  println!("<list> is a sequence of comp language (postfix) operations and \
  <file> specifies the directory path to a file containing a sequence of \
  comp operations. Each operation is either a command (symbol) or a value. \
  As examples, 'comp 3 4 +' adds the values 3 and 4 and '3 dup x 4 dup x +' \
  computes the sum of the squares of 3 and 4. The available commands are \
  listed below.");
  println!();
  println!("The usage guide can be found at https://github.com/usefulmove/\
  comp/blob/main/USAGE.md.");
  println!();
  println!("Commands:");
  println!("{CMDS}");
  println!();
  println!("Examples:");
  println!("    comp 1 2 +                  add 1 and 2");
  println!("    comp 5 2 /                  divide 5 by 2");
  println!("    comp 3 dup x 4 dup x +      sum of the squares of 3 and 4");
}

fn show_version() {
  let version: &str = env!("CARGO_PKG_VERSION");
  println!("  comp {}", version.to_string() + RELEASE_STATUS);
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



#[cfg(test)]
#[path = "./comp_test.rs"]
mod comp_test;
