use std::fs::File;
use crypter::CryptError;
use clap::{App, Arg, SubCommand};
use std::io;
use std::process::exit;
use std::str::FromStr;

pub fn cli() -> Result<(File, String, u64, bool), CryptError> {
    let matches = App::new("SimpleCrypt")
        .usage("crypter [FLAGS] <INPUT> <key>")
        .version("0.2.0")
        .author("Moritz von der Heiden, Samuel Burch")
        .about("Simple tool to en- and decrypt files.")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("key")
            .help("Sets the key to do the crypto with")
            .required(true)
            .index(2))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("test")
                    .about("controls testing features")
                    .version("1.3")
                    .author("Someone E. <someone_else@other.com>")
                    .arg(Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely")))
        .get_matches();

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    let verbosity = matches.occurrences_of("v");
    match verbosity {
        0 => (),
        1 | _ => println!("Some verbose info"),
    }

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let mut filename = matches.value_of("INPUT").unwrap().to_string();
    let file = File::open(&filename)?;
    if verbosity >= 1 {
        println!("Using input file: {}", filename);
    }

    let mut key = match u64::from_str(matches.value_of("key").unwrap()) {
        Ok(k) => k,
        Err(e) => return Err(CryptError::from(e)),
    };
    let two :u64 = 2;
    let min_key = two.pow(63);
    if key < min_key {
        println!("Your key should be bigger than 2^63 (20 digits) but it is only {} :(", key);
        match get_better_key() {
            Some(k) => key = k,
            None => (),
        }
    }

    let mut encrypted = false;
    if filename.ends_with(".encrypt") {
        encrypted = true;
    }

    if encrypted {
        filename = filename.trim_right_matches(".encrypt").to_string();
    } else {
        filename.push_str(".encrypt");
    }
    match check_file_existance(&filename) {
        Some(s) => filename = s,
        None => (),
    }
    
    Ok((file, filename, key, encrypted))
}

fn get_better_key() -> Option<u64> {
    println!("Do you want to continue? (y/n)");
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => {
                println!("ERROR: {}", e);
                exit(1);
            }
        }
        let conv_char = input.trim_right().to_ascii_lowercase();
        if conv_char == "n" {
            loop {
                println!("Please enter your new key:");
                input.clear();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        match u64::from_str(input.trim()) {
                            Ok(k) => return Some(k),
                            Err(e) => println!("ERROR: {}", e),
                        }
                    }
                    Err(e) => {
                        println!("ERROR: {}", e);
                    }
                }
            }
        }
        else if conv_char == "y" {
            break;
        }
        else {
            println!("Invalid answer...");
        }
    }
    None
}

fn check_file_existance(name: &String) -> Option<String> {
    match File::open(&name) {
        Ok(_) => {
            loop {
                let mut input = String::new();
                println!("The file {} already exists. Do you want to overwrite it? (y/n)", name);
                match io::stdin().read_line(&mut input) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("ERROR: {}", e);
                        exit(1);
                    }
                }
                let conv_char = input.trim_right().to_ascii_lowercase();
                if conv_char == "y" {
                    break;
                }
                if conv_char == "n" {
                    println!("Please choose a new name for the file.");
                    println!("Note that this time there won't be a check if the file already exists.");
                    input.clear();
                    match io::stdin().read_line(&mut input) {
                        Ok(_) => {
                            return Some(input.trim_right().to_string());
                        }
                        Err(e) => {
                            println!("ERROR: {}", e);
                            exit(1);
                        }
                    }
                }
            }
        }
        Err(_) => (),
    }
    None
}
