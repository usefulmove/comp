use colored::ColoredString;
use std::env;
use std::fs;
use std::path::Path;

mod comp;
mod mona;

const RELEASE_STATE: &str = "d";

/*

    note: base data structure is a vector (linked
    list) used as a stack. atoms on the list are
    either be symbols (commands) or values. each
    calculation is a list of operations that are
    processed in order of occurrence. this is an
    implementation of a list processor (lisp) for
    reverse polish notation s-expressions (sexp).

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
            let mut cmds: Vec<&str> = interpreter.get_cmds();
            cmds.sort_unstable();

            for cmd in cmds {
                print!("{} ", theme.color_rgb(cmd, &theme.blue_smurf));
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
                std::process::exit(exitcode::NOINPUT);
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
                std::process::exit(exitcode::OSFILE);
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

    std::process::exit(exitcode::OK);
} // main

struct BoxedClosure<'a> {
    f: Box<dyn Fn(&str) -> ColoredString + 'a>,
}

impl<'a> BoxedClosure<'a> {
    fn new<F>(f: F) -> Self
    where
        F: Fn(&str) -> ColoredString + 'a,
    {
        BoxedClosure {
            f: Box::new(f),
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

    let mut clos_color_stack_top = BoxedClosure::new(
        |x| theme.color_rgb(x, &theme.blue_coffee_bold)
    );
    let mut clos_color_stack = BoxedClosure::new(
        |x| theme.color_rgb(x, &theme.blue_smurf)
    );
    let mut clos_color_annotate = BoxedClosure::new(
        |x| theme.color_blank(x)
    );
    if annotate {
        clos_color_annotate = BoxedClosure::new(
            |x| theme.color_rgb(x, &theme.charcoal_cream)
        );
    }
    if monochrome {
        clos_color_stack_top = BoxedClosure::new(
            |x| theme.color_rgb(x, &theme.white_bold)
        );
        clos_color_stack = BoxedClosure::new(
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
                    (clos_color_annotate.f)(level_map(level)),
                    (clos_color_stack_top.f)(&stack.remove(0)),
                )
            }
            _ => {
                println!(
                    // all other elements
                    "{}  {}",
                    (clos_color_annotate.f)(level_map(level)),
                    (clos_color_stack.f)(&stack.remove(0)),
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


#[cfg(test)]
mod unit_test {
    use crate::comp::Interpreter;

    #[test]
    fn test_core() {
        let mut test_interc = Interpreter::new();

        test_interc.stack.push(1.to_string());
        test_interc.stack.push(2.to_string());
        test_interc.stack.push(3.to_string());
        test_interc.stack.push(4.to_string());

        test_interc.c_add_one("");
        test_interc.c_add_one("");
        test_interc.c_add_one("");
        test_interc.c_sub_one("");
        test_interc.c_sub_one("");
        test_interc.c_sub_one("");

        test_interc.c_rot("");
        test_interc.c_rot("");
        test_interc.c_roll("");
        test_interc.c_roll("");

        test_interc.c_degrad("");
        test_interc.c_cos("");
        test_interc.c_acos("");
        test_interc.c_sin("");
        test_interc.c_asin("");
        test_interc.c_tan("");
        test_interc.c_atan("");
        test_interc.c_raddeg("");
        test_interc.c_round("");
        test_interc.c_roll("");
        test_interc.c_roll("");
        test_interc.c_roll("");
        test_interc.c_roll("");
        test_interc.c_dup("");
        test_interc.c_drop("");
        test_interc.c_swap("");
        test_interc.c_swap("");
        test_interc.c_add("");
        test_interc.c_sub("");
        test_interc.c_div("");

        test_interc.stack.push(10.to_string());
        test_interc.c_log2("");
        test_interc.stack.push(10.to_string());
        test_interc.stack.push(2.to_string());
        test_interc.c_logn("");
        test_interc.c_sub("");
        test_interc.c_round("");
        test_interc.c_add("");

        assert!(test_interc.pop_stack_float() == -0.2);
    }

    #[test]
    fn test_support() {
        assert!(Interpreter::gcd(55, 10) == 5);
        assert!(Interpreter::factorial(10.) == 3628800.);
    }

    #[test]
    fn test_roots() {
        let mut test_interc = Interpreter::new();

        test_interc.stack.push(2.to_string());
        test_interc.c_dup("");
        test_interc.c_sqrt("");
        test_interc.c_swap("");
        test_interc.stack.push(32.to_string());
        test_interc.c_exp("");
        test_interc.stack.push((32. * 2.).to_string());
        test_interc.c_throot("");

        assert!(test_interc.pop_stack_float() == test_interc.pop_stack_float());

        test_interc.stack.push(1.to_string());
        test_interc.stack.push((-2.).to_string());
        test_interc.c_chs("");
        test_interc.c_chs("");
        test_interc.c_pi("");
        test_interc.c_mult("");
        test_interc.c_pi("");
        test_interc.stack.push(2.to_string());
        test_interc.c_exp("");
        test_interc.stack.push(1.to_string());
        test_interc.c_add("");
        test_interc.c_proot("");
        test_interc.c_add_all("");
        test_interc.stack.push(2.to_string());
        test_interc.c_div("");
        test_interc.c_pi("");

        assert!(test_interc.pop_stack_float() == test_interc.pop_stack_float());
    }

    #[test]
    #[should_panic]
    fn test_cls() {
        let mut test_interc = Interpreter::new();

        test_interc.stack.push(1.to_string());
        test_interc.stack.push(2.to_string());
        test_interc.stack.push(3.to_string());
        test_interc.stack.push(4.to_string());
        test_interc.stack.push(1.to_string());
        test_interc.stack.push(2.to_string());
        test_interc.stack.push(3.to_string());
        test_interc.stack.push(4.to_string());
        test_interc.stack.push(1.to_string());
        test_interc.stack.push(2.to_string());
        test_interc.stack.push(3.to_string());
        test_interc.stack.push(4.to_string());
        test_interc.c_cls("");

        assert!(test_interc.pop_stack_float() == 0.);
    }

    #[test]
    fn test_mem() {
        let mut test_interc = Interpreter::new();

        test_interc.stack.push(1.to_string());
        test_interc.stack.push(2.to_string());
        test_interc.stack.push(3.to_string());
        test_interc.stack.push(4.to_string());
        test_interc.stack.push(1.to_string());
        test_interc.stack.push(2.to_string());
        test_interc.stack.push(3.to_string());
        test_interc.stack.push(4.to_string());
        test_interc.stack.push(1.to_string());
        test_interc.stack.push(2.to_string());
        test_interc.stack.push(3.to_string());
        test_interc.stack.push(4.to_string());
        test_interc.c_chs("");
        test_interc.c_abs("");
        test_interc.c_inv("");
        test_interc.c_inv("");
        test_interc.c_pi("");
        test_interc.c_euler("");
        test_interc.stack.push(0.to_string());
        test_interc.c_store_b(""); // 0
        test_interc.c_store_a(""); // e
        test_interc.c_store_c(""); // pi
        test_interc.c_cls("");
        test_interc.c_push_b(""); // 0
        test_interc.c_push_c(""); // pi
        test_interc.c_add("");
        test_interc.c_push_a(""); // e
        test_interc.c_add("");

        assert!(test_interc.pop_stack_float() == std::f64::consts::PI + std::f64::consts::E);
    }

    #[test]
    fn test_cmp() {
        let mut test_interc = Interpreter::new();

        test_interc.stack.push(10.to_string());
        test_interc.c_log10("");
        test_interc.c_euler("");
        test_interc.c_ln("");
        test_interc.stack.push(105.to_string());
        test_interc.stack.push(2.to_string());
        test_interc.c_mod("");
        test_interc.stack.push(3049.to_string());
        test_interc.stack.push(1009.to_string());
        test_interc.c_gcd("");
        test_interc.c_mult_all("");

        assert!(test_interc.pop_stack_float() == 1.);

        test_interc.stack.push(20.to_string());
        test_interc.c_fact("");

        assert!(test_interc.pop_stack_float() == 2432902008176640000.);
    }

    #[test]
    fn test_rand() {
        let mut test_interc = Interpreter::new();

        test_interc.stack.push(2.to_string());
        test_interc.c_rand("");
        test_interc.stack.push(2.to_string());
        test_interc.c_rand("");
        test_interc.stack.push(2.to_string());
        test_interc.c_rand("");
        test_interc.stack.push(2.to_string());
        test_interc.c_rand("");
        test_interc.stack.push(2.to_string());
        test_interc.c_rand("");
        test_interc.stack.push(2.to_string());
        test_interc.c_rand("");
        test_interc.stack.push(2.to_string());
        test_interc.c_rand("");
        test_interc.stack.push(2.to_string());
        test_interc.c_rand("");
        test_interc.stack.push(2.to_string());
        test_interc.c_rand("");
        test_interc.stack.push(2.to_string());
        test_interc.c_rand("");
        test_interc.c_max("");

        assert!(test_interc.pop_stack_float() <= 1.);
    }

    #[test]
    fn test_minmax() {
        let mut test_interc = Interpreter::new();

        test_interc.stack.push(1.to_string());
        test_interc.stack.push(2.to_string());
        test_interc.c_min("");

        assert!(test_interc.pop_stack_float() == 1.);

        test_interc.stack.push(1.to_string());
        test_interc.stack.push(2.to_string());
        test_interc.c_max("");

        assert!(test_interc.pop_stack_float() == 2.);

        test_interc.stack.push(1.to_string());
        test_interc.stack.push(2.to_string());
        test_interc.stack.push(3.to_string());
        test_interc.stack.push(4.to_string());
        test_interc.c_min_all("");

        assert!(test_interc.pop_stack_float() == 1.);

        test_interc.stack.push(1.to_string());
        test_interc.stack.push(2.to_string());
        test_interc.stack.push(3.to_string());
        test_interc.stack.push(4.to_string());
        test_interc.c_max_all("");

        assert!(test_interc.pop_stack_float() == 4.);
    }


    /* unit tests ----------------------------------------------------------- */

    #[test]
    fn test_conv() {
        let mut test_interc = Interpreter::new();

        test_interc.stack.push(100.to_string());
        test_interc.c_celfah("");
        test_interc.c_fahcel("");
        test_interc.c_dechex("");
        test_interc.c_hexbin("");
        test_interc.c_binhex("");
        test_interc.c_hexdec("");
        test_interc.c_decbin("");
        test_interc.c_bindec("");
        test_interc.c_ftm("");
        test_interc.c_mft("");

        assert!(test_interc.pop_stack_float() == 100.);
    }

    #[test]
    fn test_avg() {
        let mut test_interc = Interpreter::new();

        test_interc.stack.push((-2).to_string());
        test_interc.stack.push(2.to_string());
        test_interc.c_avg("");

        assert!(test_interc.pop_stack_float() == 0.);

        test_interc.stack.push(1.to_string());
        test_interc.stack.push(2.to_string());
        test_interc.stack.push(3.to_string());
        test_interc.stack.push(4.to_string());
        test_interc.c_avg_all("");

        assert!(test_interc.pop_stack_float() == 2.5);
    }

}