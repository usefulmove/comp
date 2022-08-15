use std::env;
use std::fs;
use std::path::Path;

mod cmdin;
mod poc;
mod mona;

const RELEASE_STATE: &str = "l";

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

// -- command list -------------------------------------------------------------
const CMDS: &str = "drop dup swap cls roll rot + ++ +_ - -- x x_ / chs abs round \
int inv sqrt throot proot ^ exp % mod ! gcd pi e g deg_rad rad_deg sin asin cos \
acos tan atan log log2 log10 ln logn sa _a sb _b sc _c dec_hex hex_dec dec_bin \
bin_dec hex_bin bin_hex rgb_hex hex_rgb C_F F_C a_b min min_ max max_ avg avg_ rand";

fn main() {
    // enable or disable backtrace on error
    env::set_var("RUST_BACKTRACE", "0");

    // construct command interpreter
    let mut cinter = cmdin::Interpreter::new();

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
        "mona" => {
            println!("{}", mona::MONA);
            return;
        }
        "-f" | "--file" => {
            // read operations list input from file
            if args.get(2).is_none() {
                eprintln!("  {}: no file path provided", poc::color_red_bold("error"),);
                std::process::exit(exit_code::NO_INPUT);
            }

            // read file contents
            let filename: String = args[2].to_string();
            let path: &Path = Path::new(&filename);

            let file_contents = fs::read_to_string(&path);
            if let Err(ref error) = file_contents {
                eprintln!(
                    "  {}: could not read [{}]: {error}",
                    poc::color_red_bold("error"),
                    poc::color_blue_coffee_bold(&path.display().to_string()),
                );
                std::process::exit(exit_code::OS_FILE_ERROR);
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

    output_stack(
        &mut cinter.stack.clone(),
        cinter.config.show_stack_level,
        cinter.config.monochrome,
    );

    std::process::exit(exit_code::SUCCESS);
} // main

fn show_help() {
    println!();
    println!("{}", poc::color_white_bold("COMP"));
    println!(
        "    {} {} {} {}",
        poc::color_grey_mouse("comp"),
        poc::color_grey_mouse(".."),
        poc::color_white_bold("command interpreter"),
        poc::color_grey_mouse(env!("CARGO_PKG_VERSION")),
    );
    println!();
    println!("{}", poc::color_white_bold("USAGE"));
    println!(
        "    {} {} {}",
        poc::color_grey_mouse("comp"),
        poc::color_white_bold("[OPTIONS]"),
        poc::color_blue_coffee_bold("<list>"),
    );
    println!(
        "    {} {} {}",
        poc::color_grey_mouse("comp"),
        poc::color_yellow_canary_bold("-f"),
        poc::color_blue_coffee_bold("<path>"),
    );
    println!();
    println!("{}", poc::color_white_bold("OPTIONS"));
    println!(
        "        {}      show version",
        poc::color_yellow_canary_bold("--version"),
    );
    println!(
        "    {}{} {}         read from file at the specified path",
        poc::color_yellow_canary_bold("-f"),
        poc::color_white_bold(","),
        poc::color_yellow_canary_bold("--file"),
    );
    println!(
        "        {}         show help information",
        poc::color_yellow_canary_bold("--help"),
    );
    println!();
    println!("{}", poc::color_white_bold("DESCRIPTION"));
    println!(
        "The comp interpreter takes a {} sequence of (postfix) operations as \
    command line arguments or a {} argument that specifies the path to a file \
    containing a list of operations. Each operation is either a command ({}) \
    or a {}. The available commands are listed below.",
        poc::color_blue_coffee_bold("<list>"),
        poc::color_blue_coffee_bold("<path>"),
        poc::color_green_eggs_bold("symbol"),
        poc::color_blue_smurf_bold("value"),
    );
    println!();
    println!(
        "    Usage Guide:   {}",
        poc::color_grey_mouse("https://github.com/usefulmove/comp/blob/main/USAGE.md"),
    );
    println!(
        "    Repository:    {}",
        poc::color_grey_mouse("https://github.com/usefulmove/comp#readme"),
    );
    println!();
    println!("{}", poc::color_white_bold("EXAMPLES"));
    println!(
        "    {} {} {}                  {}",
        poc::color_grey_mouse("comp"),
        poc::color_blue_smurf_bold("1 2"),
        poc::color_green_eggs_bold("+"),
        poc::color_white_bold("add 1 and 2"),
    );
    println!(
        "    {} {} {}                  {}",
        poc::color_grey_mouse("comp"),
        poc::color_blue_smurf_bold("5 2"),
        poc::color_green_eggs_bold("/"),
        poc::color_white_bold("divide 5 by 2"),
    );
    println!(
        "    {} {} {} {} {}      {}",
        poc::color_grey_mouse("comp"),
        poc::color_blue_smurf_bold("3"),
        poc::color_green_eggs_bold("dup x"),
        poc::color_blue_smurf_bold("4"),
        poc::color_green_eggs_bold("dup x +"),
        poc::color_white_bold("sum of the squares of 3 and 4"),
    );
    println!();
    println!("{}", poc::color_white_bold("COMMANDS"));
    println!("{}", poc::color_grey_mouse(CMDS));
    println!();
}

fn show_version() {
    let version: &str = env!("CARGO_PKG_VERSION");
    println!(
        "  {} {}{}",
        poc::color_grey_mouse("comp"),
        poc::color_blue_smurf_bold(version),
        poc::color_white_bold(RELEASE_STATE),
    );
}

fn output_stack(stack: &mut Vec<String>, annotate: bool, monochrome: bool) {
    let mut f_color_annotate: fn(&str) -> colored::ColoredString = poc::color_blank;
    let mut f_color_stack_high: fn(&str) -> colored::ColoredString = poc::color_blue_coffee_bold;
    let mut f_color_stack: fn(&str) -> colored::ColoredString = poc::color_blue_smurf;
    if annotate {
        f_color_annotate = poc::color_charcoal_cream;
    }
    if monochrome {
        f_color_stack_high = poc::color_white;
        f_color_stack = poc::color_white;
    }

    while !stack.is_empty() {
        let level: u32 = stack.len() as u32;
        match level {
            1 => {
                println!(
                    // top element
                    "{}  {}",
                    f_color_annotate(level_map(level)),
                    f_color_stack_high(&stack.remove(0)),
                )
            }
            _ => {
                println!(
                    // all other elements
                    "{}  {}",
                    f_color_annotate(level_map(level)),
                    f_color_stack(&stack.remove(0)),
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
