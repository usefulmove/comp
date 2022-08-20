use std::env;
use std::fs;
use std::path::Path;

use colored::ColoredString;

mod cmdin;
mod poc;
mod mona;

const RELEASE_STATE: &str = "p";

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
    let theme: poc::Theme = poc::Theme::new();

    // construct command interpreter
    let mut interpreter = cmdin::Interpreter::new();

    // get command arguments
    let mut args: Vec<String> = env::args().collect();

    // if no arguments are passed, behave as if help flag was passed
    if args.len() <= 1 {
        args.push("help".to_string());
    }

    match args[1].as_str() {
        "--commands" | "--" => {
            // display available commands
            let mut cmds: Vec<&str> = interpreter.get_cmds();
            cmds.sort_unstable();

            for cmd in cmds {
                print!("{} ", theme.color_rgb(cmd, &theme.blue_smurf_bold));
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
                std::process::exit(exit_code::NO_INPUT);
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
                std::process::exit(exit_code::OS_FILE_ERROR);
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

    std::process::exit(exit_code::SUCCESS);
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
    let theme: poc::Theme = poc::Theme::new();

    println!();
    println!("{}", theme.color_rgb("COMP", &theme.cream_bold));
    println!(
        "    {} {} {} {}",
        theme.color_rgb("comp", &theme.grey_mouse),
        theme.color_rgb("..", &theme.grey_mouse),
        theme.color_rgb("command interpreter", &theme.cream_bold),
        theme.color_rgb(env!("CARGO_PKG_VERSION"), &theme.grey_mouse),
    );
    println!();
    println!("{}", theme.color_rgb("USAGE", &theme.cream_bold));
    println!(
        "    {} {} {}",
        theme.color_rgb("comp", &theme.grey_mouse),
        theme.color_rgb("[OPTIONS]", &theme.cream_bold),
        theme.color_rgb("<list>", &theme.blue_coffee_bold),
    );
    println!(
        "    {} {} {}",
        theme.color_rgb("comp", &theme.grey_mouse),
        theme.color_rgb("-f", &theme.yellow_canary_bold),
        theme.color_rgb("<path>", &theme.blue_coffee_bold),
    );
    println!();
    println!("{}", theme.color_rgb("OPTIONS", &theme.cream_bold));
    println!(
        "        {}      show version",
        theme.color_rgb("--version", &theme.yellow_canary_bold),
    );
    println!(
        "    {}{} {}         read from file at the specified path",
        theme.color_rgb("-f", &theme.yellow_canary_bold),
        theme.color_rgb(",", &theme.grey_mouse),
        theme.color_rgb("--file", &theme.yellow_canary_bold),
    );
    println!(
        "        {}     display available commands",
        theme.color_rgb("--commands", &theme.yellow_canary_bold),
    );
    println!(
        "        {}         show help information",
        theme.color_rgb("--help", &theme.yellow_canary_bold),
    );
    println!();
    println!("{}", theme.color_rgb("DESCRIPTION", &theme.cream_bold));
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
    println!("{}", theme.color_rgb("EXAMPLES", &theme.cream_bold));
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
    let theme: poc::Theme = poc::Theme::new();

    let version: &str = env!("CARGO_PKG_VERSION");
    println!(
        "  {} {}{}",
        theme.color_rgb("comp", &theme.grey_mouse),
        theme.color_rgb(version, &theme.blue_smurf_bold),
        theme.color_rgb(RELEASE_STATE, &theme.cream_bold),
    );
}

fn output_stack(stack: &mut Vec<String>, annotate: bool, monochrome: bool) {
    // color theme
    let theme: poc::Theme = poc::Theme::new();

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
#[path = "../test/test.rs"]
mod comp_tests;