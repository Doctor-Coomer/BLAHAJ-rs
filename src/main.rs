extern crate clap;
extern crate rand;
extern crate utf8_chars;
extern crate ctrlc;

use clap::{App, Arg};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::IsTerminal;
use utf8_chars::BufReadCharsExt;

mod cat;
mod flags;
mod shark;

fn main() {

    //Makes the cursor visible, if once it wasn't, once the program terminates unexpectedly.
    ctrlc::set_handler(move || {
        print!("\u{001B}[?25h");
	std::process::exit(0);
    }).expect("Error setting Ctrl-C handler.");

    let mut filename: String = String::new();
    let mut c = parse_cli_args(&mut filename);

    if filename == "" {
        let stdin = io::stdin(); // For lifetime reasons
        cat::print_chars_lol(
            BufReader::new(stdin.lock()).chars().map(|r| r.unwrap()),
            &mut c,
            true,
        );
    } else if lolcat_file(&filename, &mut c).is_err() {
        eprintln!("Error opening file {}.", filename)
    }
}

fn lolcat_file(filename: &str, c: &mut cat::Control) -> Result<(), io::Error> {
    let f = File::open(filename)?;
    let file = BufReader::new(&f);
    cat::print_lines_lol(file.lines().map(|r| r.unwrap()), c);
    Ok(())
}

fn parse_cli_args(filename: &mut String) -> cat::Control {
    let matches = lolcat_clap_app().get_matches();
    
    let flag_color: String;
    if matches.is_present("random") {
	flag_color = flags::ALL_NAMES[rand::random::<usize>()%flags::ALL_NAMES.len()].to_string(); 
    } else {
	flag_color = matches.value_of("flag color").unwrap_or("femboy").to_string();
    }
    
    let seed = 0;

    *filename = matches.value_of("filename").unwrap_or("").to_string();

    let print_color = matches.is_present("force-color") || std::io::stdout().is_terminal();

    let terminal_supports_truecolor = match std::env::var("COLORTERM") {
        Ok(val) => val == "truecolor" || val == "24bit",
        Err(_) => false,
    };

    let mut retval = cat::Control {
        seed,
	flag_name: flag_color,
        background_mode: matches.is_present("background"),
	individual_mode: matches.is_present("individual"),
	word_mode: matches.is_present("words"),
	print_color,
        terminal_supports_truecolor,
    };

    if matches.is_present("help") {
        print_rainbow_help(&mut retval);
        std::process::exit(0);
    }
    if matches.is_present("flag") {
	let size_mul: usize = match matches.value_of("multiplier") {
	    Some(a) => a.parse::<usize>().unwrap(),
	    None => 1
	}; 
	print_flag_graphic(size_mul, &mut retval);
	std::process::exit(0);
    }
    if matches.is_present("shark") {
	print_shark(&mut retval);
	std::process::exit(0);
    }
    if matches.is_present("flags") {
	print_all_flags_and_names(&mut retval);
	std::process::exit(0);	
    }
    
    retval
}

fn print_rainbow_help(c: &mut cat::Control) {
    let app = lolcat_clap_app();

    let mut help = Vec::new();
    app.write_help(&mut help).unwrap();
    let help = String::from_utf8(help).unwrap();

    println!("{}", help);
    //cat::print_lines_lol(help.lines(), c);
}

fn print_flag_graphic(mul: usize, c: &mut cat::Control) {
    let flag_color = flags::get_flag(&c.flag_name);

    let mut flag: String = String::new();
    let mut seed: usize = 0;
    
    for y in 1..flag_color.len()*mul+1 {
	for _x in 0..(flag_color.len()*4)*mul {
	    flag += "█";
	}	
	c.seed = seed;
	if y%mul == 0 {
	    seed += 1;
	} 
 	cat::print_lines_lol(flag.lines(), c);
	flag = String::new();
    }

}

fn print_all_flags_and_names(c: &mut cat::Control) {
    c.individual_mode = true;
    
    println!("Available flags/colors:\n");

    for i in 0..flags::ALL_NAMES_SORTED.len() {
	let mut flag_name: String = flags::ALL_NAMES_SORTED[i].to_string();
	flag_name.replace_range(0..1, &flag_name[0..1].to_uppercase());
	let flag_color = flags::get_flag(&flags::ALL_NAMES_SORTED[i]);
	c.flag_name = flags::ALL_NAMES_SORTED[i].to_string();
	c.seed = 0;
	
	let mut flag = String::new();
	for _x in 0..flag_color.len() {
	    flag += "█";
	}


	print!("{} ", flag_name);
	cat::print_lines_lol(flag.lines(), c);
    }
}

fn print_shark(c: &mut cat::Control) {
    cat::print_lines_lol(shark::SHARK.lines(), c);
}

fn lolcat_clap_app() -> App<'static, 'static> {
    App::new("BLÅHAJ")
        .version("v1.0.0")
	.arg(
            Arg::with_name("background")
                .short("b")
                .long("background")
                .help("Color the background")
                .takes_value(false),
        )
 	.arg (
	    Arg::with_name("flag color")
                .short("c")
                .long("colors")
                .help("Color scheme to use (Default: femboy)")
                .takes_value(true),
	)
 	.arg (
	    Arg::with_name("individual")
                .short("i")
                .long("individual")
                .help("Color individual characters")
                .takes_value(false),
	)
	.arg (
	    Arg::with_name("words")
                .short("w")
                .long("words")
                .help("Color individual words")
                .takes_value(false),
	)
	.arg(
            Arg::with_name("multiplier")
                .short("m")
                .long("multiplier")
                .help("Multiplier for the flag size (-f)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("shark")
                .short("s")
                .long("shark")
                .help("Shork :3")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("flag")
                .short("f")
                .long("flag")
                .help("Return a flag")
                .takes_value(false),
        )
	.arg(
            Arg::with_name("flags")
                .long("flags")
                .help("List all available flags")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("random")
                .short("r")
                .long("random")
                .help("Use a random color scheme")
                .takes_value(false),
        )	
        .arg(
            Arg::with_name("filename")
                .short("i")
                .long("input file name")
                .help("Blahaj this file. Reads from STDIN if missing")
                .takes_value(true)
                .index(1),
        )
        .arg(
            Arg::with_name("help")
                .short("h")
                .long("help")
                .help("Prints help information")
                .takes_value(false),
        )
}
