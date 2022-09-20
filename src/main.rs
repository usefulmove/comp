use colored::ColoredString;
use std::{env, fs};
use std::path::Path;
use std::process::exit;

mod comp;
mod mona;

const RELEASE_STATE: &str = "b";

/*

    note: base data structure is a vector (linked
    list) used as a stack. atoms on the list are
    either be symbols (commands) or values. each
    calculation is a list of operations that are
    processed in order of occurrence. this is an
    implementation of a list processor (lisp) for
    reverse polish notation s-expressions (sexp).

      operations list structure
        ( object : command or value )
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
    // enable or disable backtrace on error
    env::set_var("RUST_BACKTRACE", "0");

    // color theme
    let theme = coq::Theme::new();

    // construct command interpreter
    let mut interpreter = comp::Interpreter::new();

    // get command arguments
    let mut args: Vec<String> = env::args().collect();

    // if no arguments are passed, behave as if help flag was passed
    if args.len() <= 1 {
        args.push(String::from("help"));
    }

    match args[1].as_str() {
        "--commands" | "--" => {
            // display available commands
            let mut cmds: Vec<String> = interpreter.get_cmds();
            cmds.sort_unstable();

            for cmd in cmds {
                print!("{} ", theme.color_rgb(&cmd, &theme.blue_smurf));
            }
            println!();
            return;
        }
        "--file" | "-f" => {
            // read operations list input from file
            if args.get(2).is_none() {
                eprintln!(
                    "  {}: no file path provided",
                    theme.color_rgb("error", &theme.red_bold),
                );
                exit(exitcode::NOINPUT);
            }
            // read file contents
            let filename: String = args[2].to_string();
            let path: &Path = Path::new(&filename);

            let file_contents: String = match fs::read_to_string(&path) {
                Ok(content) => content,
                Err(error) => {
                    eprintln!(
                        "  {}: could not read [{}]: {error}",
                        theme.color_rgb("error", &theme.red_bold),
                        theme.color_rgb(&path.display().to_string(), &theme.blue_coffee_bold),
                    );
                    exit(exitcode::OSFILE);
                }
            };
            // create operations list vector from file contents - split elements
            let operations = file_contents
                .split_whitespace()
                .map(|x| x.to_string());

            interpreter.ops.extend(operations);

            // add additional operations from command line
            if args.get(3).is_some() {
                interpreter.ops.extend((&args[3..]).to_vec());
            }
        }
        "--help" | "help" => {
            // display command usage information
            show_help();
            return;
        }
        "mona" => {
            println!("{}", mona::MONA);
            return;
        }
        "--version" | "version" => {
            // display version information
            show_version();
            return;
        }
        _ => {
            // read operations list input from command line arguments
            interpreter.ops = (&args[1..]).to_vec();
        }

    };

    // load configuration
    interpreter.read_and_apply_config("comp.toml");

    // process operations list ( ops list was loaded into the interpreter
    // in the match statement above based on command line arguments )
    interpreter.process_ops();

    /* display stack to user */
    output_stack(
        &mut interpreter.stack.clone(),
        interpreter.config.show_stack_level,
        interpreter.config.monochrome,
    );

    exit(exitcode::OK);
} // main

struct BoxedClosure<'a> {
    f: Box<dyn Fn(&str) -> ColoredString + 'a>,
}

impl<'a> BoxedClosure<'a> {
    fn new<F>(closure: F) -> Self
    where F: Fn(&str) -> ColoredString + 'a,
    {
        BoxedClosure {
            f: Box::new(closure),
        }
    }
}

