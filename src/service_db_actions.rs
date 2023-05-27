extern crate rusqlite;

use rusqlite::{Connection, Result};
use regex::Regex;

/// Creates the database table for service titles if they do not exist
/// ### Parameters
/// - `conn` : Connection to a sqlite database (rusqlite object)
/// ### Side-effects
/// Creates tables if they do not exist. Quietly fails to do so if they already
/// exists
fn create_tables(conn: &Connection) {
    let query = "CREATE TABLE services(
        title TEXT NOT NULL UNIQUE,
        pass_num INTEGER
    )";

    match conn.execute(query, ()) {
        Ok(_) => (),
        Err(_) => ()
    };
}

/// Creates a connection to a sqlite database file. If file does not exist,
/// creates a file in CWD and then opens it with a connection. This function is
/// used by the service-associated database interactions only
/// ### Params
/// - `filename`: the name of the database file in the current working director
/// ### Returns
/// Rusqlite Connection to the database 
/// ### Side-effect
/// Makes connection with sqlite database
pub fn get_connection(filename: &str) -> Connection {
    let conn = Connection::open(filename).unwrap();
    create_tables(&conn);
    conn
}

/// Inserts new services into the database for future tab-to-complete.
/// Also updates the password number if a new password number is used.
/// ### Params
/// - `conn`: Rusqlite connection to database
/// - `service_title`: string representation of the service title
/// - `pass_num`: password number to update to
/// ### Side-effect
/// Writes to database
pub fn update_db(conn: &Connection, service_title: &str, pass_num: u8) {
    let query_service = format!("INSERT OR REPLACE INTO services (title, pass_num) VALUES (\"{}\", {})", service_title, pass_num);
    match conn.execute(query_service.as_str(), ()) {
        Ok(_) => (),
        Err(_) => ()
    };
}

/// Reads all services from table into a vector
/// ### Params
/// - `conn`: Rusqlite connection
/// ### Returns
/// `Result<Vec<String>>` so that the errors are passed onto the function that calls this one
/// ### Side-effect
/// Reads from database
pub fn read_all_rows(conn: &Connection) -> Result<Vec<String>> {
    // Prepare a query to select all service titles from the services table
    let query = "SELECT title FROM services";
    // Execute the query and collect the results into a vector of strings
    let titles: Vec<String> = conn
        .prepare(query)?
        .query_map([], |row| row.get(0))?
        .collect::<Result<_>>()?;
    // Return the vector of titles
    Ok(titles)
}

/// Applies a regular expression to the given array slice containing Strings
/// and returns a vector of length at most three strings from the array slice that
/// satisfy the regular expression.
/// ### Params
/// - `titles`: the array slice containing the titles of the services
/// - `pattern`: the regular expression pattern to look for in the given array slice
/// ### Returns
/// `Result<Vec<String>> of at most 3 Strings that satisfy the given regex
/// ### Side-effect:
/// Reads from database
pub fn apply_regex(titles: &[String], pattern: &str) -> Result<Vec<String>> {
    // Create a regex object from the pattern
    let re = Regex::new(pattern).unwrap();
    // Create an empty vector to store the matching titles
    let mut matches = Vec::new();
    // Iterate over the titles and check if they match the pattern
    for title in titles {
        if re.is_match(&title) {
            // If the title matches, push it to the vector of matches
            matches.push(title.clone());
            // If the vector of matches has reached 3, break the loop
            if matches.len() == 3 {
                break;
            }
        }
    }
    // Return the vector of matches
    Ok(matches)
}
