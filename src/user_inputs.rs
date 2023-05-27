use std::io::Write;
use zeroize::Zeroize;
use crate::{service_cli, generator, characters::Key, auth::auth_user};
use arboard::Clipboard;

/// Takes in input from stdin in terminal with the given prompt
/// ### Params
/// - `prompt`: The prompt to give the user for the input
/// ### Returns
/// String (plus new line character) of the user's input
/// ### Side-effect
/// Reads from stdin, prints prompt to stdout
pub fn get_visible_input_prompt(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let stdin = std::io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();
    std::io::BufRead::read_line(&mut reader, &mut input).unwrap_or_else(|err| {
         println!("Encountered error while reading from stdin: {:}", err);
         std::process::exit(1);
    });

    input
}

/// Takes in input from stdin in terminal with given prompt but hides the input
/// as it is being typed. For taking secret inputs only.
/// ### Params
/// - `prompt`: The prompt to give the user for the input
/// ### Returns
/// String of the user's input (no new line characer)
/// ### Side-effect
/// Reads from stdin, prints prompt to stdout
pub fn get_hidden_input_prompt(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let hidden_input = rpassword::read_password().unwrap();
    return hidden_input
}

/// Gets service input from the user
/// ### Returns:
/// Option<String> which is None if the user input is empty string
/// ### Side-effect
/// Prints curses interface, takes user input and reads/writes database entries
fn get_service() -> Option<String> {
    get_visible_input_prompt("Press Enter and then input service or optionally service {space} password number.");
    let service = service_cli::create_service_screen();
    
    match service.as_str() {
        "" => None,
        _ => Some(service),
    }
}

/// Gets password property i.e. password legal character-types from the user input
/// ### Returns:
/// A Key object which will return to default Key if the user input fails
/// ### Side-effect:
/// Reads from stdin
fn get_key() -> Key {
    print!("Input key in format TFTF where T is true and F is false. ");
    println!("And the order is uppercase, lowercase, numbers, and symbols respectively");
    let key_str = get_visible_input_prompt("> ");
    if key_str.len() != 5 {
        println!("Operation failed, number of input characters incorrect. Switching key to default.");
        return Key::default();
    }

    // check for all false
    if key_str.trim().to_lowercase() == "ffff" {
        println!("Operation failed, all cannot be false. Switching key to default.");
        return Key::default();
    }

    let mut bools: [bool; 4] = [true, true, true, true];

    for (index, c) in key_str.trim().chars().enumerate() {
        match c {
            't' | 'T' => bools[index] = true,
            'f' | 'F' => bools[index] = false,
            _ => {
                println!("Encountered non-t or non-f character. Switching key to default.");
                return Key::default();
            }
        }
    }

    Key::from_arr(bools)
}

/// Gets length of the password to generate
/// ### Returns:
/// A unsigned byte of user input length. If user input fails, default length 16 is returned.
/// ### Side-effect
/// Reads from stdit
fn get_len() -> u8 {
    let len_str = get_visible_input_prompt("> ");
    let len: u8 = len_str.trim().parse().unwrap_or_else(|_| {
        println!("Failed to convert to number. Setting to default password length.");
        return 16;
    });

    if len < 4 {
        println!("Number too small. Password length must be >= 4. Setting default password length.");
        return 16;
    }

    return len;
}

/// Generates a password from given parameters and copies it to clipboard
/// ### Params
/// - `username`: name of logged in user
/// - `service` : valid or invalid service as set by the user
/// - `key`     : key containing desired properties of generated password
/// - `len`     : length of the password to generate
/// - `clipboard`: clipboard object to copy the generated password into
/// ### Side-effect
/// Passes value to the system clipboard
/// ### Panics
/// No, but returns early if service is not set
fn gen(username: &str, service: &Option<String>, key: Key, len: u8, clipboard: &mut Clipboard) {
    if service.is_none() {
        println!("Service is unset. Please set service first!");
        return;
    }

    let mut salt: String = String::new();
    salt.push_str(&username);
    salt.push_str(&service.as_ref().unwrap());
    let mut password = get_hidden_input_prompt("Password: ");
    if !auth_user(username, &mut password) {
        println!("Password did not match login password. Try again.");
        return;
    }
    let mut generated_pass = generator::generate_pass(&mut password, salt.as_bytes(), len, key);
    
    clipboard.set_text(generated_pass.to_string()).unwrap();
    generated_pass.zeroize();
    println!("Generated password and copied to clipboard!");
}

/// Prints out the help string 
fn help() {
    println!("Available commands:");
    println!("'serv': Use this to set service for which the password is being generated, ex: Netflix");
    print!("'key' : Use this to set combination of character types you want in the password,");
    println!(" i.e. uppercase, lowercase, nums, symbols. Default: All characters legal");
    println!("'len' : Use this to set length of password. Default: 16");
    println!("'gen' : Use this to generate the password and copy to clipboard. You will be asked to authenticate!");
    println!("'help': This command.");
    println!("'print': Prints out the set values for all arguments");
    println!("'exit': Exits program.")
}

/// Prints all the user selections made that will influence the generated password
/// ### Parameters
/// - `username` : name of logged in user
/// - `service`  : service as set by the user (can be None)
/// - `key`      : Key containing set (or default) properties of generated password by user
fn print(username: &str, service: &Option<String>, key: Key, len: u8) {
    print!("Logged in as: {}", username);
    let service_string = match service {
        Some(s) => s,
        None => "service not set" 
    };
    println!("Service set as: {}", service_string);
    println!("Generated password will contain: {}", key.to_str());
    println!("Generated password length set to: {}", len);
}

/// Starts the CLI for taking in password properties as set by the user and generating passwords
/// ### Params
/// - `username` : name of the logged in user
/// - `clipboard`: clipboard object where the generated password will be delivered
/// ### Side-effect
/// Takes in user input and the various functions it calls may carry out database operations
/// #### Panics
/// No. But it will exit if the user asks for an exit
pub fn start_user_input_loop(username: &str, clipboard: &mut Clipboard) {
    let mut service = None;
    let mut key = Key::default();
    let mut len = 16;

    loop {
        let command = get_visible_input_prompt("> ");

        match command.as_str() {
            "serv\n"  => service = get_service(),
            "key\n"   => key = get_key(),
            "len\n"   => len = get_len(),
            "gen\n"   => gen(username, &service, key, len, clipboard),
            "help\n"  => help(),
            "print\n" => print(username, &service, key, len),
            "exit\n"  => std::process::exit(0),
            _ => println!("Unknown command. Type 'help' to get list of valid commands.")
        }
    }

}