use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::num::{ParseFloatError, ParseIntError};

use crate::poc;

pub struct Interpreter {
    pub stack: Vec<String>,
    pub mem_a: f64,
    pub mem_b: f64,
    pub mem_c: f64,
    pub ops: Vec<String>,
    pub fns: Vec<Function>,
    pub cmap: HashMap<String, fn(&mut Interpreter, &str)>,
    pub config: Config,
}

impl Interpreter {
    // constructor
    pub fn new() -> Interpreter {
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
    pub fn process_ops(&mut self) {
        while !self.ops.is_empty() {
            let operation: String = self.ops.remove(0); // pop first operation
            self.process_node(&operation);
        }
    }

    // add native command to interpreter
    pub fn compose_native(&mut self, name: &str, func: fn(&mut Interpreter, &str)) {
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
        self.compose_native("g", Interpreter::c_accelg); // standard acceleration due to gravity (m/s2)
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
        self.compose_native("ifeq", Interpreter::c_ifeq); // ifequal .. else
        self.compose_native("<", Interpreter::c_comment); // function comment
        self.compose_native("pln", Interpreter::c_println); // print line
        /* conversion */
        self.compose_native("dec_hex", Interpreter::c_dechex); // decimal to hexadecimal
        self.compose_native("hex_dec", Interpreter::c_hexdec); // hexadecimal to decimal
        self.compose_native("dec_bin", Interpreter::c_decbin); // decimal to binary
        self.compose_native("bin_dec", Interpreter::c_bindec); // binary to decimal
        self.compose_native("bin_hex", Interpreter::c_binhex); // binary to hexadecimal
        self.compose_native("hex_bin", Interpreter::c_hexbin); // hexadecimal to binary
        self.compose_native("c_f", Interpreter::c_celfah); // Celsius to Fahrenheit
        self.compose_native("C_F", Interpreter::c_celfah);
        self.compose_native("f_c", Interpreter::c_fahcel); // Fahrenheit to Celsius
        self.compose_native("F_C", Interpreter::c_fahcel);
        self.compose_native("mi_km", Interpreter::c_mikm); // miles to kilometers
        self.compose_native("km_mi", Interpreter::c_kmmi); // kilometers to miles
        self.compose_native("ft_m", Interpreter::c_ftm); // feet to meters
        self.compose_native("m_ft", Interpreter::c_mft); // meters to feet
        self.compose_native("hex_rgb", Interpreter::c_hexrgb); // hexadecimal string to RGB
        self.compose_native("rgb_hex", Interpreter::c_rgbhex); // RGB to hexadecimal string
        self.compose_native("tip", Interpreter::c_tip); // calculate tip
        self.compose_native("tip+", Interpreter::c_tip_plus); // calculate better tip
        self.compose_native("a_b", Interpreter::c_conv_const); // apply convert constant
        /* rgb colors */
        self.compose_native("rgb", Interpreter::c_rgb); // show RGB color
        self.compose_native("rgbh", Interpreter::c_rgbh); // show RGB color (hexadecimal)
    }

    pub fn process_node(&mut self, op: &str) {
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
    pub fn pop_stack_string(&mut self) -> String {
        self.stack.pop().unwrap()
    }

    pub fn pop_stack_float(&mut self) -> f64 {
        let element: String = self.stack.pop().unwrap();
        match self.parse_float(&element) {
            Ok(val) => val, // parse success
            Err(_error) => {
                // parse fail
                eprintln!(
                    "  {}: unknown expression [{}] is not a recognized operation \
                    or valid value (f)",
                    poc::color_red_bold("error"),
                   poc::color_blue_coffee_bold(&element),
                );
                std::process::exit(99);
            }
        }
    }

    pub fn pop_stack_uint(&mut self) -> u64 {
        let element: String = self.stack.pop().unwrap();
        match self.parse_uint(&element) {
            Ok(val) => val, // parse success
            Err(_error) => {
                // parse fail
                eprintln!(
                    "  {}: unknown expression [{}] is not a recognized operation \
                    or valid value (u)",
                   poc::color_red_bold("error"),
                   poc::color_blue_coffee_bold(&element),
                );
                std::process::exit(99);
            }
        }
    }

    pub fn pop_stack_uint8(&mut self) -> u8 {
        let element: String = self.stack.pop().unwrap();
        match self.parse_uint8(&element) {
            Ok(val) => val, // parse success
            Err(_error) => {
                // parse fail
                eprintln!(
                    "  {}: unknown expression [{}] is not a recognized operation \
                    or valid value (u)",
                   poc::color_red_bold("error"),
                   poc::color_blue_coffee_bold(&element),
                );
                std::process::exit(99);
            }
        }
    }

    pub fn pop_stack_int_from_hex(&mut self) -> i64 {
        let element: String = self.stack.pop().unwrap();

        match i64::from_str_radix(&element, 16) {
            Ok(val) => val, // parse success
            Err(_error) => {
                // parse fail
                eprintln!(
                    "  {}: unknown expression [{}] is not a recognized operation \
                    or valid value (i_h)",
                   poc::color_red_bold("error"),
                   poc::color_blue_coffee_bold(&element),
                );
                std::process::exit(99);
            }
        }
    }

    pub fn pop_stack_u8_from_hex(&mut self) -> u8 {
        let element: String = self.stack.pop().unwrap();

        match u8::from_str_radix(&element, 16) {
            Ok(val) => val, // parse success
            Err(_error) => {
                // parse fail
                eprintln!(
                    "  {}: unknown expression [{}] is not a recognized operation \
                    or valid value (i_h)",
                   poc::color_red_bold("error"),
                   poc::color_blue_coffee_bold(&element),
                );
                std::process::exit(99);
            }
        }
    }

    pub fn pop_stack_int_from_bin(&mut self) -> i64 {
        let element: String = self.stack.pop().unwrap();

        match i64::from_str_radix(&element, 2) {
            Ok(val) => val, // parse success
            Err(_error) => {
                // parse fail
                eprintln!(
                    "  {}: unknown expression [{}] is not a recognized operation \
                    or valid value (i_b)",
                   poc::color_red_bold("error"),
                   poc::color_blue_coffee_bold(&element),
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

    fn parse_uint8(&self, op: &str) -> Result<u8, ParseIntError> {
        let value: u8 = op.parse::<u8>()?;
        Ok(value)
    }
    // -------------------------------------------------------------------------

    // confirm stack depth
    fn check_stack_error(&self, min_depth: usize, command: &str) {
        if self.stack.len() < min_depth {
            eprintln!(
                "  {}: [{}] operation called without at least {min_depth} \
                element(s) on stack",
               poc::color_red_bold("error"),
               poc::color_blue_coffee_bold(command),
            );
            std::process::exit(99);
        }
    }

    // command functions -------------------------------------------------------
    // ---- stack manipulation -------------------------------------------------

    pub fn c_drop(&mut self, op: &str) {
        if !self.stack.is_empty() {
            self.stack.pop();
        } else {
            eprintln!(
                "  {}: [{}] operation called on empty stack",
               poc::color_yellow_canary_bold("warning"),
               poc::color_blue_coffee_bold(op),
            );
            // do not stop execution
        }
    }

    pub fn c_dup(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let end: usize = self.stack.len() - 1;

        self.stack.push(self.stack[end].clone()); // remove last
    }

    pub fn c_swap(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let end: usize = self.stack.len() - 1;

        self.stack.swap(end, end - 1);
    }

    pub fn c_cls(&mut self, _op: &str) {
        self.stack.clear();
    }

    pub fn c_roll(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let o: String = self.stack.pop().unwrap(); // remove last
                                                   //
        self.stack.splice(0..0, [o]); // add as first
    }

    pub fn c_rot(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let o: String = self.stack.remove(0); // remove first
                                              //
        self.stack.push(o); // add as last
    }

    // ---- memory usage -------------------------------------------------------

    pub fn c_store_a(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        self.mem_a = self.pop_stack_float();
    }

    pub fn c_push_a(&mut self, _op: &str) {
        self.stack.push(self.mem_a.to_string());
    }

    pub fn c_store_b(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        self.mem_b = self.pop_stack_float();
    }

    pub fn c_push_b(&mut self, _op: &str) {
        self.stack.push(self.mem_b.to_string());
    }

    pub fn c_store_c(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        self.mem_c = self.pop_stack_float();
    }

    pub fn c_push_c(&mut self, _op: &str) {
        self.stack.push(self.mem_c.to_string());
    }

    // ---- math operations ----------------------------------------------------

    pub fn c_add(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a + b).to_string());
    }

    pub fn c_add_all(&mut self, op: &str) {
        while self.stack.len() > 1 {
            self.c_add(op);
        }
    }

    pub fn c_add_one(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a + 1.0).to_string());
    }

    pub fn c_sub(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a - b).to_string());
    }

