use colored::ColoredString;
use std::{env, fs};
use std::path::Path;
use std::process::exit;

mod comp;
mod mona;

const RELEASE_STATE: &str = "a";

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
    let theme = cor::Theme::new();

    // construct command interpreter
    let mut interpreter = comp::Interpreter::new();

    // get command arguments
    let args: Vec<String> = env::args().collect();


    if args.len() > 1 {
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
                    interpreter.ops.extend((args[3..]).to_vec());
                }
            }
            "--help" | "help" => {
                // display command usage information
                show_help();
                return;
            }
            "mona" => {
                print!("{}", theme.color_rgb(mona::MONA, &cor::Color::new(243, 196, 129, true)));
                println!("{}", theme.color_rgb("allen mullen", &cor::Color::new(60, 49, 32, false)));
                return;
            }
            "--version" | "version" => {
                // display version information
                show_version();
                return;
            }
            _ => {
                // read operations list input from command line arguments
                interpreter.ops = (args[1..]).to_vec();
            }

        };
    }

    // load configuration
    interpreter.load_config("comp.toml");

    // load stack
    if interpreter.config.stack_persistence {
        interpreter.load_stack();
    }

    // process operations list ( ops list was loaded into the interpreter
    // in the match statement above based on command line arguments )
    interpreter.process_ops();

    /* display stack to user */
    output_stack(
        interpreter.get_stack(),
        interpreter.config.show_stack_level,
        interpreter.config.monochrome,
    );

    // save stack
    if interpreter.config.stack_persistence {
        interpreter.save_stack();
    }

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
    let theme = cor::Theme::new();

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
        theme.color_rgb("-f", &theme.yellow_canary_bold),
        theme.color_rgb("<path>", &theme.blue_coffee_bold),
    );
    println!();
    println!(
        "{}",
        theme.color_rgb("OPTIONS", &theme.cream_bold)
    );
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
        "    {}{} {}     display available commands",
        theme.color_rgb("--", &theme.yellow_canary_bold),
        theme.color_rgb(",", &theme.grey_mouse),
        theme.color_rgb("--commands", &theme.yellow_canary_bold),
    );
    println!(
        "        {}         show help information",
        theme.color_rgb("--help", &theme.yellow_canary_bold),
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
    let theme = cor::Theme::new();

    let version: &str = env!("CARGO_PKG_VERSION");
    println!(
        "  {} {}{}",
        theme.color_rgb("comp", &theme.grey_mouse),
        theme.color_rgb(version, &theme.blue_smurf_bold),
        theme.color_rgb(RELEASE_STATE, &theme.white_bold),
    );
}

fn output_stack(stack: Vec<String>, annotate: bool, monochrome: bool) {
    // color theme
    let theme = cor::Theme::new();

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
                    (color_stack_top_closure.f)(ent),
                )
            }
            _ => {
                println!( // all other elements
                    "{}  {}",
                    (color_annotate_closure.f)(level_map(level)),
                    (color_stack_closure.f)(ent),
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