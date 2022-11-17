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

                cmds.iter()
                    .for_each(|cmd| print!("{} ", theme.blue_smurf(cmd)));
                println!();

                return;
            }
            "--file" | "-f" => {
                // read operations list input from file
                if args.get(2).is_none() {
                    eprintln!(
                        "  {}: no file path provided",
                        theme.red_bold("error"),
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
                            theme.red_bold("error"),
                            theme.blue_coffee_bold(&path.display().to_string()),
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
                if args.get(3).is_some() {interpreter.ops.extend((args[3..]).to_vec())}
            }
            "--help" | "help" => {
                // display command usage information
                show_help();
                return;
            }
            "magic8" => {
                use std::collections::HashMap;
                use rand::Rng;

                let mut magic8:HashMap<u8, &str> = HashMap::new();
                magic8.insert(0, "it is certain");
                magic8.insert(1, "it is decidedly so");
                magic8.insert(2, "without a doubt");
                magic8.insert(3, "yes definitely");
                magic8.insert(4, "you may rely on it");
                magic8.insert(5, "as I see it, yes");
                magic8.insert(6, "most likely");
                magic8.insert(7, "outlook good");
                magic8.insert(8, "yes");
                magic8.insert(9, "signs point to yes");
                magic8.insert(10, "reply hazy, try again");
                magic8.insert(11, "ask again later");
                magic8.insert(12, "better not tell you now");
                magic8.insert(13, "cannot predict now");
                magic8.insert(14, "concentrate and ask again");
                magic8.insert(15, "don't count on it");
                magic8.insert(16, "my reply is no");
                magic8.insert(17, "my sources say no");
                magic8.insert(18, "outlook not so good");
                magic8.insert(19, "very doubtful");

                let mut rng = rand::thread_rng();
                let id: u8 = rng.gen_range(0..20);
                println!(
                    "  {}{}{}",
                    theme.grey_mouse("\""),
                    theme.blue_smurf_bold(magic8.get(&id).unwrap()),
                    theme.grey_mouse("\""),
                );
                return;
            }
            "mona" => {
                let anom = mona::MONA.chars().rev().collect::<String>();
                println!("{}\n", theme.white_bold(&anom));
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
    interpreter.load_config();

    // load stack
    if interpreter.config.stack_persistence {interpreter.load_stack()}

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
    if interpreter.config.stack_persistence {interpreter.save_stack()}

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
        theme.cream_bold("COMP")
    );
    println!(
        "    {} {} {} {}",
        theme.grey_mouse("comp"),
        theme.charcoal_cream(".."),
        theme.cream_bold("command interpreter"),
        theme.grey_mouse(env!("CARGO_PKG_VERSION")),
    );
    println!();
    println!(
        "{}",
        theme.cream_bold("USAGE")
    );
    println!(
        "    {} {} {}",
        theme.grey_mouse("comp"),
        theme.cream_bold("[OPTIONS]"),
        theme.blue_coffee_bold("<list>"),
    );
    println!(
        "    {} {} {}",
        theme.grey_mouse("comp"),
        theme.yellow_canary_bold("-f"),
        theme.blue_coffee_bold("<path>"),
    );
    println!();
    println!(
        "{}",
        theme.cream_bold("OPTIONS")
    );
    println!(
        "        {}      show version",
        theme.yellow_canary_bold("--version"),
    );
    println!(
        "    {}{} {}         read from file at the specified path",
        theme.yellow_canary_bold("-f"),
        theme.grey_mouse(","),
        theme.yellow_canary_bold("--file"),
    );
    println!(
        "    {}{} {}     display available commands",
        theme.yellow_canary_bold("--"),
        theme.grey_mouse(","),
        theme.yellow_canary_bold("--commands"),
    );
    println!(
        "        {}         show help information",
        theme.yellow_canary_bold("--help"),
    );
    println!();
    println!(
        "{}",
        theme.cream_bold("DESCRIPTION")
    );
    println!(
        "The comp interpreter takes a {} sequence of (postfix) operations as \
    command line arguments or a {} argument that specifies the path to a file \
    containing a list of operations. Each operation is either a command ({}) \
    or a {}. The available commands are listed below.",
        theme.blue_coffee_bold("<list>"),
        theme.blue_coffee_bold("<path>"),
        theme.green_eggs_bold("symbol"),
        theme.blue_smurf_bold("value"),
    );
    println!();
    println!(
        "    Usage Guide:   {}",
        theme.grey_mouse("https://github.com/usefulmove/comp/blob/main/USAGE.md"),
    );
    println!(
        "    Repository:    {}",
        theme.grey_mouse("https://github.com/usefulmove/comp#readme"),
    );
    println!();
    println!(
        "{}",
        theme.cream_bold("EXAMPLES")
    );
    println!(
        "    {} {} {}                  {}",
        theme.grey_mouse("comp"),
        theme.blue_smurf_bold("1 2"),
        theme.green_eggs_bold("+"),
        theme.cream_bold("add 1 and 2"),
    );
    println!(
        "    {} {} {}                  {}",
        theme.grey_mouse("comp"),
        theme.blue_smurf_bold("5 2"),
        theme.green_eggs_bold("/"),
        theme.cream_bold("divide 5 by 2"),
    );
    println!(
        "    {} {} {} {} {}      {}",
        theme.grey_mouse("comp"),
        theme.blue_smurf_bold("3"),
        theme.green_eggs_bold("dup x"),
        theme.blue_smurf_bold("4"),
        theme.green_eggs_bold("dup x +"),
        theme.cream_bold("sum of the squares of 3 and 4"),
    );
    println!();
}

fn show_version() {
    // color theme
    let theme = cor::Theme::new();

    let version: &str = env!("CARGO_PKG_VERSION");
    println!(
        "  {} {}{}",
        theme.grey_mouse("comp"),
        theme.blue_smurf_bold(version),
        theme.white_bold(RELEASE_STATE),
    );
}

fn output_stack(stack: Vec<String>, annotate: bool, monochrome: bool) {
    // color theme
    let theme = cor::Theme::new();

    let mut color_stack_top_closure = BoxedClosure::new(
        |x| theme.blue_coffee_bold(x)
    );
    let mut color_stack_closure = BoxedClosure::new(
        |x| theme.blue_smurf(x)
    );
    let mut color_annotate_closure = BoxedClosure::new(
        |x| theme.color_blank(x)
    );
    if annotate {
        color_annotate_closure = BoxedClosure::new(
            |x| theme.charcoal_cream(x)
        );
    }
    if monochrome {
        color_stack_top_closure = BoxedClosure::new(|x| theme.white_bold(x));
        color_stack_closure = BoxedClosure::new(|x| theme.white(x));
    }

    let len = stack.len();
    stack.iter()
        .enumerate()
        .for_each(|(i, ent)| {
            let level = len - i;

            match level {
                1 => { println!("{}  {}", // top element
                        (color_annotate_closure.f)(annotate_level(level)),
                        (color_stack_top_closure.f)(ent),
                    )
                }
                _ => { println!("{}  {}", // all other elements
                        (color_annotate_closure.f)(annotate_level(level)),
                        (color_stack_closure.f)(ent),
                    )
                }
            }
        });

}

fn annotate_level(level: usize) -> &'static str {
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