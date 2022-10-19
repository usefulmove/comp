use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::{fmt, fs};
use std::num::{ParseFloatError, ParseIntError};
use std::path::Path;
use std::process::exit;

static PERSISTENCE_FILE: &str = ".comp";
static CONFIG_FILE: &str = "comp.toml";

pub struct Function {
    name: String,
    fops: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub show_stack_level: bool, // annotate stack level
    pub conversion_constant: f64, // configurable constant for a_b conversion
    pub monochrome: bool, // set output to monochrome
    pub tip_percentage: f64, // tip conversion constant
    pub show_warnings: bool, // show warnings
    pub stack_persistence: bool, // stack persistence
}

impl Config {
    // constructor
    fn new() -> Self {
        Self { // config defaults
            show_stack_level: true,
            conversion_constant: 1.,
            monochrome: false,
            tip_percentage: 0.15,
            show_warnings: true,
            stack_persistence: false,
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let theme = cor::Theme::new();
        let output_color = &theme.blue_smurf;
        write!(
            f,
            "\n\
            show_stack_level = {}\n\
            conversion_const = {}\n\
            monochrome = {}\n\
            tip_percentage = {}\n\
            show_warnings = {}\n\
            stack_persistence = {}\n\
            ",
            theme.color_rgb(
                &self.show_stack_level.to_string(),
                output_color,
            ),
            theme.color_rgb(
                &self.conversion_constant.to_string(),
                output_color,
            ),
            theme.color_rgb(
                &self.monochrome.to_string(),
                output_color,
            ),
            theme.color_rgb(
                &self.tip_percentage.to_string(),
                output_color,
            ),
            theme.color_rgb(
                &self.show_warnings.to_string(),
                output_color,
            ),
            theme.color_rgb(
                &self.stack_persistence.to_string(),
                output_color,
            ),
        )
    }
}

pub struct Interpreter {
    pub ops: Vec<String>,
    pub config: Config,
    stack: Vec<String>,
    mem: HashMap<String, String>,
    mem_a: f64,
    mem_b: f64,
    mem_c: f64,
    fns: Vec<Function>,
    cmdmap: HashMap<String, fn(&mut Interpreter, &str)>,
    theme: cor::Theme,
}

impl Interpreter {
    // constructor
    pub fn new() -> Self {
        let mut cint = Self {
            stack: vec![],
            mem: HashMap::new(), // local interpreter memory
            mem_a: 0., // local interpreter memory
            mem_b: 0.,
            mem_c: 0.,
            ops: vec![], // operations list
            fns: vec![], // user-defined functions
            cmdmap: HashMap::new(), // interpreter command map
            config: Config::new(), // configuration object
            theme: cor::Theme::new(), // output format theme
        };
        cint.init();

        cint
    }

    // process operations method
    pub fn process_ops(&mut self) {
        while !self.ops.is_empty() {
            let op: &str = &self.ops.remove(0); // pop first operation
            self.evaluate_op(op);
        }
    }

    // add native command to interpreter
    pub fn compose_native(&mut self, name: &str, func: fn(&mut Self, &str)) {
        self.cmdmap.insert(name.to_string(), func);
    }

    fn init(&mut self) {

        /* stack manipulation */
        self.compose_native("drop", Self::c_drop); // drop element on top of stack
        self.compose_native("dropn", Self::c_dropn); // drop n elements
        self.compose_native("take", Self::c_take); // take element on top of stack
        self.compose_native("taken", Self::c_taken); // take n elements
        self.compose_native("dup", Self::c_dup); // duplicate
        self.compose_native("swap", Self::c_swap); // swap x and y
        self.compose_native("cls", Self::c_cls); // clear stack
        self.compose_native("clr", Self::c_cls); // clear stack
        self.compose_native("roll", Self::c_roll); // roll stack
        self.compose_native("rolln", Self::c_rolln); // roll stack (n)
        self.compose_native("rot", Self::c_rot); // rotate stack (reverse direction from roll)
        self.compose_native("rotn", Self::c_rotn); // rotate stack (n)
        self.compose_native("..", Self::c_range); // add range of numbers to stack (generic)
        self.compose_native("io", Self::c_iota); // add range of integers to stack (limited - base 1)
        self.compose_native("i0", Self::c_iota_zero); // add range of integers to stack (limited - base 0)
        self.compose_native("flip", Self::c_flip); // flip stack order

        /* memory usage */
        self.compose_native("store", Self::c_store); // store (pop value off stack and store in generic memory)
        self.compose_native("sa", Self::c_store_a); // store (pop value off stack and store)
        self.compose_native("_a", Self::c_push_a); // retrieve (push stored value onto the stack)
        self.compose_native("sb", Self::c_store_b); // store
        self.compose_native("_b", Self::c_push_b); // retrieve
        self.compose_native("sc", Self::c_store_c); // store
        self.compose_native("_c", Self::c_push_c); // retrieve

        /* maths operations */
        self.compose_native("+", Self::c_add); // add
        self.compose_native("+_", Self::c_sum); // sum (add all stack elements)
        self.compose_native("sum", Self::c_sum);
        self.compose_native("++", Self::c_add_one); // add one
        self.compose_native("-", Self::c_sub); // subtract
        self.compose_native("--", Self::c_sub_one); // subtract one
        self.compose_native("x", Self::c_mult); // multiply
        self.compose_native("x_", Self::c_product); // product (multiply all stack elements)
        self.compose_native("prod", Self::c_product);
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
        self.compose_native("nroot", Self::c_nroot); // nth root
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
        self.compose_native("loge", Self::c_ln);
        self.compose_native("rand", Self::c_rand); // random number
        self.compose_native("max", Self::c_max); // maximum
        self.compose_native("max_all", Self::c_max_all); // maximum (all)
        self.compose_native("max_", Self::c_max_all);
        self.compose_native("min", Self::c_min); // minimum
        self.compose_native("min_all", Self::c_min_all); // minimum (all)
        self.compose_native("min_", Self::c_min_all);
        self.compose_native("minmax", Self::c_minmax); // minmax
        self.compose_native("avg", Self::c_avg); // average
        self.compose_native("avg_all", Self::c_avg_all); // average (all)
        self.compose_native("avg_", Self::c_avg_all); //
        self.compose_native("sgn", Self::c_sign); // sign function
        self.compose_native("tng", Self::c_triangle); // trianglar numbers function
        self.compose_native("divs", Self::c_divisors); // find divisors of a number

        /* control flow */
        self.compose_native("(", Self::c_load_function); // function definition
        self.compose_native("[", Self::c_load_lambda); // anonymous function definition
        self.compose_native("ifeq", Self::c_ifeq); // ifequal .. else
        self.compose_native("eq", Self::c_equal); // equal
        self.compose_native("lt", Self::c_lessthan); // less than
        self.compose_native("lte", Self::c_lessthanorequal); // less than or equal
        self.compose_native("gt", Self::c_greaterthan); // greater than
        self.compose_native("gte", Self::c_greaterthanorequal); // greater than or equal
        self.compose_native("{", Self::c_comment); // function comment

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
        self.compose_native("b_a", Self::c_conv_const_inv); // apply convert constant (inverse)
        self.compose_native("ascii", Self::c_ascii); // ascii table

        /* binary operations */
        self.compose_native("not", Self::c_not); // bitwise not
        self.compose_native("and", Self::c_and); // bitwise and
        self.compose_native("nand", Self::c_nand); // bitwise nand
        self.compose_native("or", Self::c_or); // bitwise or
        self.compose_native("nor", Self::c_nor); // bitwise nor
        self.compose_native("xor", Self::c_xor); // bitwise xor
        self.compose_native("ones", Self::c_ones); // count number of high bits

        /* RGB colors */
        self.compose_native("rgb", Self::c_rgb); // show RGB color
        self.compose_native("rgbh", Self::c_rgbh); // show RGB color (hexadecimal)
        self.compose_native("rgb_avg", Self::c_rgb_avg); // calculate average RGB color

        /* higher-order functions */
        self.compose_native("map", Self::c_map); // map annonymous function to stack
        self.compose_native("fold", Self::c_fold); // fold stack using annonymous function
        self.compose_native("scan", Self::c_scan); // scan stack using annonymous function

        /* configuration */
        self.compose_native("save_config", Self::c_save_config); // save configuration
        self.compose_native("show_config", Self::c_print_config); // show current configuration

        /* output */
        self.compose_native("peek", Self::c_peek); // peek at top of stack
        self.compose_native("print", Self::c_print); // print element on top of stack

    }