    pub fn c_sub_one(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a - 1.0).to_string());
    }

    pub fn c_mult(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a * b).to_string());
    }

    pub fn c_mult_all(&mut self, op: &str) {
        while self.stack.len() > 1 {
            self.c_mult(op);
        }
    }

    pub fn c_div(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a / b).to_string());
    }

    pub fn c_chs(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((-1.0 * a).to_string());
    }

    pub fn c_abs(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.abs()).to_string());
    }

    pub fn c_round(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.round()).to_string());
    }

    pub fn c_inv(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((1.0 / a).to_string());
    }

    pub fn c_sqrt(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.sqrt()).to_string());
    }

    pub fn c_throot(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a.powf(1.0 / b)).to_string());
    }

    pub fn c_proot(&mut self, op: &str) {
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

    pub fn c_exp(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a.powf(b)).to_string());
    }

    pub fn c_mod(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a % b).to_string());
    }

    pub fn c_fact(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((Interpreter::factorial(a)).to_string());
    }

    pub fn c_gcd(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: u64 = self.pop_stack_uint();
        let a: u64 = self.pop_stack_uint();

        self.stack.push(Interpreter::gcd(a, b).to_string());
    }

    pub fn c_pi(&mut self, _op: &str) {
        self.stack.push(std::f64::consts::PI.to_string());
    }

    pub fn c_euler(&mut self, _op: &str) {
        self.stack.push(std::f64::consts::E.to_string());
    }

    pub fn c_accelg(&mut self, _op: &str) {
        self.stack.push(9.80665.to_string());
    }

    pub fn c_degrad(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.to_radians()).to_string());
    }

    pub fn c_raddeg(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.to_degrees()).to_string());
    }

    pub fn c_sin(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.sin()).to_string());
    }

    pub fn c_asin(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.asin()).to_string());
    }

    pub fn c_cos(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.cos()).to_string());
    }

    pub fn c_acos(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.acos()).to_string());
    }

    pub fn c_tan(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.tan()).to_string());
    }

    pub fn c_atan(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.atan()).to_string());
    }

    pub fn c_log10(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.log10()).to_string());
    }

    pub fn c_log2(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.log2()).to_string());
    }

    pub fn c_logn(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a.log(b)).to_string());
    }

    pub fn c_ln(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.ln()).to_string());
    }

    pub fn c_max(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a.max(b)).to_string());
    }

    pub fn c_max_all(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let mut m: f64 = 0.0;
        while !self.stack.is_empty() {
            m = m.max(self.pop_stack_float());
        }

        self.stack.push(m.to_string());
    }

    pub fn c_min(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a.min(b)).to_string());
    }

    pub fn c_min_all(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let mut m: f64 = f64::MAX;
        while !self.stack.is_empty() {
            m = m.min(self.pop_stack_float());
        }

        self.stack.push(m.to_string());
    }

    pub fn c_avg(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push(((a + b) / 2.0).to_string());
    }

    pub fn c_avg_all(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 2, op);

        let mut sum: f64 = 0.0;
        let len: usize = self.stack.len();
        for _i in 0..len {
            sum += self.pop_stack_float();
        }

        self.stack.push((sum / len as f64).to_string());
    }

    pub fn c_rand(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: u64 = self.pop_stack_uint();
        let num: f64 = (a as f64 * rand::random::<f64>()).floor();

        self.stack.push(num.to_string());
    }

    // -- conversions ----------------------------------------------------------

    pub fn c_dechex(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: u64 = self.pop_stack_uint();

        self.stack.push(format!("{:x}", a));
    }

    pub fn c_hexdec(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a = self.pop_stack_int_from_hex();

        self.stack.push(a.to_string());
    }

    pub fn c_decbin(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: u64 = self.pop_stack_uint();

        self.stack.push(format!("{:b}", a));
    }

    pub fn c_bindec(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a = self.pop_stack_int_from_bin();

        self.stack.push(a.to_string());
    }

    pub fn c_binhex(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a = self.pop_stack_int_from_bin();

        self.stack.push(format!("{:x}", a));
    }

    pub fn c_hexbin(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a = self.pop_stack_int_from_hex();

        self.stack.push(format!("{:b}", a));
    }

    pub fn c_celfah(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a = self.pop_stack_float();

        self.stack.push((a * 9.0 / 5.0 + 32.0).to_string());
    }

    pub fn c_fahcel(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a = self.pop_stack_float();

        self.stack.push(((a - 32.0) * 5.0 / 9.0).to_string());
    }

    pub fn c_mikm(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a = self.pop_stack_float();

        self.stack.push((a * 1.609344).to_string());
    }

    pub fn c_kmmi(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a = self.pop_stack_float();

        self.stack.push((a / 1.609344).to_string());
    }

    pub fn c_ftm(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a = self.pop_stack_float();

        self.stack.push((a / 3.281).to_string());
    }

    pub fn c_mft(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a = self.pop_stack_float();

        self.stack.push((a * 3.281).to_string());
    }

    pub fn c_hexrgb(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let she: String = self.stack.pop().unwrap();

        if she.len() < 5 {
            eprintln!(
                "  {}: argument too short [{}] is not of sufficient length",
               poc::color_red_bold("error"),
               poc::color_blue_coffee_bold(&she),
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

    pub fn c_rgbhex(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 3, op);

        let b: u64 = self.pop_stack_uint();
        let g: u64 = self.pop_stack_uint();
        let r: u64 = self.pop_stack_uint();

        self.stack.push(format!("{:02x}{:02x}{:02x}", r, g, b));
    }

    pub fn c_tip(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a * 0.15).to_string());
    }

    pub fn c_tip_plus(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a * 0.20).to_string());
    }

    pub fn c_conv_const(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a * self.config.conversion_constant).to_string());
    }

    pub fn c_rgb(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 3, op);

        let b: u8 = self.pop_stack_uint8();
        let g: u8 = self.pop_stack_uint8();
        let r: u8 = self.pop_stack_uint8();

        self.stack.push(poc::format_rgb_shadow(r, g, b));
        self.stack.push(poc::format_rgb_hex(r, g, b));
    }

    pub fn c_rgbh(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 3, op);

        let b: u8 = self.pop_stack_u8_from_hex();
        let g: u8 = self.pop_stack_u8_from_hex();
        let r: u8 = self.pop_stack_u8_from_hex();

        self.stack.push(poc::format_rgb_shadow(r, g, b));
        self.stack.push(poc::format_rgb_hex(r, g, b));
    }

    // -- control flow ---------------------------------------------------------

    pub fn c_function(&mut self, _op: &str) {
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

    pub fn c_ifeq(&mut self, op: &str) {
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

    pub fn c_comment(&mut self, _op: &str) {
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

    pub fn c_println(&mut self, op: &str) {
        Interpreter::check_stack_error(self, 1, op);

        println!("{}", self.pop_stack_string());
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
    pub fn factorial(o: f64) -> f64 {
        let n = o.floor();

        if n < 2.0 {
            1.0
        } else {
            n * Interpreter::factorial(n - 1.0)
        }
    }

    // greatest common divisor
    pub fn gcd(a: u64, b: u64) -> u64 {
        if b != 0 {
            Interpreter::gcd(b, a % b)
        } else {
            a
        }
    }

    // read configuration file from home folder
    pub fn read_config(&mut self, filename: &str) {
        /*
        println!(
            "  reading configuration file [{}]",
            poc::color_blue_coffee_bold(filename),
        );
        */

        // read file contents
        let filename: String = filename.to_string();

        //let home_folder: String = home::home_dir().unwrap().to_str().unwrap().to_string();
        let home_folder: String = match home::home_dir() {
            Some(dir) => dir.to_str().unwrap().to_string(),
            _ => "".to_string(),
        };

        let config_filename: String = format!("{}/{}", home_folder, filename);

        let path: &Path = Path::new(&config_filename);

        let file_contents = fs::read_to_string(&path);
        if let Err(ref _error) = file_contents {
            // do nothing - default config will be used
        } else {
            // read file successfully
            let config_file_toml: String = file_contents.unwrap();

            // deserialize configuration TOML and update configuration
            let res: Result<Config, toml::de::Error> = toml::from_str(&config_file_toml);
            let cfg: Config = match res {
                Ok(c) => c,
                Err(_error) => {
                    // parse fail
                    eprintln!(
                        "  {}: configuration file [{}] (ignored) is corrupt or is incorrectly constructed",
                       poc::color_yellow_canary_bold("warning"),
                       poc::color_blue_smurf_bold("conf.toml"),
                    );
                    Config::new()
                }
            };

            self.config = cfg;
        }
    }
}

pub struct Function {
    name: String,
    fops: Vec<String>,
}

#[derive(Deserialize)]
pub struct Config {
    pub show_stack_level: bool,
    pub conversion_constant: f64,
    pub monochrome: bool,
}

impl Config {
    // constructor
    fn new() -> Config {
        Config {
            show_stack_level: true,
            conversion_constant: 1.0,
            monochrome: false,
        }
    }
}