fn show_help() {
    // color theme
    let theme = coq::Theme::new();

    println!();
    println!(
        "{}",
        theme.color_rgb("COMP", &theme.cream_bold)
    );
    println!(
        "    {} {} {} {}",
        theme.color_rgb("comp", &theme.grey_mouse),
        theme.color_rgb("..", &theme.charcoal_cream),
        theme.color_rgb("command interpreter", &theme.cream_bold),
        theme.color_rgb(env!("CARGO_PKG_VERSION"), &theme.grey_mouse),
    );
    println!();
    println!(
        "{}",
        theme.color_rgb("USAGE", &theme.cream_bold)
    );
    println!(
        "    {} {} {}",
        theme.color_rgb("comp", &theme.grey_mouse),
        theme.color_rgb("[OPTIONS]", &theme.cream_bold),
        theme.color_rgb("<list>", &theme.blue_coffee_bold),
    );
    println!(
        "    {} {} {}",
        theme.color_rgb("comp", &theme.grey_mouse),
        theme.color_rgb("-f", &theme.yellow_canary),
        theme.color_rgb("<path>", &theme.blue_coffee_bold),
    );
    println!();
    println!(
        "{}",
        theme.color_rgb("OPTIONS", &theme.cream_bold)
    );
    println!(
        "        {}      show version",
        theme.color_rgb("--version", &theme.yellow_canary),
    );
    println!(
        "    {}{} {}         read from file at the specified path",
        theme.color_rgb("-f", &theme.yellow_canary),
        theme.color_rgb(",", &theme.grey_mouse),
        theme.color_rgb("--file", &theme.yellow_canary),
    );
    println!(
        "    {}{} {}     display available commands",
        theme.color_rgb("--", &theme.yellow_canary),
        theme.color_rgb(",", &theme.grey_mouse),
        theme.color_rgb("--commands", &theme.yellow_canary),
    );
    println!(
        "        {}         show help information",
        theme.color_rgb("--help", &theme.yellow_canary),
    );
    println!();
    println!(
        "{}",
        theme.color_rgb("DESCRIPTION", &theme.cream_bold)
    );
    println!(
        "The comp interpreter takes a {} sequence of (postfix) operations as \
    command line arguments or a {} argument that specifies the path to a file \
    containing a list of operations. Each operation is either a command ({}) \
    or a {}. The available commands are listed below.",
        theme.color_rgb("<list>", &theme.blue_coffee_bold),
        theme.color_rgb("<path>", &theme.blue_coffee_bold),
        theme.color_rgb("symbol", &theme.green_eggs_bold),
        theme.color_rgb("value", &theme.blue_smurf_bold),
    );
    println!();
    println!(
        "    Usage Guide:   {}",
        theme.color_rgb("https://github.com/usefulmove/comp/blob/main/USAGE.md", &theme.grey_mouse),
    );
    println!(
        "    Repository:    {}",
        theme.color_rgb("https://github.com/usefulmove/comp#readme", &theme.grey_mouse),
    );
    println!();
    println!(
        "{}",
        theme.color_rgb("EXAMPLES", &theme.cream_bold)
    );
    println!(
        "    {} {} {}                  {}",
        theme.color_rgb("comp", &theme.grey_mouse),
        theme.color_rgb("1 2", &theme.blue_smurf_bold),
        theme.color_rgb("+", &theme.green_eggs_bold),
        theme.color_rgb("add 1 and 2", &theme.cream_bold),
    );
    println!(
        "    {} {} {}                  {}",
        theme.color_rgb("comp", &theme.grey_mouse),
        theme.color_rgb("5 2", &theme.blue_smurf_bold),
        theme.color_rgb("/", &theme.green_eggs_bold),
        theme.color_rgb("divide 5 by 2", &theme.cream_bold),
    );
    println!(
        "    {} {} {} {} {}      {}",
        theme.color_rgb("comp", &theme.grey_mouse),
        theme.color_rgb("3", &theme.blue_smurf_bold),
        theme.color_rgb("dup x", &theme.green_eggs_bold),
        theme.color_rgb("4", &theme.blue_smurf_bold),
        theme.color_rgb("dup x +", &theme.green_eggs_bold),
        theme.color_rgb("sum of the squares of 3 and 4", &theme.cream_bold),
    );
    println!();
}

fn show_version() {
    // color theme
    let theme = coq::Theme::new();

    let version: &str = env!("CARGO_PKG_VERSION");
    println!(
        "  {} {}{}",
        theme.color_rgb("comp", &theme.grey_mouse),
        theme.color_rgb(version, &theme.blue_smurf_bold),
        theme.color_rgb(RELEASE_STATE, &theme.white_bold),
    );
}