    fn evaluate_op(&mut self, op: &str) {
        /* native command? */
        if self.cmdmap.contains_key(op) {
            let f = self.cmdmap[op];
            f(self, op); // execute command function
            return;
        }

        /* user-defined function? */
        if let Some(index) = self.is_user_function(op) {
            // user-defined function - copy user function ops (fops) into main ops
            for fop in self.fns[index].fops.iter().rev() {
                self.ops.insert(0, fop.clone());
            }
            return;
        }

        /* user memory */
        if let Some(value) = self.is_user_memory(op) {
            // user-defined memory - push value onto stack
            self.ops.insert(0, value);
            return;
        }

        /* neither native command nor user-defined function nor user-defined memory */

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
            Err(_) => {
                // parse fail
                eprintln!(
                    "  {}: unknown expression [{}] is not a recognized operation \
                    or valid value (f)",
                    self.theme.color_rgb("error", &self.theme.red_bold),
                    self.theme.color_rgb(&element, &self.theme.blue_coffee_bold),
                );
                exit(exitcode::USAGE);
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
            Err(_) => {
                // parse fail
                eprintln!(
                    "  {}: unknown expression [{}] is not a recognized operation \
                    or valid value (f)",
                    self.theme.color_rgb("error", &self.theme.red_bold),
                    self.theme.color_rgb(&element, &self.theme.blue_coffee_bold),
                );
                exit(exitcode::USAGE);
            }
        }
    }

    pub fn pop_stack_i64(&mut self) -> i64 {
        let element: String = self.stack.pop().unwrap();
        match self.parse_int(&element) {
            Ok(val) => val, // parse success
            Err(_) => {
                // parse fail
                eprintln!(
                    "  {}: unknown expression [{}] is not a recognized operation \
                    or valid value (u)",
                   self.theme.color_rgb("error", &self.theme.red_bold),
                   self.theme.color_rgb(&element, &self.theme.blue_coffee_bold),
                );
                exit(exitcode::USAGE);
            }
        }
    }

    pub fn _pop_stack_u8(&mut self) -> u8 {
        let element: String = self.stack.pop().unwrap();
        match self._parse_u8(&element) {
            Ok(val) => val, // parse success
            Err(_) => {
                // parse fail
                eprintln!(
                    "  {}: unknown expression [{}] is not a recognized operation \
                    or valid value (u)",
                   self.theme.color_rgb("error", &self.theme.red_bold),
                   self.theme.color_rgb(&element, &self.theme.blue_coffee_bold),
                );
                exit(exitcode::USAGE);
            }
        }
    }

    pub fn pop_stack_usize(&mut self) -> usize {
        let element: String = self.stack.pop().unwrap();
        match self.parse_usize(&element) {
            Ok(val) => val, // parse success
            Err(_) => {
                // parse fail
                eprintln!(
                    "  {}: unknown expression [{}] is not a recognized operation \
                    or valid value (u)",
                   self.theme.color_rgb("error", &self.theme.red_bold),
                   self.theme.color_rgb(&element, &self.theme.blue_coffee_bold),
                );
                exit(exitcode::USAGE);
            }
        }
    }

    pub fn pop_stack_u64(&mut self) -> u64 {
        let element: String = self.stack.pop().unwrap();
        match self.parse_u64(&element) {
            Ok(val) => val, // parse success
            Err(_) => {
                // parse fail
                eprintln!(
                    "  {}: unknown expression [{}] is not a recognized operation \
                    or valid value (u)",
                   self.theme.color_rgb("error", &self.theme.red_bold),
                   self.theme.color_rgb(&element, &self.theme.blue_coffee_bold),
                );
                exit(exitcode::USAGE);
            }
        }
    }

    pub fn pop_stack_u128(&mut self) -> u128 {
        let element: String = self.stack.pop().unwrap();
        match self.parse_u128(&element) {
            Ok(val) => val, // parse success
            Err(_) => {
                // parse fail
                eprintln!(
                    "  {}: unknown expression [{}] is not a recognized operation \
                    or valid value (u)",
                   self.theme.color_rgb("error", &self.theme.red_bold),
                   self.theme.color_rgb(&element, &self.theme.blue_coffee_bold),
                );
                exit(exitcode::USAGE);
            }
        }
    }

    pub fn pop_stack_int_from_hex(&mut self) -> i64 {
        let element: String = self.stack.pop().unwrap();

        match i64::from_str_radix(&element, 16) {
            Ok(val) => val, // parse success
            Err(_) => {
                // parse fail
                eprintln!(
                    "  {}: unknown expression [{}] is not a recognized operation \
                    or valid value (i_h)",
                   self.theme.color_rgb("error", &self.theme.red_bold),
                   self.theme.color_rgb(&element, &self.theme.blue_coffee_bold),
                );
                exit(exitcode::USAGE);
            }
        }
    }

    pub fn pop_stack_u8_from_hex(&mut self) -> u8 {
        let element: String = self.stack.pop().unwrap();

        match u8::from_str_radix(&element, 16) {
            Ok(val) => val, // parse success
            Err(_) => {
                // parse fail
                eprintln!(
                    "  {}: unknown expression [{}] is not a recognized operation \
                    or valid value (i_h)",
                   self.theme.color_rgb("error", &self.theme.red_bold),
                   self.theme.color_rgb(&element, &self.theme.blue_coffee_bold),
                );
                exit(exitcode::USAGE);
            }
        }
    }

