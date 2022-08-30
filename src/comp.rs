use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::num::{ParseFloatError, ParseIntError};

pub struct Interpreter {
    pub stack: Vec<String>,
    pub mem: HashMap<String, String>,
    pub mem_a: f64,
    pub mem_b: f64,
    pub mem_c: f64,
    pub ops: Vec<String>,
    pub fns: Vec<Function>,
    pub cmdmap: HashMap<String, fn(&mut Interpreter, &str)>,
    pub config: Config,
    pub theme: coq::Theme,
}

impl Interpreter {
    // constructor
    pub fn new() -> Self {
        let mut cint = Self {
            stack: Vec::new(),
            mem: HashMap::new(), // local interpreter memory
            mem_a: 0., // local interpreter memory
            mem_b: 0.,
            mem_c: 0.,
            ops: Vec::new(), // operations list
            fns: Vec::new(), // user-defined functions
            cmdmap: HashMap::new(), // interpreter command map
            config: Config::new(), // configuration object
            theme: coq::Theme::new(), // output format theme
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
    pub fn compose_native(&mut self, name: &str, func: fn(&mut Self, &str)) {
        self.cmdmap.insert(name.to_string(), func);
    }

    fn init(&mut self) {
        /* stack manipulation */
        self.compose_native("drop", Self::c_drop); // drop
        self.compose_native("dup", Self::c_dup); // duplicate
        self.compose_native("swap", Self::c_swap); // swap x and y
        self.compose_native("cls", Self::c_cls); // clear stack
        self.compose_native("clr", Self::c_cls); // clear stack
        self.compose_native("roll", Self::c_roll); // roll stack
        self.compose_native("rot", Self::c_rot); // rotate stack (reverse direction from roll)
        self.compose_native("map", Self::c_map); // map annonymous function to stack
        self.compose_native("..", Self::c_range); // add range of numbers to stack
        /* memory usage */
        self.compose_native("store", Self::c_store); // store (pop value off stack and store in specified memory)
        self.compose_native("mem", Self::c_store);
        self.compose_native("sa", Self::c_store_a); // store (pop value off stack and store)
        self.compose_native("_a", Self::c_push_a); // retrieve (push stored value onto the stack)
        self.compose_native("sb", Self::c_store_b); // store
        self.compose_native("_b", Self::c_push_b); // retrieve
        self.compose_native("sc", Self::c_store_c); // store
        self.compose_native("_c", Self::c_push_c); // retrieve
        /* math operations */
        self.compose_native("+", Self::c_add); // add
        self.compose_native("+_", Self::c_add_all); // add all
        self.compose_native("++", Self::c_add_one); // add one
        self.compose_native("-", Self::c_sub); // subtract
        self.compose_native("--", Self::c_sub_one); // subtract one
        self.compose_native("x", Self::c_mult); // multiply
        self.compose_native("x_", Self::c_mult_all); // multiply all
        self.compose_native("/", Self::c_div); // divide
        self.compose_native("chs", Self::c_chs); // change sign
        self.compose_native("abs", Self::c_abs); // absolute value
        self.compose_native("round", Self::c_round); // round
        self.compose_native("int", Self::c_round);
        self.compose_native("floor", Self::c_floor); // floor
        self.compose_native("ceil", Self::c_ceiling); // ceiling
        self.compose_native("pos", Self::c_pos);
        self.compose_native("inv", Self::c_inv); // invert (1/x)
        self.compose_native("sqrt", Self::c_sqrt); // square root
        self.compose_native("throot", Self::c_throot); // nth root
        self.compose_native("proot", Self::c_proot); // find principal roots
        self.compose_native("^", Self::c_exp); // exponentiation
        self.compose_native("exp", Self::c_exp);
        self.compose_native("%", Self::c_mod); // modulus
        self.compose_native("mod", Self::c_mod);
        self.compose_native("!", Self::c_fact); // factorial
        self.compose_native("gcd", Self::c_gcd); // greatest common divisor
        self.compose_native("pi", Self::c_pi); // pi
        self.compose_native("e", Self::c_euler); // Euler's constant
        self.compose_native("g", Self::c_accelg); // standard acceleration due to gravity (m/s2)
        self.compose_native("deg_rad", Self::c_degrad); // degrees to radians
        self.compose_native("rad_deg", Self::c_raddeg); // radians to degrees
        self.compose_native("sin", Self::c_sin); // sine
        self.compose_native("asin", Self::c_asin); // arcsine
        self.compose_native("cos", Self::c_cos); // cosine
        self.compose_native("acos", Self::c_acos); // arccosine
        self.compose_native("tan", Self::c_tan); // tangent
        self.compose_native("atan", Self::c_atan); // arctangent
        self.compose_native("log2", Self::c_log2); // logarithm (base 2)
        self.compose_native("log", Self::c_log10); // logarithm (base 10)
        self.compose_native("log10", Self::c_log10);
        self.compose_native("logn", Self::c_logn); // logarithm (base n)
        self.compose_native("ln", Self::c_ln); // natural logarithm
        self.compose_native("rand", Self::c_rand); // random number
        self.compose_native("min", Self::c_min); // minimum
        self.compose_native("min_", Self::c_min_all); // minimum
        self.compose_native("max", Self::c_max); // maximum
        self.compose_native("max_", Self::c_max_all); // maximum all
        self.compose_native("avg", Self::c_avg); // average
        self.compose_native("avg_", Self::c_avg_all); // average all
        /* control flow */
        self.compose_native("(", Self::c_function); // function definition
        self.compose_native("[", Self::c_lambda); // anonymous function definition
        self.compose_native("ifeq", Self::c_ifeq); // ifequal .. else
        self.compose_native("{", Self::c_comment); // function comment
        self.compose_native("peek", Self::c_peek); // peek at top of stack
        /* conversion */
        self.compose_native("dec_hex", Self::c_dechex); // decimal to hexadecimal
        self.compose_native("hex_dec", Self::c_hexdec); // hexadecimal to decimal
        self.compose_native("dec_bin", Self::c_decbin); // decimal to binary
        self.compose_native("bin_dec", Self::c_bindec); // binary to decimal
        self.compose_native("bin_hex", Self::c_binhex); // binary to hexadecimal
        self.compose_native("hex_bin", Self::c_hexbin); // hexadecimal to binary
        self.compose_native("c_f", Self::c_celfah); // Celsius to Fahrenheit
        self.compose_native("C_F", Self::c_celfah);
        self.compose_native("f_c", Self::c_fahcel); // Fahrenheit to Celsius
        self.compose_native("F_C", Self::c_fahcel);
        self.compose_native("mi_km", Self::c_mikm); // miles to kilometers
        self.compose_native("km_mi", Self::c_kmmi); // kilometers to miles
        self.compose_native("ft_m", Self::c_ftm); // feet to meters
        self.compose_native("m_ft", Self::c_mft); // meters to feet
        self.compose_native("hex_rgb", Self::c_hexrgb); // hexadecimal string to RGB
        self.compose_native("rgb_hex", Self::c_rgbhex); // RGB to hexadecimal string
        self.compose_native("tip", Self::c_tip); // calculate tip
        self.compose_native("a_b", Self::c_conv_const); // apply convert constant
        /* rgb colors */
        self.compose_native("rgb", Self::c_rgb); // show RGB color
        self.compose_native("rgbh", Self::c_rgbh); // show RGB color (hexadecimal)
        /* configuration */
        self.compose_native("save_cfg", Self::c_save_config); // save configuration
        self.compose_native("show_cfg", Self::c_print_config); // show current configuration
    }

    pub fn process_node(&mut self, op: &str) {
        /* native command? */
        if self.cmdmap.contains_key(op) {
            let f = self.cmdmap[op];
            f(self, op); // execute command function
            return;
        }

        /* user-defined function? */
        match self.is_user_function(op) {
            Some(index) => {
                // user-defined function - copy user function ops (fops) into main ops
                for i in (0..self.fns[index].fops.len()).rev() {
                    let fop: String = self.fns[index].fops[i].clone();
                    self.ops.insert(0, fop);
                }
                return;
            }
            None => (),
        }

        /* user memory */
        match self.is_user_memory(op) {
            Some(value) => {
                // user-defined memory - push value onto stack
                self.ops.insert(0, value.clone());
                return;
            }
            None => (),
        }

        /* neither native command nor user-defined function */

        // push value onto stack
        self.stack.push(op.to_string());
    }

    /* pop from stack helper functions */
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
                    self.theme.color_rgb("error", &self.theme.red_bold),
                    self.theme.color_rgb(&element, &self.theme.blue_coffee_bold),
                );
                std::process::exit(exitcode::USAGE);
            }
        }
    }

    pub fn pop_stack_float_pos(&mut self) -> f64 {
        let element: String = self.stack.pop().unwrap();
        match self.parse_float(&element) {
            Ok(mut val) => { // parse success
                if val < 0. {
                    val = 0.;
                }
                val
            }
            Err(_error) => {
                // parse fail
                eprintln!(
                    "  {}: unknown expression [{}] is not a recognized operation \
                    or valid value (f)",
                    self.theme.color_rgb("error", &self.theme.red_bold),
                    self.theme.color_rgb(&element, &self.theme.blue_coffee_bold),
                );
                std::process::exit(exitcode::USAGE);
            }
        }
    }

    pub fn _pop_stack_int(&mut self) -> i64 {
        let element: String = self.stack.pop().unwrap();
        match self._parse_int(&element) {
            Ok(val) => val, // parse success
            Err(_error) => {
                // parse fail
                eprintln!(
                    "  {}: unknown expression [{}] is not a recognized operation \
                    or valid value (u)",
                   self.theme.color_rgb("error", &self.theme.red_bold),
                   self.theme.color_rgb(&element, &self.theme.blue_coffee_bold),
                );
                std::process::exit(exitcode::USAGE);
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
                   self.theme.color_rgb("error", &self.theme.red_bold),
                   self.theme.color_rgb(&element, &self.theme.blue_coffee_bold),
                );
                std::process::exit(exitcode::USAGE);
            }
        }
    }

    pub fn _pop_stack_uint8(&mut self) -> u8 {
        let element: String = self.stack.pop().unwrap();
        match self._parse_uint8(&element) {
            Ok(val) => val, // parse success
            Err(_error) => {
                // parse fail
                eprintln!(
                    "  {}: unknown expression [{}] is not a recognized operation \
                    or valid value (u)",
                   self.theme.color_rgb("error", &self.theme.red_bold),
                   self.theme.color_rgb(&element, &self.theme.blue_coffee_bold),
                );
                std::process::exit(exitcode::USAGE);
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
                   self.theme.color_rgb("error", &self.theme.red_bold),
                   self.theme.color_rgb(&element, &self.theme.blue_coffee_bold),
                );
                std::process::exit(exitcode::USAGE);
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
                   self.theme.color_rgb("error", &self.theme.red_bold),
                   self.theme.color_rgb(&element, &self.theme.blue_coffee_bold),
                );
                std::process::exit(exitcode::USAGE);
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
                   self.theme.color_rgb("error", &self.theme.red_bold),
                   self.theme.color_rgb(&element, &self.theme.blue_coffee_bold),
                );
                std::process::exit(exitcode::USAGE);
            }
        }
    }

    fn parse_float(&self, op: &str) -> Result<f64, ParseFloatError> {
        let value: f64 = op.parse::<f64>()?;
        Ok(value)
    }

    fn _parse_int(&self, op: &str) -> Result<i64, ParseIntError> {
        let value: i64 = op.parse::<i64>()?;
        Ok(value)
    }

    fn parse_uint(&self, op: &str) -> Result<u64, ParseIntError> {
        let value: u64 = op.parse::<u64>()?;
        Ok(value)
    }

    fn _parse_uint8(&self, op: &str) -> Result<u8, ParseIntError> {
        let value: u8 = op.parse::<u8>()?;
        Ok(value)
    }

    // confirm stack depth
    fn check_stack_error(&self, min_depth: usize, command: &str) {
        if self.stack.len() < min_depth {
            eprintln!(
                "  {}: [{}] operation called without at least {min_depth} \
                element(s) on stack",
               self.theme.color_rgb("error", &self.theme.red_bold),
               self.theme.color_rgb(command, &self.theme.blue_coffee_bold),
            );
            std::process::exit(exitcode::USAGE);
        }
    }

    /* command functions ---------------------------------------------------- */
    /* ---- stack manipulation ---------------------------------------------- */

    pub fn c_drop(&mut self, op: &str) {
        if !self.stack.is_empty() {
            self.stack.pop();
        } else {
            eprintln!(
                "  {}: [{}] operation called on empty stack",
                self.theme.color_rgb("warning", &self.theme.yellow_canary_bold),
                self.theme.color_rgb(op, &self.theme.blue_coffee_bold),
            );
            // do not stop execution
        }
    }

    pub fn c_dup(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let end: usize = self.stack.len() - 1;

        self.stack.push(self.stack[end].clone()); // remove last
    }

    pub fn c_swap(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let end: usize = self.stack.len() - 1;

        self.stack.swap(end, end - 1);
    }

    pub fn c_cls(&mut self, _op: &str) {
        self.stack.clear();
    }

    pub fn c_roll(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let o: String = self.stack.pop().unwrap(); // remove last
                                                   //
        self.stack.splice(0..0, [o]); // add as first
    }

    pub fn c_rot(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let o: String = self.stack.remove(0); // remove first
                                              //
        self.stack.push(o); // add as last
    }

    pub fn c_map(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let stack_len: usize = self.stack.len();

        // add ops to execute anonymous function on each stack element
        for _ in 0..stack_len {
            self.ops.insert(0, String::from("roll"));
            self.ops.insert(0, String::from("_"));
        }
    }

    pub fn c_range(&mut self, op: &str) {
        Self::check_stack_error(self, 3, op);

        let step: f64  = self.pop_stack_float();
        let end: f64 = self.pop_stack_float();
        let start: f64 = self.pop_stack_float();

        let mut value: f64 = start;
        if end >= start {
            while value <= end {
                self.stack.push(value.to_string());
                value += step.abs();
            }
        } else {
            while value >= end {
                self.stack.push(value.to_string());
                value -= step.abs();
            }
        }

    }

    pub fn c_lambda(&mut self, _op: &str) {
        // clear existing anonymous function definition
        match self.is_user_function("_") {
            Some(index) => {
                self.fns.remove(index);
            }
            None => (),
        }

        // create new anonymous function instance
        self.fns.push(
            Function {
                name: String::from("_"),
                fops: Vec::new(),
            }
        );
        let fn_ind: usize = self.fns.len() - 1; // index of new function in function vector

        // build anonymous function operations list
        while self.ops[0] != "]" {
            self.fns[fn_ind].fops.push(self.ops.remove(0));
        }
        self.ops.remove(0); // remove "|"
    }

    /* ---- memory usage ---------------------------------------------------- */

    pub fn c_store(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let key = self.pop_stack_string();
        let val = self.pop_stack_string();

        self.mem.insert(key, val);
    }

    pub fn c_store_a(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        self.mem_a = self.pop_stack_float();
    }

    pub fn c_push_a(&mut self, _op: &str) {
        self.stack.push(self.mem_a.to_string());
    }

    pub fn c_store_b(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        self.mem_b = self.pop_stack_float();
    }

    pub fn c_push_b(&mut self, _op: &str) {
        self.stack.push(self.mem_b.to_string());
    }

    pub fn c_store_c(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        self.mem_c = self.pop_stack_float();
    }

    pub fn c_push_c(&mut self, _op: &str) {
        self.stack.push(self.mem_c.to_string());
    }

    /* ---- math operations ------------------------------------------------- */

    pub fn c_add(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

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
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a + 1.).to_string());
    }

    pub fn c_sub(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a - b).to_string());
    }

    pub fn c_sub_one(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a - 1.).to_string());
    }

    pub fn c_mult(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

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
        Self::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a / b).to_string());
    }

    pub fn c_chs(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((-1. * a).to_string());
    }

    pub fn c_abs(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.abs()).to_string());
    }

    pub fn c_round(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.round()).to_string());
    }

    pub fn c_floor(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push(a.floor().to_string());
    }

    pub fn c_ceiling(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push(a.ceil().to_string());
    }

    pub fn c_pos(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let mut a: f64 = self.pop_stack_float();

        if a < 0. {
            a = 0.;
        }

        self.stack.push(a.to_string());
    }

    pub fn c_inv(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((1. / a).to_string());
    }

    pub fn c_sqrt(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.sqrt()).to_string());
    }

    pub fn c_throot(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a.powf(1. / b)).to_string());
    }

    pub fn c_proot(&mut self, op: &str) {
        Self::check_stack_error(self, 3, op);

        let c: f64 = self.pop_stack_float();
        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        if (b * b - 4. * a * c) < 0. {
            self.stack
                .push((-1. * b / (2. * a)).to_string()); // r_1 real
            self.stack
                .push(((4. * a * c - b * b).sqrt() / (2. * a)).to_string()); // r_1 imag
            self.stack
                .push((-1. * b / (2. * a)).to_string()); // r_2 real
            self.stack
                .push((-1. * (4. * a * c - b * b).sqrt() / (2. * a)).to_string());
        // r_2 imag
        } else {
            self.stack
                .push((-1. * b + (b * b - 4. * a * c).sqrt() / (2. * a)).to_string()); // r_1 real
            self.stack
                .push(0.0.to_string()); // r_1 imag
            self.stack
                .push((-1. * b - (b * b - 4. * a * c).sqrt() / (2. * a)).to_string()); // r_2 real
            self.stack
                .push(0.0.to_string()); // r_2 imag
        }
    }

    pub fn c_exp(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a.powf(b)).to_string());
    }

    pub fn c_mod(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a % b).to_string());
    }

    pub fn c_fact(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((Self::factorial(a)).to_string());
    }

    pub fn c_gcd(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b: u64 = self.pop_stack_uint();
        let a: u64 = self.pop_stack_uint();

        self.stack.push(Self::gcd(a, b).to_string());
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
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.to_radians()).to_string());
    }

    pub fn c_raddeg(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.to_degrees()).to_string());
    }

    pub fn c_sin(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.sin()).to_string());
    }

    pub fn c_asin(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.asin()).to_string());
    }

    pub fn c_cos(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.cos()).to_string());
    }

    pub fn c_acos(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.acos()).to_string());
    }

    pub fn c_tan(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.tan()).to_string());
    }

    pub fn c_atan(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.atan()).to_string());
    }

    pub fn c_log10(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.log10()).to_string());
    }

    pub fn c_log2(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.log2()).to_string());
    }

    pub fn c_logn(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a.log(b)).to_string());
    }

    pub fn c_ln(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.ln()).to_string());
    }

    pub fn c_max(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a.max(b)).to_string());
    }

    pub fn c_max_all(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let mut m: f64 = 0.;
        while !self.stack.is_empty() {
            m = m.max(self.pop_stack_float());
        }

        self.stack.push(m.to_string());
    }

    pub fn c_min(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a.min(b)).to_string());
    }

    pub fn c_min_all(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let mut m: f64 = f64::MAX;
        while !self.stack.is_empty() {
            m = m.min(self.pop_stack_float());
        }

        self.stack.push(m.to_string());
    }

    pub fn c_avg(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push(((a + b) / 2.).to_string());
    }

    pub fn c_avg_all(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let mut sum: f64 = 0.;
        let len: usize = self.stack.len();
        for _ in 0..len {
            sum += self.pop_stack_float();
        }

        self.stack.push((sum / len as f64).to_string());
    }

    pub fn c_rand(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: u64 = self.pop_stack_uint();
        let num: f64 = (a as f64 * rand::random::<f64>()).floor();

        self.stack.push(num.to_string());
    }

    /* ---- conversions ----------------------------------------------------- */

    pub fn c_dechex(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: u64 = self.pop_stack_uint();

        self.stack.push(format!("{:x}", a));
    }

    pub fn c_hexdec(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a = self.pop_stack_int_from_hex();

        self.stack.push(a.to_string());
    }

    pub fn c_decbin(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: u64 = self.pop_stack_uint();

        self.stack.push(format!("{:b}", a));
    }

    pub fn c_bindec(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a = self.pop_stack_int_from_bin();

        self.stack.push(a.to_string());
    }

    pub fn c_binhex(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a = self.pop_stack_int_from_bin();

        self.stack.push(format!("{:x}", a));
    }

    pub fn c_hexbin(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a = self.pop_stack_int_from_hex();

        self.stack.push(format!("{:b}", a));
    }

    pub fn c_celfah(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a = self.pop_stack_float();

        self.stack.push((a * 9. / 5. + 32.).to_string());
    }

    pub fn c_fahcel(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a = self.pop_stack_float();

        self.stack.push(((a - 32.) * 5. / 9.).to_string());
    }

    pub fn c_mikm(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a = self.pop_stack_float();

        self.stack.push((a * 1.609344).to_string());
    }

    pub fn c_kmmi(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a = self.pop_stack_float();

        self.stack.push((a / 1.609344).to_string());
    }

    pub fn c_ftm(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a = self.pop_stack_float();

        self.stack.push((a / 3.281).to_string());
    }

    pub fn c_mft(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a = self.pop_stack_float();

        self.stack.push((a * 3.281).to_string());
    }

    pub fn c_hexrgb(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let she: String = self.stack.pop().unwrap();

        if she.len() < 5 {
            eprintln!(
                "  {}: argument too short [{}] is not of sufficient length",
               self.theme.color_rgb("error", &self.theme.red_bold),
               self.theme.color_rgb(&she, &self.theme.blue_coffee_bold),
            );
            std::process::exit(exitcode::USAGE);
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
        Self::check_stack_error(self, 3, op);

        let b: u64 = self.pop_stack_uint();
        let g: u64 = self.pop_stack_uint();
        let r: u64 = self.pop_stack_uint();

        self.stack.push(format!("{:02x}{:02x}{:02x}", r, g, b));
    }

    pub fn c_tip(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a * self.config.tip_percentage).to_string());
    }

    pub fn c_conv_const(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a * self.config.conversion_constant).to_string());
    }

    /* ---- control flow ---------------------------------------------------- */

    pub fn c_function(&mut self, _op: &str) {
        // get function name
        let fn_name: String = self.ops.remove(0);

        // create new function instance and assign function name
        self.fns.push(
            Function {
                name: fn_name,
                fops: Vec::new(),
            }
        );
        let fn_ind: usize = self.fns.len() - 1; // index of new function in function vector

        // build function operations list
        while self.ops[0] != ")" {
            self.fns[fn_ind].fops.push(self.ops.remove(0));
        }
        self.ops.remove(0); // remove ")"
    }

    pub fn c_ifeq(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

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
                "{" => {
                    nested += 1;
                }
                "}" => {
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

    pub fn c_peek(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        println!(
            "  {}",
            self.theme.color_rgb(
                &self.stack[self.stack.len() - 1],
                &self.theme.orange_sherbet_bold,
            ),
        );
    }

    /* ---- RGB colors ------------------------------------------------------ */

    pub fn c_rgb(&mut self, op: &str) {
        Self::check_stack_error(self, 3, op);

        //let b: u8 = match (self.pop_stack_float_pos() as u8) {
        //    Ok(val) => val,
        //    Err(e) => {
        //        TODO
        //    }
        //}
        let b: u8 = self.pop_stack_float_pos() as u8;
        let g: u8 = self.pop_stack_float_pos() as u8;
        let r: u8 = self.pop_stack_float_pos() as u8;

        self.stack.push(self.output_rgb_dec(coq::Color{r, g, b, bold: false}));
        self.stack.push(self.output_rgb_hex_bg(coq::Color{r, g, b, bold: false}));
    }

    pub fn c_rgbh(&mut self, op: &str) {
        Self::check_stack_error(self, 3, op);

        let b: u8 = self.pop_stack_u8_from_hex();
        let g: u8 = self.pop_stack_u8_from_hex();
        let r: u8 = self.pop_stack_u8_from_hex();

        self.stack.push(self.output_rgb_dec(coq::Color{r, g, b, bold: false}));
        self.stack.push(self.output_rgb_hex_bg(coq::Color{r, g, b, bold: false}));
    }

    /* ---- configuration --------------------------------------------------- */

    pub fn c_save_config(&mut self, _op: &str) {
        // save configuration to file
        self.save_config("comp.toml");
    }

    pub fn c_print_config(&mut self, _op: &str) {
        // print current configuration
        println!(
            "{:#?}",
            self.config,
        )
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

    fn is_user_memory(&self, op: &str) -> Option<String> {
        // is operator a user defined memory item?
        if self.mem.contains_key(op) {
            return Some(self.mem[op].clone());
        }
        None
    }

    // factorial
    pub fn factorial(o: f64) -> f64 {
        let n = o.floor();

        match n {
            n if n < 2. => 1.,
            _           => n * Self::factorial(n - 1.),
        }
    }

    // greatest common divisor
    pub fn gcd(a: u64, b: u64) -> u64 {
        match b {
            b if b != 0 => Self::gcd(b, a % b),
            _           => a,
        }
    }

    // read configuration file from home folder
    pub fn read_and_apply_config(&mut self, filename: &str) {
        /*
        println!(
            "  reading configuration file [{}]",
            self.theme.color_rgb(filename, &self.theme.blue_coffee_bold),
        );
        */

        // read file contents
        let filename: String = filename.to_string();

        let home_folder: String = match home::home_dir() {
            Some(dir) => dir.to_str().unwrap().to_string(),
            _ => String::from(""),
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
                        "  {}: configuration file [{}] (ignored) has been corrupted or \
                        is improperly constructed for this version of comp",
                        self.theme.color_rgb("warning", &self.theme.yellow_canary_bold),
                        self.theme.color_rgb("conf.toml", &self.theme.blue_smurf_bold),
                    );
                    Config::new()
                }
            };

            self.config = cfg;
        }
    }

    // save configuration file to home folder
    fn save_config(&self, filename: &str) {
        let filename: String = filename.to_string();

        let home_folder: String = match home::home_dir() {
            Some(dir) => dir.to_str().unwrap().to_string(),
            _ => String::from(""),
        };

        let config_filename: String = format!("{}/{}", home_folder, filename);

        let path: &Path = Path::new(&config_filename);

        let config_data: String = toml::to_string(&self.config).unwrap();

        match fs::write(path, config_data) {
            Ok(_) => {
                println!(
                    "  configuration file [{}] saved",
                    self.theme.color_rgb("conf.toml", &self.theme.blue_smurf_bold),
                );
            }
            Err(e) => {
                eprintln!(
                    "  {}: configuration file [{}] could not be saved: {}",
                    self.theme.color_rgb("error", &self.theme.red_bold),
                    self.theme.color_rgb("conf.toml", &self.theme.blue_smurf_bold),
                    e,
                );
            }
        }
    }

    fn output_rgb_dec(&self, color: coq::Color) -> String {
        format!(
            "{} {} {}",
            self.theme.color_rgb(
                &color.r.to_string(),
                &color,
            ),
            self.theme.color_rgb(
                &color.g.to_string(),
                &coq::Color {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                    bold: color.bold,
                },
            ),
            self.theme.color_rgb(
                &color.b.to_string(),
                &color,
            ),
        )
    }

    fn _output_rgb_hex(&self, color: coq::Color) -> String {
        format!(
            "{}",
            self.theme.color_rgb(
                &format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b),
                &color,
            ),
        )
    }

    fn output_rgb_hex_bg(&self, color: coq::Color) -> String {
        format!(
            "{}",
            self.theme.color_rgb_bg(
                &format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b),
                &color,
            ),
        )
    }

    pub fn get_cmds(&self) -> Vec<&str> {
        let mut cmds: Vec<&str> = Vec::new();
        for key in self.cmdmap.keys() {
            cmds.push(key);
        }
        cmds
    }

}

pub struct Function {
    name: String,
    fops: Vec<String>,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub show_stack_level: bool,
    pub conversion_constant: f64,
    pub monochrome: bool,
    pub tip_percentage: f64,
}

impl Config {
    // constructor
    fn new() -> Self {
        Self {
            show_stack_level: true,
            conversion_constant: 1.,
            monochrome: false,
            tip_percentage: 0.15,
        }
    }
}