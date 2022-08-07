use colored::*;
use home;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::num::{ParseFloatError, ParseIntError};
use std::path::Path;

const RELEASE_STATE: &str = "k";

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
const CMDS: &str = "drop dup swap cls roll rot + ++ +_ - -- x x_ / chs abs round \
int inv sqrt throot proot ^ exp % mod ! gcd pi e deg_rad rad_deg sin asin cos \
acos tan atan log log2 log10 ln logn sa _a sb _b sc _c dec_hex hex_dec dec_bin \
bin_dec hex_bin bin_hex rgb_hex hex_rgb c_f f_c a_b min min_ max max_ avg avg_ rand";

fn main() {
    // enable or disable backtrace on error
    env::set_var("RUST_BACKTRACE", "0");

    // construct command interpreter
    let mut cinter = Interpreter::new();

    // get command arguments
    let mut args: Vec<String> = env::args().collect();

    // if no arguments are passed, behave as if help flag was passed
    if args.len() <= 1 {
        args.push("help".to_string());
    }

    match args[1].as_str() {
        "--help" | "help" => {
            // display command usage information
            show_help();
            return;
        }
        "--version" | "version" => {
            // display version information
            show_version();
            return;
        }
        "--config" | "config" => {
            // display and write config file
            println!(
                "  configuration file [{}] cannot be generated in this release",
                color_blue_smurf_bold("comp.toml"),
            );
            //TODO (future enhancement)
            //show_config();
            //write_config();
            return;
        }
        "mona" => {
            println!("{MONA}");
            return;
        }
        "-f" | "--file" => {
            // read operations list input from file
            if args.get(2).is_none() {
                eprintln!("  {}: no file path provided", color_red_bold("error"),);
                std::process::exit(99);
            }

            // read file contents
            let filename: String = args[2].to_string();
            let path: &Path = Path::new(&filename);

            let file_contents = fs::read_to_string(&path);
            if let Err(ref error) = file_contents {
                eprintln!(
                    "  {}: could not read [{}]: {error}",
                    color_red_bold("error"),
                    color_blue_coffee_bold(path.display().to_string().as_str()),
                );
                std::process::exit(99);
            }

            let file_contents: String = file_contents.unwrap();

            // create operations list vector from file contents - split elements
            let operations = file_contents.split_whitespace().map(|o| o.to_string());
            cinter.ops.extend(operations);

            // add additional operations from command line
            if args.get(3).is_some() {
                cinter.ops.extend((&args[3..]).to_vec());
            }
        }
        _ => {
            // read operations list input from command line arguments
            cinter.ops = (&args[1..]).to_vec();
        }
    };

    // load configuration
    cinter.read_config("comp.toml");

    // process operations list
    cinter.process_ops();

    print_stack(&mut cinter.stack.clone());
}

struct Function {
    name: String,
    fops: Vec<String>,
}

#[derive(Deserialize)]
struct Config {
    conv_const: f64,
}

impl Config {
    // constructor
    fn new() -> Config {
        Config {
            conv_const: 1.0,
        }
    }
}

