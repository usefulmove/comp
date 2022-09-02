use colored::ColoredString;
use std::env;
use std::fs;
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
    let theme: coq::Theme = coq::Theme::new();

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

            let file_contents = fs::read_to_string(&path);
            if let Err(ref error) = file_contents {
                eprintln!(
                    "  {}: could not read [{}]: {error}",
                    theme.color_rgb("error", &theme.red_bold),
                    theme.color_rgb(&path.display().to_string(), &theme.blue_coffee_bold),
                );
                exit(exitcode::OSFILE);
            }

            let file_contents: String = file_contents.unwrap();

            // create operations list vector from file contents - split elements
            let operations = file_contents.split_whitespace().map(|x| x.to_string());
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

    // process operations list
    interpreter.process_ops();

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
    let theme: coq::Theme = coq::Theme::new();

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
        theme.color_rgb("-f", &theme.orange_sherbet_bold),
        theme.color_rgb("<path>", &theme.blue_coffee_bold),
    );
    println!();
    println!(
        "{}",
        theme.color_rgb("OPTIONS", &theme.cream_bold)
    );
    println!(
        "        {}      show version",
        theme.color_rgb("--version", &theme.orange_sherbet_bold),
    );
    println!(
        "    {}{} {}         read from file at the specified path",
        theme.color_rgb("-f", &theme.orange_sherbet_bold),
        theme.color_rgb(",", &theme.grey_mouse),
        theme.color_rgb("--file", &theme.orange_sherbet_bold),
    );
    println!(
        "    {}{} {}     display available commands",
        theme.color_rgb("--", &theme.orange_sherbet_bold),
        theme.color_rgb(",", &theme.grey_mouse),
        theme.color_rgb("--commands", &theme.orange_sherbet_bold),
    );
    println!(
        "        {}         show help information",
        theme.color_rgb("--help", &theme.orange_sherbet_bold),
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
    let theme: coq::Theme = coq::Theme::new();

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
    let theme: coq::Theme = coq::Theme::new();

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

    while !stack.is_empty() {
        let level: u32 = stack.len() as u32;
        match level {
            1 => {
                println!(
                    // top element
                    "{}  {}",
                    (color_annotate_closure.f)(level_map(level)),
                    (color_stack_top_closure.f)(&stack.remove(0)),
                )
            }
            _ => {
                println!(
                    // all other elements
                    "{}  {}",
                    (color_annotate_closure.f)(level_map(level)),
                    (color_stack_closure.f)(&stack.remove(0)),
                )
            }
        }
    }
}

fn level_map(level: u32) -> &'static str {
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


/* --- unit tests --- */

#[cfg(test)]
mod unit_test {
    use crate::comp::Interpreter;

    #[test]
    fn test_core() {
        let mut intp = Interpreter::new();

        intp.stack.push(1.to_string());
        intp.stack.push(2.to_string());
        intp.stack.push(3.to_string());
        intp.stack.push(4.to_string());

        intp.c_add_one("");
        intp.c_add_one("");
        intp.c_add_one("");
        intp.c_sub_one("");
        intp.c_sub_one("");
        intp.c_sub_one("");

        intp.c_rot("");
        intp.c_rot("");
        intp.c_roll("");
        intp.c_roll("");

        intp.c_degrad("");
        intp.c_cos("");
        intp.c_acos("");
        intp.c_sin("");
        intp.c_asin("");
        intp.c_tan("");
        intp.c_atan("");
        intp.c_raddeg("");
        intp.c_round("");
        intp.c_roll("");
        intp.c_roll("");
        intp.c_roll("");
        intp.c_roll("");
        intp.c_dup("");
        intp.c_drop("");
        intp.c_swap("");
        intp.c_swap("");
        intp.c_add("");
        intp.c_sub("");
        intp.c_div("");

        intp.stack.push(10.to_string());
        intp.c_log2("");
        intp.stack.push(10.to_string());
        intp.stack.push(2.to_string());
        intp.c_logn("");
        intp.c_sub("");
        intp.c_round("");
        intp.c_add("");

        assert!(intp.pop_stack_float() == -0.2);
    }

    #[test]
    fn test_support() {
        assert!(Interpreter::gcd(55, 10) == 5);
        assert!(Interpreter::factorial(10.) == 3628800.);
    }

    #[test]
    fn test_roots() {
        let mut intp = Interpreter::new();

        intp.stack.push(2.to_string());
        intp.c_dup("");
        intp.c_sqrt("");
        intp.c_swap("");
        intp.stack.push(32.to_string());
        intp.c_exp("");
        intp.stack.push((32. * 2.).to_string());
        intp.c_throot("");

        assert!(intp.pop_stack_float() == intp.pop_stack_float());

        intp.stack.push(1.to_string());
        intp.stack.push((-2.).to_string());
        intp.c_chs("");
        intp.c_chs("");
        intp.c_pi("");
        intp.c_mult("");
        intp.c_pi("");
        intp.stack.push(2.to_string());
        intp.c_exp("");
        intp.stack.push(1.to_string());
        intp.c_add("");
        intp.c_proot("");
        intp.c_sum("");
        intp.stack.push(2.to_string());
        intp.c_div("");
        intp.c_pi("");

        assert!(intp.pop_stack_float() == intp.pop_stack_float());
    }

    #[test]
    #[should_panic]
    fn test_cls() {
        let mut intp = Interpreter::new();

        intp.stack.push(1.to_string());
        intp.stack.push(2.to_string());
        intp.stack.push(3.to_string());
        intp.stack.push(4.to_string());
        intp.stack.push(1.to_string());
        intp.stack.push(2.to_string());
        intp.stack.push(3.to_string());
        intp.stack.push(4.to_string());
        intp.stack.push(1.to_string());
        intp.stack.push(2.to_string());
        intp.stack.push(3.to_string());
        intp.stack.push(4.to_string());
        intp.c_cls("");

        assert!(intp.pop_stack_float() == 0.);
    }