fn output_stack(stack: &mut Vec<String>, annotate: bool, monochrome: bool) {
    // color theme
    let theme = coq::Theme::new();

    let mut color_stack_top_closure = BoxedClosure::new(
        |x| theme.color_rgb(x, &theme.blue_coffee_bold)
    );
    let mut color_stack_closure = BoxedClosure::new(
        |x| theme.color_rgb(x, &theme.blue_smurf)
    );
    let mut color_annotate_closure = BoxedClosure::new(
        |x| theme.color_blank(x)
    );
    if annotate {
        color_annotate_closure = BoxedClosure::new(
            |x| theme.color_rgb(x, &theme.charcoal_cream)
        );
    }
    if monochrome {
        color_stack_top_closure = BoxedClosure::new(
            |x| theme.color_rgb(x, &theme.white_bold)
        );
        color_stack_closure = BoxedClosure::new(
            |x| theme.color_rgb(x, &theme.white)
        );
    }

    let len = stack.len();
    stack.iter()
        .enumerate()
        .for_each(|(i, ent)| {

        let level = len - i;

        match level {
            1 => {
                println!( // top element
                    "{}  {}",
                    (color_annotate_closure.f)(level_map(level)),
                    (color_stack_top_closure.f)(&ent),
                )
            }
            _ => {
                println!( // all other elements
                    "{}  {}",
                    (color_annotate_closure.f)(level_map(level)),
                    (color_stack_closure.f)(&ent),
                )
            }
        }

        });

}

fn level_map(level: usize) -> &'static str {
    let ret: &str = match level {
        1 => "a.",
        2 => "b.",
        3 => "c.",
        4 => "d.",
        5 => "e.",
        6 => "f.",
        7 => "g.",
        8 => "h.",
        _ => "  ",
    };
    ret
}


/* unit tests --------------------------------------------------------------- */

#[cfg(test)]
mod unit_test {
    use crate::comp::Interpreter;

    #[test]
    fn test_interpreter() {
        let mut comp = Interpreter::new();

        comp.ops.push(8.to_string());
        comp.ops.push("io".to_string());
        comp.ops.push("prod".to_string());

        comp.process_ops();

        assert!(comp.pop_stack_int() == 40320);
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
        assert!(Interpreter::factorial(10.) == 3628800.);
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

        assert!(comp.pop_stack_int() == 210);
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

        comp.stack.push(100.to_string());
        comp.c_celfah("");
        comp.c_fahcel("");
        comp.c_dechex("");
        comp.c_hexbin("");
        comp.c_binhex("");
        comp.c_hexdec("");
        comp.c_decbin("");
        comp.c_bindec("");
        comp.c_ftm("");
        comp.c_mft("");

        assert!(comp.pop_stack_float() == 100.);
    }

    #[test]
    fn test_avg() {
        let mut comp = Interpreter::new();

        comp.stack.push((-2).to_string());
        comp.stack.push(2.to_string());
        comp.c_avg("");

        assert!(comp.pop_stack_float() == 0.);

        comp.stack.push(1.to_string());
        comp.stack.push(2.to_string());
        comp.stack.push(3.to_string());
        comp.stack.push(4.to_string());
        comp.c_avg_all("");

        assert!(comp.pop_stack_float() == 2.5);
    }

    #[test]
    fn test_misc() {
        let mut comp = Interpreter::new();

        comp.stack.push(10.1.to_string());
        comp.c_round("");
        comp.stack.push(10.1.to_string());
        comp.c_floor("");
        comp.stack.push(10.1.to_string());
        comp.c_ceiling("");

        assert!(comp.pop_stack_uint() == 11);
        assert!(comp.pop_stack_uint() == 10);
        assert!(comp.pop_stack_uint() == 10);

        comp.stack.push((-99).to_string());
        comp.c_sign("");
        comp.stack.push(109.to_string());
        comp.c_sign("");
        comp.stack.push(0.to_string());
        comp.c_sign("");
        comp.c_sum("");

        assert!(comp.pop_stack_int() == 0);
    }

    #[test]
    fn test_stack() {
        let mut comp = Interpreter::new();

        comp.stack.push(1.to_string());
        comp.stack.push(2.to_string());
        comp.stack.push(3.to_string());
        comp.stack.push(4.to_string());
        comp.stack.push(5.to_string());
        comp.stack.push(3.to_string());
        comp.c_rotn("");

        assert!(comp.pop_stack_int() == 3);


        comp.c_cls("");
        comp.stack.push(1.to_string());
        comp.stack.push(2.to_string());
        comp.stack.push(3.to_string());
        comp.stack.push(4.to_string());
        comp.stack.push(5.to_string());
        comp.stack.push(3.to_string());
        comp.c_rolln("");

        assert!(comp.pop_stack_int() == 2);


        comp.c_cls("");
        comp.stack.push(1.to_string());
        comp.stack.push(2.to_string());
        comp.stack.push(3.to_string());
        comp.stack.push(4.to_string());
        comp.stack.push(5.to_string());
        comp.c_flip("");

        assert!(comp.pop_stack_int() == 1);

        comp.c_flip("");

        assert!(comp.pop_stack_int() == 5);

    }
} // unit_test