struct Interpreter {
    stack: Vec<String>,
    mem_a: f64,
    mem_b: f64,
    mem_c: f64,
    ops: Vec<String>,
    fns: Vec<Function>,
    cmap: HashMap<String, fn(&mut Interpreter, &str)>,
    config: Config,
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
            config: Config::new(),
        };
        cint.init();

        cint
    }

    // process operations method
    fn process_ops(&mut self) {
        while !self.ops.is_empty() {
            let operation: String = self.ops.remove(0); // pop first operation
            self.process_node(&operation);
        }
    }

    // add native command to interpreter
    fn compose_native(&mut self, name: &str, func: fn(&mut Interpreter, &str)) {
        self.cmap.insert(name.to_string(), func);
    }

    fn init(&mut self) {
        /* stack manipulation */
        self.compose_native("drop", Interpreter::c_drop); // drop
        self.compose_native("dup", Interpreter::c_dup); // duplicate
        self.compose_native("swap", Interpreter::c_swap); // swap x and y
        self.compose_native("cls", Interpreter::c_cls); // clear stack
        self.compose_native("clr", Interpreter::c_cls); // clear stack
        self.compose_native("roll", Interpreter::c_roll); // roll stack
        self.compose_native("rot", Interpreter::c_rot); // rotate stack (reverse direction from roll)
        /* memory usage */
        self.compose_native("sa", Interpreter::c_store_a); // store (pop value off stack and store)
        self.compose_native("_a", Interpreter::c_push_a); // retrieve (push stored value onto the stack)
        self.compose_native("sb", Interpreter::c_store_b); // store
        self.compose_native("_b", Interpreter::c_push_b); // retrieve
        self.compose_native("sc", Interpreter::c_store_c); // store
        self.compose_native("_c", Interpreter::c_push_c); // retrieve
        /* math operations */
        self.compose_native("+", Interpreter::c_add); // add
        self.compose_native("+_", Interpreter::c_add_all); // add all
        self.compose_native("++", Interpreter::c_add_one); // add one
        self.compose_native("-", Interpreter::c_sub); // subtract
        self.compose_native("--", Interpreter::c_sub_one); // subtract one
        self.compose_native("x", Interpreter::c_mult); // multiply
        self.compose_native("x_", Interpreter::c_mult_all); // multiply all
        self.compose_native("/", Interpreter::c_div); // divide
        self.compose_native("chs", Interpreter::c_chs); // change sign
        self.compose_native("abs", Interpreter::c_abs); // absolute value
        self.compose_native("round", Interpreter::c_round); // round
        self.compose_native("int", Interpreter::c_round);
        self.compose_native("inv", Interpreter::c_inv); // invert (1/x)
        self.compose_native("sqrt", Interpreter::c_sqrt); // square root
        self.compose_native("throot", Interpreter::c_throot); // nth root
        self.compose_native("proot", Interpreter::c_proot); // find principal roots
        self.compose_native("^", Interpreter::c_exp); // exponentiation
        self.compose_native("exp", Interpreter::c_exp);
        self.compose_native("%", Interpreter::c_mod); // modulus
        self.compose_native("mod", Interpreter::c_mod);
        self.compose_native("!", Interpreter::c_fact); // factorial
        self.compose_native("gcd", Interpreter::c_gcd); // greatest common divisor
        self.compose_native("pi", Interpreter::c_pi); // pi
        self.compose_native("e", Interpreter::c_euler); // Euler's constant
        self.compose_native("deg_rad", Interpreter::c_degrad); // degrees to radians
        self.compose_native("rad_deg", Interpreter::c_raddeg); // radians to degrees
        self.compose_native("sin", Interpreter::c_sin); // sine
        self.compose_native("asin", Interpreter::c_asin); // arcsine
        self.compose_native("cos", Interpreter::c_cos); // cosine
        self.compose_native("acos", Interpreter::c_acos); // arccosine
        self.compose_native("tan", Interpreter::c_tan); // tangent
        self.compose_native("atan", Interpreter::c_atan); // arctangent
        self.compose_native("log2", Interpreter::c_log2); // logarithm (base 2)
        self.compose_native("log", Interpreter::c_log10); // logarithm (base 10)
        self.compose_native("log10", Interpreter::c_log10);
        self.compose_native("logn", Interpreter::c_logn); // logarithm (base n)
        self.compose_native("ln", Interpreter::c_ln); // natural logarithm
        self.compose_native("rand", Interpreter::c_rand); // random number
        self.compose_native("min", Interpreter::c_min); // minimum
        self.compose_native("min_", Interpreter::c_min_all); // minimum
        self.compose_native("max", Interpreter::c_max); // maximum
        self.compose_native("max_", Interpreter::c_max_all); // maximum all
        self.compose_native("avg", Interpreter::c_avg); // average
        self.compose_native("avg_", Interpreter::c_avg_all); // average all
        /* control flow */
        self.compose_native("(", Interpreter::c_function); // function definition
        self.compose_native("ifeq", Interpreter::c_ifeq); // ifequal..else
        self.compose_native("<", Interpreter::c_comment); // function comment
        /* conversion */
        self.compose_native("dec_hex", Interpreter::c_dechex); // decimal to hexadecimal
        self.compose_native("hex_dec", Interpreter::c_hexdec); // hexadecimal to decimal
        self.compose_native("dec_bin", Interpreter::c_decbin); // decimal to binary
        self.compose_native("bin_dec", Interpreter::c_bindec); // binary to decimal
        self.compose_native("bin_hex", Interpreter::c_binhex); // binary to hexadecimal
        self.compose_native("hex_bin", Interpreter::c_hexbin); // hexadecimal to binary
        self.compose_native("c_f", Interpreter::c_celfah); // Celsius to Fahrenheit
        self.compose_native("f_c", Interpreter::c_fahcel); // Fahrenheit to Celsius
        self.compose_native("mi_km", Interpreter::c_mikm); // miles to kilometers
        self.compose_native("km_mi", Interpreter::c_kmmi); // kilometers to miles
        self.compose_native("hex_rgb", Interpreter::c_hexrgb); // hexadecimal string to RGB
        self.compose_native("rgb_hex", Interpreter::c_rgbhex); // RGB to hexadecimal string
        self.compose_native("tip", Interpreter::c_tip); // calculate tip
        self.compose_native("tip+", Interpreter::c_tip_plus); // calculate better tip
        self.compose_native("a_b", Interpreter::c_conv_const); // apply convert constant
    }

    fn process_node(&mut self, op: &str) {
        if self.cmap.contains_key(op) {
            // native comp command?
            let f = self.cmap[op];
            f(self, op);
        } else {
            let result: Option<usize> = self.is_user_function(op); // user-defined function?

            match result {
                Some(index) => {
                    // user-defined function
                    // copy user function ops (fops) into main ops
                    for i in (0..self.fns[index].fops.len()).rev() {
                        let fop: String = self.fns[index].fops[i].clone();
                        self.ops.insert(0, fop);
                    }
                }
                None => {
                    // neither native command nor user-defined function
                    // push value onto stack
                    self.stack.push(op.to_string());
                }
            }
        }
    }

    // pop from stack helpers --------------------------------------------------
    fn pop_stack_float(&mut self) -> f64 {
        let element: String = self.stack.pop().unwrap();
        match self.parse_float(&element) {
            Ok(val) => val, // parse success
            Err(_error) => {
                // parse fail
                eprintln!(
                    "  {}: unknown expression [{}] is not a recognized operation \
                    or valid value (f)",
                    color_red_bold("error"),
                    color_blue_coffee_bold(element.as_str()),
                );
                std::process::exit(99);
            }
        }
    }

    fn pop_stack_uint(&mut self) -> u64 {
        let element: String = self.stack.pop().unwrap();
        match self.parse_uint(&element) {
            Ok(val) => val, // parse success
            Err(_error) => {
                // parse fail
                eprintln!(
                    "  {}: unknown expression [{}] is not a recognized operation \
                    or valid value (u)",
                    color_red_bold("error"),
                    color_blue_coffee_bold(element.as_str()),
                );
                std::process::exit(99);
            }
        }
    }

    fn pop_stack_int_from_hex(&mut self) -> i64 {
        let element: String = self.stack.pop().unwrap();

        match i64::from_str_radix(&element, 16) {
            Ok(val) => val, // parse success
            Err(_error) => {
                // parse fail
                eprintln!(
                    "  {}: unknown expression [{}] is not a recognized operation \
                    or valid value (i_h)",
                    color_red_bold("error"),
                    color_blue_coffee_bold(element.as_str()),
                );
                std::process::exit(99);
            }
        }
    }

    fn pop_stack_int_from_bin(&mut self) -> i64 {
        let element: String = self.stack.pop().unwrap();

        match i64::from_str_radix(&element, 2) {
            Ok(val) => val, // parse success
            Err(_error) => {
                // parse fail
                eprintln!(
                    "  {}: unknown expression [{}] is not a recognized operation \
                    or valid value (i_b)",
                    color_red_bold("error"),
                    color_blue_coffee_bold(element.as_str()),
                );
                std::process::exit(99);
            }
        }
    }

    fn parse_float(&self, op: &str) -> Result<f64, ParseFloatError> {
        let value: f64 = op.parse::<f64>()?;
        Ok(value)
    }

    fn parse_uint(&self, op: &str) -> Result<u64, ParseIntError> {
        let value: u64 = op.parse::<u64>()?;
        Ok(value)
    }
    // -------------------------------------------------------------------------

    // confirm stack depth
    fn check_stack_error(&self, min_depth: usize, command: &str) {
        if self.stack.len() < min_depth {
            eprintln!(
                "  {}: [{}] operation called without at least {min_depth} \
                element(s) on stack",
                color_red_bold("error"),
                color_blue_coffee_bold(command),
            );
            std::process::exit(99);
        }
    }

    // command functions -------------------------------------------------------
    // ---- stack manipulation -------------------------------------------------

    fn c_drop(&mut self, op: &str) {
        if !self.stack.is_empty() {
            self.stack.pop();
        } else {
            eprintln!(
                "  {}: [{}] operation called on empty stack",
                color_yellow_canary_bold("warning"),
                color_blue_coffee_bold(op),
            );
            // do not stop execution
        }
    }

    fn c_dup(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let end: usize = self.stack.len() - 1;
        let o: String = self.stack[end].clone(); // remove last

        self.stack.push(o);
    }

    fn c_swap(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let end: usize = self.stack.len() - 1;

        self.stack.swap(end, end - 1);
    }

    fn c_cls(&mut self, _op: &str) {
        self.stack.clear();
    }

    fn c_roll(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let o: String = self.stack.pop().unwrap(); // remove last
                                                   //
        self.stack.splice(0..0, [o]); // add as first
    }

    fn c_rot(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let o: String = self.stack.remove(0); // remove first
                                              //
        self.stack.push(o); // add as last
    }

    // ---- memory usage -------------------------------------------------------

    fn c_store_a(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        self.mem_a = self.pop_stack_float();
    }

    fn c_push_a(&mut self, _op: &str) {
        self.stack.push(self.mem_a.to_string());
    }

    fn c_store_b(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        self.mem_b = self.pop_stack_float();
    }

    fn c_push_b(&mut self, _op: &str) {
        self.stack.push(self.mem_b.to_string());
    }

    fn c_store_c(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        self.mem_c = self.pop_stack_float();
    }

    fn c_push_c(&mut self, _op: &str) {
        self.stack.push(self.mem_c.to_string());
    }

    // ---- math operations ----------------------------------------------------

    fn c_add(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a + b).to_string());
    }

    fn c_add_all(&mut self, op: &str) {
        while self.stack.len() > 1 {
            self.c_add(op);
        }
    }

    fn c_add_one(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a + 1.0).to_string());
    }

    fn c_sub(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a - b).to_string());
    }

    fn c_sub_one(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a - 1.0).to_string());
    }

    fn c_mult(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a * b).to_string());
    }

    fn c_mult_all(&mut self, op: &str) {
        while self.stack.len() > 1 {
            self.c_mult(op);
        }
    }

    fn c_div(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a / b).to_string());
    }

    fn c_chs(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((-1.0 * a).to_string());
    }

    fn c_abs(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.abs()).to_string());
    }

    fn c_round(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.round()).to_string());
    }

    fn c_inv(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((1.0 / a).to_string());
    }

    fn c_sqrt(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.sqrt()).to_string());
    }

    fn c_throot(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a.powf(1.0 / b)).to_string());
    }

    fn c_proot(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 3, op);

        let c: f64 = self.pop_stack_float();
        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        if (b * b - 4.0 * a * c) < 0.0 {
            self.stack
                .push((-1.0 * b / (2.0 * a)).to_string()); // r_1 real
            self.stack
                .push(((4.0 * a * c - b * b).sqrt() / (2.0 * a)).to_string()); // r_1 imag
            self.stack
                .push((-1.0 * b / (2.0 * a)).to_string()); // r_2 real
            self.stack
                .push((-1.0 * (4.0 * a * c - b * b).sqrt() / (2.0 * a)).to_string());
        // r_2 imag
        } else {
            self.stack
                .push((-1.0 * b + (b * b - 4.0 * a * c).sqrt() / (2.0 * a)).to_string()); // r_1 real
            self.stack
                .push(0.0.to_string()); // r_1 imag
            self.stack
                .push((-1.0 * b - (b * b - 4.0 * a * c).sqrt() / (2.0 * a)).to_string()); // r_2 real
            self.stack
                .push(0.0.to_string()); // r_2 imag
        }
    }

    fn c_exp(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a.powf(b)).to_string());
    }

    fn c_mod(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a % b).to_string());
    }

    fn c_fact(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((Interpreter::factorial(a)).to_string());
    }

    fn c_gcd(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: u64 = self.pop_stack_uint();
        let a: u64 = self.pop_stack_uint();

        self.stack.push(Interpreter::gcd(a, b).to_string());
    }

    fn c_pi(&mut self, _op: &str) {
        self.stack.push(std::f64::consts::PI.to_string());
    }

    fn c_euler(&mut self, _op: &str) {
        self.stack.push(std::f64::consts::E.to_string());
    }

    fn c_degrad(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.to_radians()).to_string());
    }

    fn c_raddeg(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.to_degrees()).to_string());
    }

    fn c_sin(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.sin()).to_string());
    }

    fn c_asin(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.asin()).to_string());
    }

    fn c_cos(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.cos()).to_string());
    }

    fn c_acos(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.acos()).to_string());
    }

    fn c_tan(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.tan()).to_string());
    }

    fn c_atan(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.atan()).to_string());
    }

    fn c_log10(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.log10()).to_string());
    }

    fn c_log2(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.log2()).to_string());
    }

    fn c_logn(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a.log(b)).to_string());
    }

    fn c_ln(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.ln()).to_string());
    }

    fn c_max(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a.max(b)).to_string());
    }

    fn c_max_all(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let mut m: f64 = 0.0;
        while !self.stack.is_empty() {
            m = m.max(self.pop_stack_float());
        }

        self.stack.push(m.to_string());
    }

    fn c_min(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a.min(b)).to_string());
    }

    fn c_min_all(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let mut m: f64 = f64::MAX;
        while !self.stack.is_empty() {
            m = m.min(self.pop_stack_float());
        }

        self.stack.push(m.to_string());
    }

    fn c_avg(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push(((a + b) / 2.0).to_string());
    }

    fn c_avg_all(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let mut sum: f64 = 0.0;
        let len: usize = self.stack.len();
        for _i in 0..len {
            sum += self.pop_stack_float();
        }

        self.stack.push((sum / len as f64).to_string());
    }

    fn c_rand(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: u64 = self.pop_stack_uint();
        let num: f64 = (a as f64 * rand::random::<f64>()).floor();

        self.stack.push(num.to_string());
    }

    // -- conversions ----------------------------------------------------------

    fn c_dechex(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: u64 = self.pop_stack_uint();

        self.stack.push(format!("{:x}", a));
    }

    fn c_hexdec(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a = self.pop_stack_int_from_hex();

        self.stack.push(a.to_string());
    }

    fn c_decbin(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: u64 = self.pop_stack_uint();

        self.stack.push(format!("{:b}", a));
    }

    fn c_bindec(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a = self.pop_stack_int_from_bin();

        self.stack.push(a.to_string());
    }

    fn c_binhex(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a = self.pop_stack_int_from_bin();

        self.stack.push(format!("{:x}", a));
    }

    fn c_hexbin(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a = self.pop_stack_int_from_hex();

        self.stack.push(format!("{:b}", a));
    }

    fn c_celfah(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a = self.pop_stack_float();

        self.stack.push((a * 9.0 / 5.0 + 32.0).to_string());
    }

    fn c_fahcel(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a = self.pop_stack_float();

        self.stack.push(((a - 32.0) * 5.0 / 9.0).to_string());
    }

    fn c_mikm(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a = self.pop_stack_float();

        self.stack.push((a * 1.609344).to_string());
    }

    fn c_kmmi(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a = self.pop_stack_float();

        self.stack.push((a / 1.609344).to_string());
    }

    fn c_hexrgb(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let she: String = self.stack.pop().unwrap();

        if she.len() < 5 {
            eprintln!(
                "  {}: argument too short [{}] is not of sufficient length",
                color_red_bold("error"),
                color_blue_coffee_bold(she.as_str()),
            );
            std::process::exit(99);
        }

        let rsh: String = she[..2].to_string();
        let gsh: String = she[2..4].to_string();
        let bsh: String = she[4..].to_string();

        let r: i64 = i64::from_str_radix(&rsh, 16).unwrap();
        let g: i64 = i64::from_str_radix(&gsh, 16).unwrap();
        let b: i64 = i64::from_str_radix(&bsh, 16).unwrap();

        self.stack.push(r.to_string());
        self.stack.push(g.to_string());
        self.stack.push(b.to_string());
    }

    fn c_rgbhex(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 3, op);

        let b: u64 = self.pop_stack_uint();
        let g: u64 = self.pop_stack_uint();
        let r: u64 = self.pop_stack_uint();

        self.stack.push(format!("{:02x}{:02x}{:02x}", r, g, b));
    }

    fn c_tip(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a * 0.15).to_string());
    }

    fn c_tip_plus(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a * 0.20).to_string());
    }

    fn c_conv_const(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a * self.config.conv_const).to_string());
    }

    // -- control flow ---------------------------------------------------------

    fn c_function(&mut self, _op: &str) {
        // get function name
        let fn_name: String = self.ops.remove(0);

        // create new function instance and assign function name
        self.fns.push(Function {
            name: fn_name,
            fops: Vec::new(),
        });
        let fpos: usize = self.fns.len() - 1; // added function position in function vector

        // build function operations list
        while self.ops[0] != ")" {
            self.fns[fpos].fops.push(self.ops.remove(0));
        }
        self.ops.remove(0); // remove ")"
    }

    fn c_ifeq(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b = self.pop_stack_float();
        let a = self.pop_stack_float();

        let mut ifops: Vec<String> = Vec::new();

        let mut depth: usize = 0;

        if a == b {
            // execute _if_ condition
            // store list of operations until 'else' or 'fi'
            while (depth > 0) || ((self.ops[0] != "fi") && (self.ops[0] != "else")) {
                match self.ops[0].as_str() {
                    "ifeq" => depth += 1, // increase depth
                    "fi" => depth -= 1,   // decrease depth
                    _ => (),
                }
                ifops.push(self.ops.remove(0));
            }
            self.remove_ops_fi();
        } else {
            // execute _else_ condition ( if one exists )

            // remove ops prior to 'else' or 'fi'
            while (depth > 0) || ((self.ops[0] != "fi") && (self.ops[0] != "else")) {
                match self.ops[0].as_str() {
                    "ifeq" => depth += 1, // increase depth
                    "fi" => depth -= 1,   // decrease depth
                    _ => (),
                }
                self.ops.remove(0);
            }

            if self.ops[0] == "else" {
                self.ops.remove(0); // remove "else"
                while self.ops[0] != "fi" {
                    // store list of operations after 'else'
                    ifops.push(self.ops.remove(0));
                }
            }
            self.ops.remove(0); // remove "fi"
        }

        // add if ops to front of operations list
        for o in ifops.iter().rev() {
            self.ops.insert(0, o.to_string());
        }
    }

    fn remove_ops_fi(&mut self) {
        let end_op: &str = "fi";

        let mut depth: usize = 0;

        while (depth > 0) || (self.ops[0] != end_op) {
            match self.ops[0].as_str() {
                "ifeq" => depth += 1, // increase depth
                "fi" => depth -= 1,   // decrease depth
                _ => (),
            }
            self.ops.remove(0);
        }
        self.ops.remove(0); // remove end_op
    }

    fn c_comment(&mut self, _op: &str) {
        let mut nested: usize = 0;

        while !self.ops.is_empty() {
            let op = self.ops.remove(0);
            match op.as_str() {
                "<" => {
                    nested += 1;
                }
                ">" => {
                    if nested == 0 {
                        return;
                    } else {
                        nested -= 1;
                    }
                }
                _ => (),
            }
        }
    }

    // support functions -------------------------------------------------------

    fn is_user_function(&self, op: &str) -> Option<usize> {
        // is operator a user defined function?
        if !self.fns.is_empty() {
            for i in 0..self.fns.len() {
                if self.fns[i].name == op {
                    return Some(i);
                }
            }
        }
        None
    }

    // factorial
    fn factorial(o: f64) -> f64 {
        let n = o.floor();

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

    // read configuration file
    fn read_config(&mut self, filename: &str) {
        /*
        println!(
            "  reading configuration file [{}]",
            color_blue_coffee_bold(filename),
        );
        */

        // read file contents
        let filename: String = filename.to_string();

        let home_folder: String = home::home_dir().unwrap().to_str().unwrap().to_string();

        let config_filename: String = format!("{}/{}", home_folder, filename);

        let path: &Path = Path::new(&config_filename);

        let file_contents = fs::read_to_string(&path);
        if let Err(ref _error) = file_contents {
            // do nothing - default config will be used
        } else {
            let config_file_toml: String = file_contents.unwrap();

            // deserialize configuration TOML and update configuration
            let config: Config = toml::from_str(config_file_toml.as_str()).unwrap();
            self.config = config;
        }
    }
}