    pub fn pop_stack_int_from_bin(&mut self) -> i64 {
        let element: String = self.stack.pop().unwrap();

        match i64::from_str_radix(&element, 2) {
            Ok(val) => val, // parse success
            Err(_) => {
                // parse fail
                eprintln!(
                    "  {}: unknown expression [{}] is not a recognized operation \
                    or valid value (i_b)",
                   self.theme.color_rgb("error", &self.theme.red_bold),
                   self.theme.color_rgb(&element, &self.theme.blue_coffee_bold),
                );
                exit(exitcode::USAGE);
            }
        }
    }

    fn parse_float(&self, op: &str) -> Result<f64, ParseFloatError> {
        self.parse_f64(op)
    }

    fn parse_f64(&self, op: &str) -> Result<f64, ParseFloatError> {
        let value = op.parse::<f64>()?;
        Ok(value)
    }

    fn parse_int(&self, op: &str) -> Result<i64, ParseIntError> {
        self.parse_i64(op)
    }

    fn parse_i64(&self, op: &str) -> Result<i64, ParseIntError> {
        let value = op.parse::<i64>()?;
        Ok(value)
    }

    fn _parse_u8(&self, op: &str) -> Result<u8, ParseIntError> {
        let value = op.parse::<u8>()?;
        Ok(value)
    }

    fn parse_usize(&self, op: &str) -> Result<usize, ParseIntError> {
        let value = op.parse::<usize>()?;
        Ok(value)
    }

    fn parse_u64(&self, op: &str) -> Result<u64, ParseIntError> {
        let value = op.parse::<u64>()?;
        Ok(value)
    }

    fn parse_u128(&self, op: &str) -> Result<u128, ParseIntError> {
        let value = op.parse::<u128>()?;
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
            exit(exitcode::USAGE);
        }
    }

    /* command functions ---------------------------------------------------- */

    /*** command generation helper function ***/
    fn cmd_gen(&mut self, args: usize, op: &str, f: fn(f64, f64) -> f64) {
        Self::check_stack_error(self, args, op);

        match args {
            1 => {
                let a: f64 = self.pop_stack_float();
                self.stack.push(f(a, 0.).to_string());
            }
            2 => {
                let b: f64 = self.pop_stack_float();
                let a: f64 = self.pop_stack_float();
                self.stack.push(f(a, b).to_string());
            }
            _ => unimplemented!(),
        }
    }

    /* ---- stack manipulation ---------------------------------------------- */

    fn c_drop(&mut self, op: &str) {
        if !self.stack.is_empty() {
            self.stack.pop();
            return;
        }

       // stack empty
       if self.config.show_warnings {
            eprintln!(
                "  {}: [{}] operation called on empty stack",
                self.theme.color_rgb("warning", &self.theme.yellow_canary_bold),
                self.theme.color_rgb(op, &self.theme.blue_coffee_bold),
            );
        }
        // do not stop execution
    }

    fn c_dropn(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let mut drop_count: i64 = self.pop_stack_i64();

        if drop_count < 1 {
            eprintln!(
                "  {}: [{}] operation called with bad argument [{}]",
                self.theme.color_rgb("error", &self.theme.red_bold),
                self.theme.color_rgb(op, &self.theme.blue_coffee_bold),
                self.theme.color_rgb(&drop_count.to_string(), &self.theme.blue_coffee_bold),
            );
            exit(exitcode::USAGE);
        }

        while drop_count > 0 {
            drop_count -= 1;

            if !self.stack.is_empty() {
                self.stack.pop();
                return;
            }

            // stack empty
            if self.config.show_warnings {
                eprintln!(
                    "  {}: [{}] operation called on empty stack",
                    self.theme.color_rgb("warning", &self.theme.yellow_canary_bold),
                    self.theme.color_rgb(op, &self.theme.blue_coffee_bold),
                );
            }
            // do not stop execution
        }
    }

    fn c_take(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let keep: String = self.pop_stack_string();
        self.stack = vec![];
        self.stack.push(keep);
    }

    fn c_taken(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let take_count: usize = self.pop_stack_usize();
        let len: usize = self.stack.len();

        if take_count < 1 {
            eprintln!(
                "  {}: [{}] operation called with bad argument [{}]",
                self.theme.color_rgb("error", &self.theme.red_bold),
                self.theme.color_rgb(op, &self.theme.blue_coffee_bold),
                self.theme.color_rgb(&take_count.to_string(), &self.theme.blue_coffee_bold),
            );
            exit(exitcode::USAGE);
        }

        if take_count > len {
            if self.config.show_warnings {
                eprintln!(
                    "  {}: [{}] operation called with argument [{}] \
                    greater than stack depth [{}]",
                    self.theme.color_rgb("warning", &self.theme.yellow_canary_bold),
                    self.theme.color_rgb(op, &self.theme.blue_coffee_bold),
                    self.theme.color_rgb(&take_count.to_string(), &self.theme.blue_coffee_bold),
                    self.theme.color_rgb(&len.to_string(), &self.theme.blue_coffee_bold),
                );
            }
            return;
        }

        self.stack = self.stack[(len-take_count)..len].to_vec();
    }

    fn c_dup(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        self.stack.push(
            self.stack[self.stack.len()-1]
                .clone()
        ); // remove last
    }

    fn c_swap(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let end: usize = self.stack.len() - 1;

        self.stack.swap(end, end - 1);
    }

    fn c_cls(&mut self, _op: &str) {
        self.stack.clear();
    }

    fn c_roll(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        self.stack.rotate_right(1);
    }

    fn c_rolln(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let a: usize = self.pop_stack_usize();

        self.stack.rotate_right(a);
    }

    fn c_rot(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        self.stack.rotate_left(1);
    }

    fn c_rotn(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let a: usize = self.pop_stack_usize();

        self.stack.rotate_left(a);
    }