    #[test]
    fn test_mem() {
        let mut intp = Interpreter::new();

        intp.stack.push(1.to_string());
        intp.stack.push(2.to_string());
        intp.stack.push(3.to_string());
        intp.stack.push(4.to_string());
        intp.stack.push(1.to_string());
        intp.stack.push(2.to_string());
        intp.stack.push(3.to_string());
        intp.stack.push(4.to_string());
        intp.stack.push(1.to_string());
        intp.stack.push(2.to_string());
        intp.stack.push(3.to_string());
        intp.stack.push(4.to_string());
        intp.c_chs("");
        intp.c_abs("");
        intp.c_inv("");
        intp.c_inv("");
        intp.c_pi("");
        intp.c_euler("");
        intp.stack.push(0.to_string());
        intp.c_store_b(""); // 0
        intp.c_store_a(""); // e
        intp.c_store_c(""); // pi
        intp.c_cls("");
        intp.c_push_b(""); // 0
        intp.c_push_c(""); // pi
        intp.c_add("");
        intp.c_push_a(""); // e
        intp.c_add("");

        assert!(intp.pop_stack_float() == std::f64::consts::PI + std::f64::consts::E);
    }

    #[test]
    fn test_cmp() {
        let mut intp = Interpreter::new();

        intp.stack.push(10.to_string());
        intp.c_log10("");
        intp.c_euler("");
        intp.c_ln("");
        intp.stack.push(105.to_string());
        intp.stack.push(2.to_string());
        intp.c_mod("");
        intp.stack.push(3049.to_string());
        intp.stack.push(1009.to_string());
        intp.c_gcd("");
        intp.c_product("");

        assert!(intp.pop_stack_float() == 1.);

        intp.stack.push(20.to_string());
        intp.c_fact("");

        assert!(intp.pop_stack_float() == 2432902008176640000.);
    }

    #[test]
    fn test_rand() {
        let mut intp = Interpreter::new();

        intp.stack.push(2.to_string());
        intp.c_rand("");
        intp.stack.push(2.to_string());
        intp.c_rand("");
        intp.stack.push(2.to_string());
        intp.c_rand("");
        intp.stack.push(2.to_string());
        intp.c_rand("");
        intp.stack.push(2.to_string());
        intp.c_rand("");
        intp.stack.push(2.to_string());
        intp.c_rand("");
        intp.stack.push(2.to_string());
        intp.c_rand("");
        intp.stack.push(2.to_string());
        intp.c_rand("");
        intp.stack.push(2.to_string());
        intp.c_rand("");
        intp.stack.push(2.to_string());
        intp.c_rand("");
        intp.c_max("");

        assert!(intp.pop_stack_float() <= 1.);
    }

    #[test]
    fn test_minmax() {
        let mut intp = Interpreter::new();

        intp.stack.push(1.to_string());
        intp.stack.push(2.to_string());
        intp.c_min("");

        assert!(intp.pop_stack_float() == 1.);

        intp.stack.push(1.to_string());
        intp.stack.push(2.to_string());
        intp.c_max("");

        assert!(intp.pop_stack_float() == 2.);

        intp.stack.push(1.to_string());
        intp.stack.push(2.to_string());
        intp.stack.push(3.to_string());
        intp.stack.push(4.to_string());
        intp.c_min_all("");

        assert!(intp.pop_stack_float() == 1.);

        intp.stack.push(1.to_string());
        intp.stack.push(2.to_string());
        intp.stack.push(3.to_string());
        intp.stack.push(4.to_string());
        intp.c_max_all("");

        assert!(intp.pop_stack_float() == 4.);

        intp.stack.push((-1).to_string());
        intp.stack.push((-5).to_string());
        intp.stack.push((-10).to_string());
        intp.c_minmax("");

        assert!(intp.pop_stack_float() == -1.);
        assert!(intp.pop_stack_float() == -10.);
    }


    /* unit tests ----------------------------------------------------------- */

    #[test]
    fn test_conv() {
        let mut intp = Interpreter::new();

        intp.stack.push(100.to_string());
        intp.c_celfah("");
        intp.c_fahcel("");
        intp.c_dechex("");
        intp.c_hexbin("");
        intp.c_binhex("");
        intp.c_hexdec("");
        intp.c_decbin("");
        intp.c_bindec("");
        intp.c_ftm("");
        intp.c_mft("");

        assert!(intp.pop_stack_float() == 100.);
    }

    #[test]
    fn test_avg() {
        let mut intp = Interpreter::new();

        intp.stack.push((-2).to_string());
        intp.stack.push(2.to_string());
        intp.c_avg("");

        assert!(intp.pop_stack_float() == 0.);

        intp.stack.push(1.to_string());
        intp.stack.push(2.to_string());
        intp.stack.push(3.to_string());
        intp.stack.push(4.to_string());
        intp.c_avg_all("");

        assert!(intp.pop_stack_float() == 2.5);
    }

    #[test]
    fn test_misc() {
        let mut intp = Interpreter::new();

        intp.stack.push(10.1.to_string());
        intp.c_round("");
        intp.stack.push(10.1.to_string());
        intp.c_floor("");
        intp.stack.push(10.1.to_string());
        intp.c_ceiling("");

        assert!(intp.pop_stack_uint() == 11);
        assert!(intp.pop_stack_uint() == 10);
        assert!(intp.pop_stack_uint() == 10);
    }

} // unit_test