fn show_help() {
    println!();
    println!("{}", color_white_bold("COMP"));
    println!(
        "    {} {} {} {}",
        color_grey_mouse("comp"),
        color_grey_mouse(".."),
        color_white_bold("command interpreter"),
        color_grey_mouse(env!("CARGO_PKG_VERSION")),
    );
    println!();
    println!("{}", color_white_bold("USAGE"));
    println!(
        "    {} {} {}",
        color_grey_mouse("comp"),
        color_white_bold("[OPTIONS]"),
        color_blue_coffee_bold("<list>"),
    );
    println!(
        "    {} {} {}",
        color_grey_mouse("comp"),
        color_yellow_canary_bold("-f"),
        color_blue_coffee_bold("<path>"),
    );
    println!();
    println!("{}", color_white_bold("OPTIONS"));
    println!(
        "        {}      show version",
        color_yellow_canary_bold("--version"),
    );
    println!(
        "    {}{} {}         read from file at the specified path",
        color_yellow_canary_bold("-f"),
        color_white_bold(","),
        color_yellow_canary_bold("--file"),
    );
    println!(
        "        {}         show help information",
        color_yellow_canary_bold("--help"),
    );
    println!();
    println!("{}", color_white_bold("DESCRIPTION"));
    println!(
        "The comp interpreter takes a {} sequence of (postfix) operations as \
    command line arguments or a {} argument that specifies the path to a file \
    containing a list of operations. Each operation is either a command ({}) \
    or a {}. The available commands are listed below.",
        color_blue_coffee_bold("<list>"),
        color_blue_coffee_bold("<path>"),
        color_green_eggs_bold("symbol"),
        color_blue_smurf_bold("value"),
    );
    println!();
    println!(
        "    Usage Guide:   {}",
        color_grey_mouse("https://github.com/usefulmove/comp/blob/main/USAGE.md"),
    );
    println!(
        "    Repository:    {}",
        color_grey_mouse("https://github.com/usefulmove/comp#readme"),
    );
    println!();
    println!("{}", color_white_bold("EXAMPLES"));
    println!(
        "    {} {} {}                  {}",
        color_grey_mouse("comp"),
        color_blue_smurf_bold("1 2"),
        color_green_eggs_bold("+"),
        color_white_bold("add 1 and 2"),
    );
    println!(
        "    {} {} {}                  {}",
        color_grey_mouse("comp"),
        color_blue_smurf_bold("5 2"),
        color_green_eggs_bold("/"),
        color_white_bold("divide 5 by 2"),
    );
    println!(
        "    {} {} {} {} {}      {}",
        color_grey_mouse("comp"),
        color_blue_smurf_bold("3"),
        color_green_eggs_bold("dup x"),
        color_blue_smurf_bold("4"),
        color_green_eggs_bold("dup x +"),
        color_white_bold("sum of the squares of 3 and 4"),
    );
    println!();
    println!("{}", color_white_bold("COMMANDS"));
    println!("{}", color_grey_mouse(CMDS));
    println!();
}

