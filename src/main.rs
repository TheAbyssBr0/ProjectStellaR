use auth::auth_user;
use user_inputs::{get_visible_input_prompt, get_hidden_input_prompt};
use zeroize::Zeroize;

mod characters;
mod generator;
mod user_inputs;
mod service_cli;
mod service_db_actions;
mod auth;

fn main() {
    println!("Launched stellar password manager.");

    // Authorize
    let username = get_visible_input_prompt("Username: ");
    let mut password = get_hidden_input_prompt("Password: ");

    if !auth_user(username.as_str(), password.as_mut_str()) {
        println!("Authorization failed! Exiting application.");
        std::process::exit(0);
    }

    // zeroize password
    password.zeroize();

    println!("Logged in as: {}\nType in 'help' for available commands.", username);
    // start clipboard (clipboard values disappear when clipboard is dropped so DON'T DROP IT TOO SOON)
    let mut clipboard = arboard::Clipboard::new().unwrap();

    // initiate main user input loop
    user_inputs::start_user_input_loop(&username.as_str(), &mut clipboard);
}