    fn c_range(&mut self, op: &str) {
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

    fn c_iota(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: i64 = self.pop_stack_i64();

        if a < 1 {
            eprintln!(
                "  {}: [{}] operation called with invalid argument - argument cannot be less than 1",
                self.theme.color_rgb("error", &self.theme.red_bold),
                self.theme.color_rgb(op, &self.theme.blue_coffee_bold),
            );
            exit(exitcode::USAGE);
        }

        for i in 1..=a as i64 {
            self.stack.push(i.to_string());
        }
    }

    fn c_iota_zero(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: i64 = self.pop_stack_i64();

        if a < 0 {
            eprintln!(
                "  {}: [{}] operation called with invalid argument - argument cannot be negative",
                self.theme.color_rgb("error", &self.theme.red_bold),
                self.theme.color_rgb(op, &self.theme.blue_coffee_bold),
            );
            exit(exitcode::USAGE);
        }

        for i in 0..=a as i64 {
            self.stack.push(i.to_string());
        }
    }

    fn c_flip(&mut self, _op: &str) {
        self.stack = self.stack
            .clone()
            .into_iter()
            .rev()
            .collect();
    }

    /* ---- memory usage ---------------------------------------------------- */

    fn c_store(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let key = self.pop_stack_string();
        let val = self.pop_stack_string();

        self.mem.insert(key, val);
    }

    fn c_store_a(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        self.mem_a = self.pop_stack_float();
    }

    fn c_push_a(&mut self, _op: &str) {
        self.stack.push(self.mem_a.to_string());
    }

    fn c_store_b(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        self.mem_b = self.pop_stack_float();
    }

    fn c_push_b(&mut self, _op: &str) {
        self.stack.push(self.mem_b.to_string());
    }

    fn c_store_c(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        self.mem_c = self.pop_stack_float();
    }

    fn c_push_c(&mut self, _op: &str) {
        self.stack.push(self.mem_c.to_string());
    }

    /* ---- math operations ------------------------------------------------- */

    fn c_add(&mut self, op: &str) {
        self.cmd_gen(2, op, |a, b| a + b);
    }

    fn c_sum(&mut self, op: &str) {
        while self.stack.len() > 1 {
            self.c_add(op);
        }
    }

    fn c_add_one(&mut self, op: &str) {
        self.cmd_gen(1, op, |a, _| a + 1.);
    }

    fn c_sub(&mut self, op: &str) {
        self.cmd_gen(2, op, |a, b| a - b);
    }

    fn c_sub_one(&mut self, op: &str) {
        self.cmd_gen(1, op, |a, _| a - 1.);
    }

    fn c_mult(&mut self, op: &str) {
        self.cmd_gen(2, op, |a, b| a * b);
    }

    fn c_product(&mut self, op: &str) {
        while self.stack.len() > 1 {
            self.c_mult(op);
        }
    }

    fn c_div(&mut self, op: &str) {
        self.cmd_gen(2, op, |a, b| a / b);
    }

    fn c_chs(&mut self, op: &str) {
        self.cmd_gen(1, op, |a, _| -1. * a);
    }

    fn c_abs(&mut self, op: &str) {
        self.cmd_gen(1, op, |a, _| a.abs());
    }

    fn c_round(&mut self, op: &str) {
        self.cmd_gen(1, op, |a, _| a.round());
    }

    fn c_floor(&mut self, op: &str) {
        self.cmd_gen(1, op, |a, _| a.floor());
    }

    fn c_ceiling(&mut self, op: &str) {
        self.cmd_gen(1, op, |a, _| a.ceil());
    }

    fn c_pos(&mut self, op: &str) {
        self.cmd_gen(1, op, |a, _| if a < 0. {0.} else {a});
    }

    fn c_inv(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((1. / a).to_string());
    }

    fn c_sqrt(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.sqrt()).to_string());
    }

    fn c_nroot(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a.powf(1. / b)).to_string());
    }

    fn c_proot(&mut self, op: &str) {
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

    fn c_exp(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a.powf(b)).to_string());
    }

    fn c_mod(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a % b).to_string());
    }

    fn c_fact(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: u128 = self.pop_stack_u128();

        self.stack.push((Self::factorial(a)).to_string());
    }

    fn c_gcd(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b: u64 = self.pop_stack_u64();
        let a: u64 = self.pop_stack_u64();

        self.stack.push(Self::gcd(a, b).to_string());
    }

    fn c_pi(&mut self, _op: &str) {
        self.stack.push(std::f64::consts::PI.to_string());
    }

    fn c_euler(&mut self, _op: &str) {
        self.stack.push(std::f64::consts::E.to_string());
    }

    fn c_accelg(&mut self, _op: &str) {
        self.stack.push(9.80665.to_string());
    }

    fn c_degrad(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.to_radians()).to_string());
    }

    fn c_raddeg(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.to_degrees()).to_string());
    }

    fn c_sin(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.sin()).to_string());
    }

    fn c_asin(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.asin()).to_string());
    }

    fn c_cos(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.cos()).to_string());
    }

    fn c_acos(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.acos()).to_string());
    }

    fn c_tan(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.tan()).to_string());
    }

    fn c_atan(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.atan()).to_string());
    }

    fn c_log10(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.log10()).to_string());
    }

    fn c_log2(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.log2()).to_string());
    }

    fn c_logn(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a.log(b)).to_string());
    }

    fn c_ln(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a.ln()).to_string());
    }

    fn c_rand(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: u64 = self.pop_stack_u64();
        let num: f64 = (a as f64 * rand::random::<f64>()).floor();

        self.stack.push(num.to_string());
    }

    fn c_max(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a.max(b)).to_string());
    }

    fn c_max_all(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let mut m: f64 = f64::MIN;
        while !self.stack.is_empty() {
            m = m.max(self.pop_stack_float());
        }

        self.stack.push(m.to_string());
    }

    fn c_min(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push((a.min(b)).to_string());
    }

    fn c_min_all(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let mut m: f64 = f64::MAX;
        while !self.stack.is_empty() {
            m = m.min(self.pop_stack_float());
        }

        self.stack.push(m.to_string());
    }

    fn c_minmax(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let mut max: f64 = f64::MIN;
        let mut min: f64 = f64::MAX;
        while !self.stack.is_empty() {
            let a: f64 = self.pop_stack_float();

            if a > max { max = a }
            if a < min { min = a }
        }

        self.stack.push((min).to_string());
        self.stack.push((max).to_string());
    }

    fn c_avg(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b: f64 = self.pop_stack_float();
        let a: f64 = self.pop_stack_float();

        self.stack.push(((a + b) / 2.).to_string());
    }

    fn c_avg_all(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let mut sum: f64 = 0.;
        let len: usize = self.stack.len();
        for _ in 0..len {
            sum += self.pop_stack_float();
        }

        self.stack.push((sum / len as f64).to_string());
    }

    fn c_sign(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        let sgn: f64 = match a {
            x if x < 0. => -1.,
            x if x > 0. => 1.,
            _ => 0.,
        };

        self.stack.push(sgn.to_string());
    }

    fn c_triangle(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let mut a: i64 = self.pop_stack_i64();

        if a < 0 { a = 0 }

        self.stack.push((a*(a+1)/2).to_string());
    }

    fn c_divisors(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: i64 = self.pop_stack_i64().abs();

        let mut divisors: Vec<i64> = vec![1];
        let sq: i64 = (a as f64).sqrt() as i64;

        (2..=sq).for_each(|n| {
            if a % n == 0 {
                divisors.push(n);
                if n != sq { divisors.push(a/n) }
            }
        });

        divisors.sort();

        divisors.into_iter()
            .for_each(|n| self.stack.push(n.to_string()));
    }

    /* ---- conversions ----------------------------------------------------- */

    fn c_dechex(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: u64 = self.pop_stack_u64();

        self.stack.push(format!("{:x}", a));
    }

    fn c_hexdec(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a = self.pop_stack_int_from_hex();

        self.stack.push(a.to_string());
    }

    fn c_decbin(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: u64 = self.pop_stack_u64();

        self.stack.push(format!("{:b}", a));
    }

    fn c_bindec(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a = self.pop_stack_int_from_bin();

        self.stack.push(a.to_string());
    }

    fn c_binhex(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a = self.pop_stack_int_from_bin();

        self.stack.push(format!("{:x}", a));
    }

    fn c_hexbin(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a = self.pop_stack_int_from_hex();

        self.stack.push(format!("{:b}", a));
    }

    fn c_celfah(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a = self.pop_stack_float();

        self.stack.push((a * 9. / 5. + 32.).to_string());
    }

    fn c_fahcel(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a = self.pop_stack_float();

        self.stack.push(((a - 32.) * 5. / 9.).to_string());
    }

    fn c_mikm(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a = self.pop_stack_float();

        self.stack.push((a * 1.609344).to_string());
    }

    fn c_kmmi(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a = self.pop_stack_float();

        self.stack.push((a / 1.609344).to_string());
    }

    fn c_ftm(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a = self.pop_stack_float();

        self.stack.push((a / 3.281).to_string());
    }

    fn c_mft(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a = self.pop_stack_float();

        self.stack.push((a * 3.281).to_string());
    }

    fn c_hexrgb(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let she: String = self.stack.pop().unwrap();

        if she.len() < 5 {
            eprintln!(
                "  {}: argument too short [{}] is not of sufficient length",
               self.theme.color_rgb("error", &self.theme.red_bold),
               self.theme.color_rgb(&she, &self.theme.blue_coffee_bold),
            );
            exit(exitcode::USAGE);
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
        Self::check_stack_error(self, 3, op);

        let b: u64 = self.pop_stack_u64();
        let g: u64 = self.pop_stack_u64();
        let r: u64 = self.pop_stack_u64();

        self.stack.push(format!("{:02x}{:02x}{:02x}", r, g, b));
    }

    fn c_tip(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a * self.config.tip_percentage).to_string());
    }

    fn c_conv_const(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a * self.config.conversion_constant).to_string());
    }

    fn c_conv_const_inv(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: f64 = self.pop_stack_float();

        self.stack.push((a / self.config.conversion_constant).to_string());
    }

    fn c_ascii(&mut self, _op: &str) {
        (0..=255)
            .map(|a| (a, a as u8 as char))
            .filter(|(_val, c)| c.is_alphanumeric() || c.is_ascii_punctuation())
            .map(|(val, c)| {
                format!(
                    "'{}'  {}",
                    self.theme.color_rgb(&c.to_string(), &self.theme.blue_coffee_bold),
                    self.theme.color_rgb(&val.to_string(), &self.theme.grey_mouse),
                )
             })
            .for_each(|s| println!("  {}", s));
    }

    /* ---- binary operations ----------------------------------------------- */

    fn c_not(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: u64 = self.pop_stack_u64();

        self.stack.push((!a).to_string());
    }

    fn c_and(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b: u64 = self.pop_stack_u64();
        let a: u64 = self.pop_stack_u64();

        self.stack.push((a & b).to_string());
    }

    fn c_nand(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b: u64 = self.pop_stack_u64();
        let a: u64 = self.pop_stack_u64();

        self.stack.push((!(a & b)).to_string());
    }

    fn c_or(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b: u64 = self.pop_stack_u64();
        let a: u64 = self.pop_stack_u64();

        self.stack.push((a | b).to_string());
    }

    fn c_nor(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b: u64 = self.pop_stack_u64();
        let a: u64 = self.pop_stack_u64();

        self.stack.push((!(a | b)).to_string());
    }

    fn c_xor(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b: u64 = self.pop_stack_u64();
        let a: u64 = self.pop_stack_u64();

        self.stack.push((a ^ b).to_string());
    }

    fn c_ones(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let a: u64 = self.pop_stack_u64();

        self.stack.push(a.count_ones().to_string());
    }

    /* ---- control flow ---------------------------------------------------- */

    fn c_load_function(&mut self, _op: &str) {
        // get function name
        let fn_name: String = self.ops.remove(0);

        // create new function instance and assign function name
        self.fns.push(
            Function {
                name: fn_name,
                fops: vec![],
            }
        );
        let fn_ind: usize = self.fns.len() - 1; // index of new function in function vector

        // build function operations list
        while self.ops[0] != ")" {
            self.fns[fn_ind].fops.push(self.ops.remove(0));
        }
        self.ops.remove(0); // remove ")"
    }

    fn c_load_lambda(&mut self, _op: &str) {
        // clear existing anonymous function definition
        if let Some(index) = self.is_user_function("_") {
            self.fns.remove(index);
        }

        // create new anonymous function instance
        self.fns.push(
            Function {
                name: String::from("_"),
                fops: vec![],
            }
        );
        let fn_ind: usize = self.fns.len() - 1; // index of new function in function vector

        // build anonymous function operations list
        while self.ops[0] != "]" {
            self.fns[fn_ind].fops.push(self.ops.remove(0));
        }
        self.ops.remove(0); // remove "|"
    }

    fn c_equal(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b = self.pop_stack_float();
        let a = self.pop_stack_float();

        match a == b {
            false => self.stack.push("0".to_string()),
            true => self.stack.push("1".to_string()),
        }
    }

    fn c_lessthan(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b = self.pop_stack_float();
        let a = self.pop_stack_float();

        match a < b {
            false => self.stack.push("0".to_string()),
            true => self.stack.push("1".to_string()),
        }
    }

    fn c_lessthanorequal(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b = self.pop_stack_float();
        let a = self.pop_stack_float();

        match a <= b {
            false => self.stack.push("0".to_string()),
            true => self.stack.push("1".to_string()),
        }
    }

    fn c_greaterthan(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b = self.pop_stack_float();
        let a = self.pop_stack_float();

        match a > b {
            false => self.stack.push("0".to_string()),
            true => self.stack.push("1".to_string()),
        }
    }

    fn c_greaterthanorequal(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b = self.pop_stack_float();
        let a = self.pop_stack_float();

        match a >= b {
            false => self.stack.push("0".to_string()),
            true => self.stack.push("1".to_string()),
        }
    }

    fn c_ifeq(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b = self.pop_stack_float();
        let a = self.pop_stack_float();

        let mut if_ops: Vec<String> = vec![];

        let mut depth: usize = 0;

        if a == b {
            // execute if condition
            // store list of operations until 'else' or 'fi'
            while (depth > 0) || ((self.ops[0] != "fi") && (self.ops[0] != "else")) {
                match self.ops[0].as_str() {
                    "ifeq" => depth += 1, // increase depth
                    "fi" => depth -= 1,   // decrease depth
                    _ => (),
                }
                if_ops.push(self.ops.remove(0));
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
                    if_ops.push(self.ops.remove(0));
                }
            }
            self.ops.remove(0); // remove "fi"
        }

        for op in if_ops.iter().rev() {
            self.ops.insert(0, op.to_string());
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

    /* ---- RGB colors ------------------------------------------------------ */

    fn c_rgb(&mut self, op: &str) {
        Self::check_stack_error(self, 3, op);

        let b: u8 = self.pop_stack_float_pos() as u8;
        let g: u8 = self.pop_stack_float_pos() as u8;
        let r: u8 = self.pop_stack_float_pos() as u8;

        self.stack.push(self.output_rgb_dec(cor::Color{r, g, b, bold: false}));
        self.stack.push(self.output_rgb_hex_bg(cor::Color{r, g, b, bold: false}));
    }

    fn c_rgbh(&mut self, op: &str) {
        Self::check_stack_error(self, 3, op);

        let b: u8 = self.pop_stack_u8_from_hex();
        let g: u8 = self.pop_stack_u8_from_hex();
        let r: u8 = self.pop_stack_u8_from_hex();

        self.stack.push(self.output_rgb_dec(cor::Color{r, g, b, bold: false}));
        self.stack.push(self.output_rgb_hex_bg(cor::Color{r, g, b, bold: false}));
    }

    fn c_rgb_avg(&mut self, op: &str) {
        Self::check_stack_error(self, 2, op);

        let b: String = self.pop_stack_string();
        let a: String = self.pop_stack_string();

        if a.len() != 6 || b.len() != 6 {
            eprintln!(
                "  {}: argument is incorrect for [{}] command",
               self.theme.color_rgb("error", &self.theme.red_bold),
               self.theme.color_rgb(op, &self.theme.blue_coffee_bold),
            );
            exit(exitcode::USAGE);
        }

        let a_r = &a[0..2];
        let a_g = &a[2..4];
        let a_b = &a[4..6];

        let b_r = &b[0..2];
        let b_g = &b[2..4];
        let b_b = &b[4..6];

        let r = ((u16::from_str_radix(a_r, 16).unwrap() + u16::from_str_radix(b_r, 16).unwrap()) / 2) as u8;
        let g = ((u16::from_str_radix(a_g, 16).unwrap() + u16::from_str_radix(b_g, 16).unwrap()) / 2) as u8;
        let b = ((u16::from_str_radix(a_b, 16).unwrap() + u16::from_str_radix(b_b, 16).unwrap()) / 2) as u8;

        self.stack.push(self.output_rgb_dec(cor::Color{r, g, b, bold: false}));
        self.stack.push(self.output_rgb_hex_bg(cor::Color{r, g, b, bold: false}));
    }

    /* ---- higher-order functions ------------------------------------------ */

    fn c_map(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        // add ops to execute anonymous function on each stack element (backwards)
        for _ in 0..self.stack.len() {
            self.ops.insert(0, String::from("_")); // execute anonymous function
            self.ops.insert(0, String::from("rot")); // rotate stack
        }
    }

    fn c_fold(&mut self, op: &str) {
        Self::check_stack_error(self, 3, op);

        // add ops to execute anonymous function on each stack element (backwards)
        for _ in 0..(self.stack.len() - 1) {
            self.ops.insert(0, String::from("_")); // execute anonymous function
            self.ops.insert(0, String::from("rot")); // rotate stack
        }
    }

    fn c_scan(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        // add ops to execute anonymous function on each stack element (backwards)
        for _ in 0..(self.stack.len() - 1) {
            self.ops.insert(0, String::from("_")); // execute anonymous function
            self.ops.insert(0, String::from("rot")); // rotate stack
            self.ops.insert(0, String::from("dup")); // copy element
        }
        self.ops.insert(0, String::from("rot")); // rotate stack
    }

    /* ---- configuration --------------------------------------------------- */

    fn c_save_config(&mut self, _op: &str) {
        // save configuration to file
        self.save_config("comp.toml");
    }

    fn c_print_config(&mut self, _op: &str) {
        // print current configuration
        println!(
            "{}",
            self.config,
        )
    }

    /* ---- output ---------------------------------------------------------- */

    fn c_peek(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        println!(
            "  {}",
            self.theme.color_rgb(
                &self.stack[self.stack.len() - 1],
                &self.theme.white,
            ),
        );
    }

    fn c_print(&mut self, op: &str) {
        Self::check_stack_error(self, 1, op);

        let out = self.pop_stack_string();

        println!(
            "  {}",
            self.theme.color_rgb(
                &out,
                &self.theme.yellow_canary,
            ),
        );
    }

    // support functions -------------------------------------------------------

    fn is_user_function(&self, op: &str) -> Option<usize> {
        // is operator a user defined function?
        if !self.fns.is_empty() {
           for (i, f) in self.fns.iter().enumerate() {
               if f.name == op { return Some(i) }
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
    pub fn factorial(n: u128) -> u128 {
        (1..=n).product()
    }

    // greatest common divisor
    pub fn gcd(a: u64, b: u64) -> u64 {
        match b {
            0 => a,
            _ => Self::gcd(b, a % b),
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
                    self.theme.color_rgb(CONFIG_FILE, &self.theme.blue_smurf_bold),
                );
            }
            Err(error) => {
                eprintln!(
                    "  {}: configuration file [{}] could not be saved: {}",
                    self.theme.color_rgb("error", &self.theme.red_bold),
                    self.theme.color_rgb(CONFIG_FILE, &self.theme.blue_smurf_bold),
                    error,
                );
            }
        }
    }

    // load configuration file from home folder
    pub fn load_config(&mut self) {
    /*
        println!(
            "  reading configuration file [{}]",
            self.theme.color_rgb(filename, &self.theme.blue_coffee_bold),
        );
    */

        // read file contents
        let filename: String = CONFIG_FILE.to_string();

        let home_folder: String = match home::home_dir() {
            Some(dir) => dir.to_str().unwrap().to_string(),
            _ => String::from(""),
        };

        let config_filename: String = format!("{}/{}", home_folder, filename);

        let path: &Path = Path::new(&config_filename);

        if let Ok(config_file_toml) = fs::read_to_string(&path) {
            // read file success
            // deserialize configuration TOML and update configuration
            let cfg: Config = match toml::from_str(&config_file_toml) {
                Ok(c) => c,
                Err(_) => {
                    // parse fail
                    if self.config.show_warnings {
                        eprintln!(
                            "  {}: configuration file [{}] (ignored) has been corrupted or \
                            is improperly constructed for this version of comp",
                            self.theme.color_rgb("warning", &self.theme.yellow_canary_bold),
                            self.theme.color_rgb(CONFIG_FILE, &self.theme.blue_smurf_bold),
                        );
                    }
                    Config::new()
                }
            };

            self.config = cfg;
        }
    }

    // save stack file to home folder for later use (persistence)
    pub fn save_stack(&self) {
        let home_folder: String = match home::home_dir() {
            Some(dir) => dir.to_str().unwrap().to_string(),
            _ => String::from(""),
        };

        let config_filename: String = format!("{}/{}", home_folder, PERSISTENCE_FILE);

        let path: &Path = Path::new(&config_filename);

        let stack_data: String = serde_yaml::to_string(&self.stack).unwrap();

        match fs::write(path, stack_data) {
            Ok(_) => {
                /*
                println!(
                    "  stack snapshot [{}] saved",
                    self.theme.color_rgb(PERSISTENCE_FILE, &self.theme.blue_smurf_bold),
                );
                */
            }
            Err(error) => {
                eprintln!(
                    "  {}: stack snapshot [{}] could not be saved: {}",
                    self.theme.color_rgb("error", &self.theme.red_bold),
                    self.theme.color_rgb(PERSISTENCE_FILE, &self.theme.blue_smurf_bold),
                    error,
                );
            }
        }
    }

    // load stack file from home folder
    pub fn load_stack(&mut self) {
        let home_folder: String = match home::home_dir() {
            Some(dir) => dir.to_str().unwrap().to_string(),
            _ => String::from(""),
        };

        let config_filename: String = format!("{}/{}", home_folder, PERSISTENCE_FILE);

        let path: &Path = Path::new(&config_filename);

        if let Ok(stack_file_yaml) = fs::read_to_string(&path) {
            // read file success
            // deserialize stack YAML and load
            match serde_yaml::from_str(&stack_file_yaml) {
                Ok(s) => self.stack = s,
                Err(_) => {
                    // parse fail
                    if self.config.show_warnings {
                        eprintln!(
                            "  {}: stack snapshot [{}] (ignored) has been corrupted or \
                            is improperly constructed for this version of comp",
                            self.theme.color_rgb("warning", &self.theme.yellow_canary_bold),
                            self.theme.color_rgb(PERSISTENCE_FILE, &self.theme.blue_smurf_bold),
                        );
                    }
                }
            };
        }
    }

    fn output_rgb_dec(&self, color: cor::Color) -> String {
        format!(
            "{} {} {}",
            self.theme.color_rgb(
                &color.r.to_string(),
                &color,
            ),
            self.theme.color_rgb(
                &color.g.to_string(),
                &cor::Color {
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

    fn _output_rgb_hex(&self, color: cor::Color) -> String {
        format!(
            "{}",
            self.theme.color_rgb(
                &format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b),
                &color,
            ),
        )
    }

    fn output_rgb_hex_bg(&self, color: cor::Color) -> String {
        format!(
            "{}",
            self.theme.color_rgb_bg(
                &format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b),
                &color,
            ),
        )
    }

    pub fn get_cmds(&self) -> Vec<String> {
        self.cmdmap.keys().cloned().collect()
    }

    pub fn get_stack(&self) -> Vec<String> {
        self.stack.clone()
    }

}


/* unit tests --------------------------------------------------------------- */

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn test_interpreter() {
        let mut comp = Interpreter::new();

        comp.ops.push(8.to_string());
        comp.ops.push("io".to_string());
        comp.ops.push("prod".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_i64() == 40320);
    }

    #[test]
    fn test_core() {
        let mut comp = Interpreter::new();

        comp.ops.push(1.to_string());
        comp.ops.push(2.to_string());
        comp.ops.push(3.to_string());
        comp.ops.push(4.to_string());

        comp.ops.push("++".to_string());
        comp.ops.push("++".to_string());
        comp.ops.push("++".to_string());
        comp.ops.push("--".to_string());
        comp.ops.push("--".to_string());
        comp.ops.push("--".to_string());

        comp.ops.push("rot".to_string());
        comp.ops.push("rot".to_string());
        comp.ops.push("roll".to_string());
        comp.ops.push("roll".to_string());

        comp.ops.push("deg_rad".to_string());
        comp.ops.push("cos".to_string());
        comp.ops.push("acos".to_string());
        comp.ops.push("sin".to_string());
        comp.ops.push("asin".to_string());
        comp.ops.push("tan".to_string());
        comp.ops.push("atan".to_string());
        comp.ops.push("rad_deg".to_string());
        comp.ops.push("round".to_string());
        comp.ops.push("roll".to_string());
        comp.ops.push("roll".to_string());
        comp.ops.push("roll".to_string());
        comp.ops.push("roll".to_string());
        comp.ops.push("dup".to_string());
        comp.ops.push("drop".to_string());
        comp.ops.push("swap".to_string());
        comp.ops.push("swap".to_string());
        comp.ops.push("+".to_string());
        comp.ops.push("-".to_string());
        comp.ops.push("/".to_string());

        comp.ops.push(10.to_string());
        comp.ops.push("log2".to_string());
        comp.ops.push(10.to_string());
        comp.ops.push(2.to_string());
        comp.ops.push("logn".to_string());
        comp.ops.push("-".to_string());
        comp.ops.push("round".to_string());
        comp.ops.push("+".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_float() == -0.2);
    }

    #[test]
    fn test_support() {
        assert!(Interpreter::gcd(55, 10) == 5);
        assert!(Interpreter::factorial(10) == 3628800);
    }

    #[test]
    fn test_roots() {
        let mut comp = Interpreter::new();

        comp.ops.push(2.to_string());
        comp.ops.push("dup".to_string());
        comp.ops.push("sqrt".to_string());
        comp.ops.push("swap".to_string());
        comp.ops.push(32.to_string());
        comp.ops.push("^".to_string());
        comp.ops.push((32. * 2.).to_string());
        comp.ops.push("nroot".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_float() == comp.pop_stack_float());

        comp.ops.push(1.to_string());
        comp.ops.push((-2).to_string());
        comp.ops.push("chs".to_string());
        comp.ops.push("chs".to_string());
        comp.ops.push("pi".to_string());
        comp.ops.push("x".to_string());
        comp.ops.push("pi".to_string());
        comp.ops.push(2.to_string());
        comp.ops.push("^".to_string());
        comp.ops.push(1.to_string());
        comp.ops.push("+".to_string());
        comp.ops.push("proot".to_string());
        comp.ops.push("sum".to_string());
        comp.ops.push(2.to_string());
        comp.ops.push("/".to_string());
        comp.ops.push("pi".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_float() == comp.pop_stack_float());
    }

    #[test]
    #[should_panic]
    fn test_cls() {
        let mut comp = Interpreter::new();

        comp.ops.push(1.to_string());
        comp.ops.push(2.to_string());
        comp.ops.push(3.to_string());
        comp.ops.push(4.to_string());
        comp.ops.push(1.to_string());
        comp.ops.push(2.to_string());
        comp.ops.push(3.to_string());
        comp.ops.push(4.to_string());
        comp.ops.push(1.to_string());
        comp.ops.push(2.to_string());
        comp.ops.push(3.to_string());
        comp.ops.push(4.to_string());
        comp.ops.push("cls".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_float() == 0.);
    }

    #[test]
    fn test_mem() {
        let mut comp = Interpreter::new();

        comp.ops.push(1.to_string());
        comp.ops.push(2.to_string());
        comp.ops.push(3.to_string());
        comp.ops.push(4.to_string());
        comp.ops.push(1.to_string());
        comp.ops.push(2.to_string());
        comp.ops.push(3.to_string());
        comp.ops.push(4.to_string());
        comp.ops.push(1.to_string());
        comp.ops.push(2.to_string());
        comp.ops.push(3.to_string());
        comp.ops.push(4.to_string());
        comp.ops.push("chs".to_string());
        comp.ops.push("abs".to_string());
        comp.ops.push("inv".to_string());
        comp.ops.push("inv".to_string());
        comp.ops.push("pi".to_string());
        comp.ops.push("e".to_string());
        comp.ops.push(0.to_string());
        comp.ops.push("sb".to_string());
        comp.ops.push("sa".to_string());
        comp.ops.push("sc".to_string());
        comp.ops.push("cls".to_string());
        comp.ops.push("_b".to_string());
        comp.ops.push("_c".to_string());
        comp.ops.push("+".to_string());
        comp.ops.push("_a".to_string());
        comp.ops.push("+".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_float() == std::f64::consts::PI + std::f64::consts::E);
    }

    #[test]
    fn test_cmp() {
        let mut comp = Interpreter::new();

        comp.ops.push(10.to_string());
        comp.ops.push("log".to_string());
        comp.ops.push("e".to_string());
        comp.ops.push("ln".to_string());
        comp.ops.push(105.to_string());
        comp.ops.push(2.to_string());
        comp.ops.push("%".to_string());
        comp.ops.push(3049.to_string());
        comp.ops.push(1009.to_string());
        comp.ops.push("gcd".to_string());
        comp.ops.push("prod".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_float() == 1.);

        comp.ops.push(20.to_string());
        comp.ops.push("!".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_float() == 2432902008176640000.);

        comp.ops.push(20.to_string());
        comp.ops.push("tng".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_i64() == 210);
    }

    #[test]
    fn test_rand() {
        let mut comp = Interpreter::new();

        comp.ops.push(2.to_string());
        comp.ops.push("rand".to_string());
        comp.ops.push(2.to_string());
        comp.ops.push("rand".to_string());
        comp.ops.push(2.to_string());
        comp.ops.push("rand".to_string());
        comp.ops.push(2.to_string());
        comp.ops.push("rand".to_string());
        comp.ops.push(2.to_string());
        comp.ops.push("rand".to_string());
        comp.ops.push(2.to_string());
        comp.ops.push("rand".to_string());
        comp.ops.push(2.to_string());
        comp.ops.push("rand".to_string());
        comp.ops.push(2.to_string());
        comp.ops.push("rand".to_string());
        comp.ops.push(2.to_string());
        comp.ops.push("rand".to_string());
        comp.ops.push(2.to_string());
        comp.ops.push("rand".to_string());
        comp.ops.push("max".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_float() <= 1.);
    }

    #[test]
    fn test_minmax() {
        let mut comp = Interpreter::new();

        comp.ops.push(1.to_string());
        comp.ops.push(2.to_string());
        comp.ops.push("min".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_float() == 1.);


        comp.ops.push(1.to_string());
        comp.ops.push(2.to_string());
        comp.ops.push("max".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_float() == 2.);


        comp.ops.push(1.to_string());
        comp.ops.push(2.to_string());
        comp.ops.push(3.to_string());
        comp.ops.push(4.to_string());
        comp.ops.push("min_all".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_float() == 1.);


        comp.ops.push(1.to_string());
        comp.ops.push(2.to_string());
        comp.ops.push(3.to_string());
        comp.ops.push(4.to_string());
        comp.ops.push("max_all".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_float() == 4.);


        comp.ops.push((-1).to_string());
        comp.ops.push((-5).to_string());
        comp.ops.push((-10).to_string());
        comp.ops.push("minmax".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_float() == -1.);
        assert!(comp.pop_stack_float() == -10.);
    }

    #[test]
    fn test_conv() {
        let mut comp = Interpreter::new();

        comp.ops.push(100.to_string());
        comp.ops.push("c_f".to_string());
        comp.ops.push("f_c".to_string());
        comp.ops.push("dec_hex".to_string());
        comp.ops.push("hex_bin".to_string());
        comp.ops.push("bin_hex".to_string());
        comp.ops.push("hex_dec".to_string());
        comp.ops.push("dec_bin".to_string());
        comp.ops.push("bin_dec".to_string());
        comp.ops.push("ft_m".to_string());
        comp.ops.push("m_ft".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_float() == 100.);
    }

    #[test]
    fn test_avg() {
        let mut comp = Interpreter::new();

        comp.ops.push((-2).to_string());
        comp.ops.push(2.to_string());
        comp.ops.push("avg".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_float() == 0.);


        comp.ops.push(1.to_string());
        comp.ops.push(2.to_string());
        comp.ops.push(3.to_string());
        comp.ops.push(4.to_string());
        comp.ops.push("avg_all".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_float() == 2.5);
    }

    #[test]
    fn test_misc() {
        let mut comp = Interpreter::new();

        comp.ops.push(10.1.to_string());
        comp.ops.push("round".to_string());
        comp.ops.push(10.1.to_string());
        comp.ops.push("floor".to_string());
        comp.ops.push(10.1.to_string());
        comp.ops.push("ceil".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_u64() == 11);
        assert!(comp.pop_stack_u64() == 10);
        assert!(comp.pop_stack_u64() == 10);


        comp.ops.push((-99).to_string());
        comp.ops.push("sgn".to_string());
        comp.ops.push(109.to_string());
        comp.ops.push("sgn".to_string());
        comp.ops.push(0.to_string());
        comp.ops.push("sgn".to_string());
        comp.ops.push("sum".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_i64() == 0);


        comp.ops.push("cls".to_string());
        comp.ops.push(28.to_string());
        comp.ops.push("divs".to_string());
        comp.ops.push("sum".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_i64() == 28);
    }

    #[test]
    fn test_stack() {
        let mut comp = Interpreter::new();

        comp.ops.push(1.to_string());
        comp.ops.push(2.to_string());
        comp.ops.push(3.to_string());
        comp.ops.push(4.to_string());
        comp.ops.push(5.to_string());
        comp.ops.push(3.to_string());
        comp.ops.push("rotn".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_i64() == 3);


        comp.ops.push("cls".to_string());
        comp.ops.push(1.to_string());
        comp.ops.push(2.to_string());
        comp.ops.push(3.to_string());
        comp.ops.push(4.to_string());
        comp.ops.push(5.to_string());
        comp.ops.push(3.to_string());
        comp.ops.push("rolln".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_i64() == 2);


        comp.ops.push("cls".to_string());
        comp.ops.push(1.to_string());
        comp.ops.push(2.to_string());
        comp.ops.push(3.to_string());
        comp.ops.push(4.to_string());
        comp.ops.push(5.to_string());
        comp.ops.push("flip".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_i64() == 1);


        comp.ops.push("flip".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_i64() == 5);

    }

    #[test]
    fn test_binary_ops() {
        let mut comp = Interpreter::new();

        comp.ops.push(10.to_string());
        comp.ops.push(6.to_string());
        comp.ops.push("and".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_u64() == 2);


        comp.ops.push(10.to_string());
        comp.ops.push(6.to_string());
        comp.ops.push("nand".to_string());
        comp.ops.push("not".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_u64() == 2);


        comp.ops.push(10.to_string());
        comp.ops.push(6.to_string());
        comp.ops.push("or".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_u64() == 14);


        comp.ops.push(10.to_string());
        comp.ops.push(6.to_string());
        comp.ops.push("nor".to_string());
        comp.ops.push("not".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_u64() == 14);


        comp.ops.push(10.to_string());
        comp.ops.push(6.to_string());
        comp.ops.push("xor".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_u64() == 12);


        comp.ops.push(341.to_string());
        comp.ops.push("ones".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_u64() == 5);

    }

} // unit_test