fn show_version() {
    let version: &str = env!("CARGO_PKG_VERSION");
    println!(
        "  {} {}{}",
        color_grey_mouse("comp"),
        color_blue_smurf_bold(version),
        color_white_bold(RELEASE_STATE),
    );
}

fn print_stack(stack: &mut Vec<String>) {
    while !stack.is_empty() {
        match stack.len() {
            1 => {
                println!(
                    "  {}",
                    // format top element
                    color_green_eggs_bold(stack.remove(0).as_str()),
                )
            }
            _ => {
                println!(
                    "  {}",
                    // format other elements
                    color_blue_smurf_bold(stack.remove(0).as_str()),
                )
            }
        }
    }
}

fn color_red_bold(message: &str) -> ColoredString {
    message.truecolor(241, 95, 78).bold()
}

fn _color_orange_sherbet_bold(message: &str) -> ColoredString {
    message.truecolor(239, 157, 110).bold()
}

fn color_yellow_canary_bold(message: &str) -> ColoredString {
    message.truecolor(255, 252, 103).bold()
}

fn color_green_eggs_bold(message: &str) -> ColoredString {
    message.truecolor(135, 255, 175).bold()
}

fn color_blue_smurf_bold(message: &str) -> ColoredString {
    message.truecolor(0, 128, 255).bold()
}

fn color_blue_coffee_bold(message: &str) -> ColoredString {
    message.truecolor(0, 192, 255).bold()
}

fn color_white_bold(message: &str) -> ColoredString {
    message.truecolor(255, 255, 255).bold()
}

fn color_grey_mouse(message: &str) -> ColoredString {
    message.truecolor(155, 155, 155)
}

fn _color_charcoal_creamy_bold(message: &str) -> ColoredString {
    message.truecolor(38, 38, 38).bold()
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
#[path = "../test/comp.test.rs"]
mod comp_tests;
