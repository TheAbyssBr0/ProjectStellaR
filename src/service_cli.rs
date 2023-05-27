//! This is the n_curses part for the service with autocomplete, spell check,
//! and database interactions
extern crate ncurses;

use ncurses::*;
use rusqlite::Connection;

use crate::service_db_actions;

/// Gets suggestions based on previously used services and what the user has typed in so far
/// or could type with already typed informatoin.
/// ### Params
/// `input`: The contents of the buffer that reads what the user has typed in so far
/// `conn` : Rusqlite connection
/// ### Returns
/// A vector of at most 3 Strings that are possible candidates for auto complete based on user input 
/// ### Side-effect
/// Calls other functions that reads from sqlite services database
fn get_suggestions(input: &str, conn: &Connection) -> Vec<String> {
    let all_services_vec = service_db_actions::read_all_rows(conn).unwrap();
    let services_vec_regex = service_db_actions::apply_regex(&all_services_vec, format!("{}.*", input).as_str());
    return services_vec_regex.unwrap();
}

/// Gets the previously recorded password number from the database for given service.
/// ### Params
/// `conn`: Rusqlite Connection
/// `service_title`: The record to look up for associated password number value
/// ### Returns
/// The password number associated with the service title or 1 (default) if service does not exist
/// ### Side-effect
/// Reads from database and also may write to database
fn read_pass_num(conn: &Connection, service_title: &str) -> u8{
    let query = format!("SELECT (pass_num) FROM services WHERE title = \"{}\"", service_title);
    match conn.query_row(query.as_str(), (), |row| row.get(0)) {
        Ok(pass_num) => pass_num,
        Err(_) => 1
    }
}

/// Creates a curses environment for taking user input on service. Provides auto completion feature.
/// ### Returns
/// String representation of the service title and password number concatenated together
/// ### Side effects
/// Initiates connection to database and calls other functions that read from and write to database
pub fn create_service_screen() -> String {
    // Initialize the screen
    initscr();
    // Turn off echoing of input characters
    noecho();
    // Enable keypad mode for arrow keys and tab key
    keypad(stdscr(), true);
    // Get the terminal size
    let mut max_y = 0;
    let mut max_x = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    // Check if the terminal size is at least 25 chars wide and 5 chars tall
    if max_y < 6 || max_x < 25 {
        // End the screen and print an error message
        endwin();
        println!("Terminal is not big enough! Exiting.");
        std::process::exit(0);
    }
    // Connect to database
    let conn = service_db_actions::get_connection("service_records.db");

    // Create a window for user input and suggestions
    let win = newwin(5, 25, (max_y - 5) / 2, (max_x - 25) / 2);
    // Enable keypad mode for arrow keys and tab key
    keypad(win, true);
    // Refresh the screen and the window
    refresh();
    wrefresh(win);
    // Create a string buffer for user input
    let mut buffer = String::new();
    // Create a vector for suggestions
    let mut suggestions: Vec<String> = Vec::new();
    // Create a variable for the current selected suggestion index
    let mut selected = 0;
    // Print the prompt in the first line of the window, left-aligned
    let prompt = "> ";
    mvwprintw(win, 1, 0, prompt);
    // Move the cursor to the end of the prompt
    wmove(win, 1, prompt.len() as i32);
    // Loop until the user presses enter or escape
    loop {
        // Get a character from the user
        let ch = wgetch(win);
        match ch {
            // If the character is enter, break the loop
            KEY_ENTER | 10 | 13 => if buffer.len() > 0 {break},
            // If the character is escape, clear the buffer and break the loop
            27 => {
                buffer.clear();
                break;
            }
            // If the character is backspace, delete the last character from the buffer
            KEY_BACKSPACE | 127 => {
                buffer.pop();
            }
            // If the character is up arrow, decrement the selected index and wrap around if needed
            KEY_UP => {
                if selected > 0 {
                    selected -= 1;
                } else {
                    selected = suggestions.len() - 1;
                }
            }
            // If the character is down arrow, increment the selected index and wrap around if needed
            KEY_DOWN => {
                if selected < suggestions.len() - 1 {
                    selected += 1;
                } else {
                    selected = 0;
                }
            }
            // If the character is tab, replace the buffer with the selected suggestion
            9 => {
                if !suggestions.is_empty() {
                    buffer = suggestions[selected].clone();
                }
            }
            // If the character is printable, append it to the buffer
            _ => {
                if ch >= 32 && ch <= 126 {
                    buffer.push(ch as u8 as char);
                }
            }
        }
        // Clear the window content except the prompt
        werase(win);
        mvwprintw(win, 1, 0, prompt);
        // Print the buffer in the first line of the window, after the prompt
        mvwprintw(win, 1, prompt.len() as i32, &buffer);
        // Get the suggestions based on the buffer content
        suggestions = get_suggestions(&buffer, &conn);
        // Print the suggestions in the next lines of the window, left-aligned and highlighted if selected
        for (i, suggestion) in suggestions.iter().enumerate() {
            if i == selected {
                wattron(win, A_REVERSE());
            }
            mvwprintw(win, (i + 2) as i32, 0, &suggestion);
            if i == selected {
                wattroff(win, A_REVERSE());
            }
        }
        // Move cursor back in place
        wmove(win, 1, (prompt.len() + buffer.len()) as i32);
        // Refresh the window
        wrefresh(win);
    }
    // End the screen
    endwin();

    if buffer == "".to_string() {
        return "".to_string()
    }

    // Check if buffer contains password number in the end
    let processed: Vec<String> = buffer.split(' ').map(|s| s.to_string()).collect();
    let mut has_pass_num = false;
    let mut pass_num = match processed.last().unwrap().parse::<u8>() {
        Ok(p_num) => {
            has_pass_num = true;
            p_num
        },
        Err(_) => 1,
    };

    let offset = if has_pass_num {1} else {0};
    let service_title = (&processed[0..processed.len() - offset]).join(" ").to_ascii_lowercase();

    if !has_pass_num {
        pass_num = read_pass_num(&conn, &service_title.as_str());
    }

    service_db_actions::update_db(&conn, service_title.as_str(), pass_num);

    format!("{}{}", service_title, pass_num)
}
