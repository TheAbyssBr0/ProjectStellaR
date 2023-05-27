//! Handles authentication. Has functions for making sure the 
//! password and usernames match (if existing user) and helps
//! make sure the user puts in the correct password

use argon2::Config;
use rand::distributions::{Alphanumeric, DistString};
use rusqlite::Connection;
use zeroize::Zeroize;
use std::str;

use crate::user_inputs;

/// Creates the database table for user authentication if they do not exist
/// ### Parameters
/// - `conn` : Connection to a sqlite database (rusqlite object)
/// ### Side-effects
/// Creates tables if they do not exist. Quietly fails to do so if they already
/// exists
fn create_auth_tables(conn: &Connection) {
    let query = "CREATE TABLE auth(
        username TEXT NOT NULL UNIQUE,
        password_hash TEXT,
        password_salt TEXT
    )";

    match conn.execute(query, ()) {
        Ok(_) => (),
        Err(_) => ()
    };
}

/// Authenticates user based on the input username and password
/// ### Parameters
/// - `username` : username of person to authenticate
/// - `password` : passsword of the person to authenticate
/// ### Returns
/// a boolean; true if the username and password match and false otherwise
/// ### Side-effects
/// Reads in password confirmation for new users from stdin.
/// 
/// Will zeroize the password if the password and confirm password do not match
/// 
/// Reads from and writes to authentication sqlite database.
pub fn auth_user(username: &str, password: &mut str) -> bool {
    let conn = Connection::open("auth.db").unwrap();
    create_auth_tables(&conn);
    let mut user_new = false;

    // Try to put in username into unique table. If it fails, username exists
    let query_add_user = format!("INSERT INTO auth (username) VALUES (\"{}\")", username);
    match conn.execute(query_add_user.as_str(), ()) {
        Ok(_) => {
            user_new = true;
        },
        Err(_) => ()
    }

    if user_new {
        let mut confirm_pass = user_inputs::get_hidden_input_prompt("Confirm password: ");
        if confirm_pass != password.to_string() {
            confirm_pass.zeroize();
            password.zeroize();

            // clear username from auth table
            let query = format!("DELETE FROM auth WHERE username = \"{}\"", username);
            conn.execute(query.as_str(), ()).unwrap();
            
            return false;
        }
        let salt = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let argon_config = Config::default();
        let hash = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &argon_config).unwrap();
        let query = format!("INSERT OR REPLACE INTO auth (username, password_hash, password_salt) VALUES (\"{}\", \"{}\", \"{}\")", username, hash, salt);
        conn.execute(query.as_str(), ()).unwrap();
    } 

    if !user_new {
        let mut query = format!("SELECT (password_hash) from auth WHERE username = \"{}\"", username);
        let stored_hash: String = conn.query_row(query.as_str(), (), |row| row.get(0)).unwrap();
        query = format!("SELECT (password_salt) from auth WHERE username = \"{}\"", username);
        let salt: String = conn.query_row(query.as_str(), (), |row| row.get(0)).unwrap();
        let argon_config = Config::default();
        let calculated_hash = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &argon_config).unwrap();
        if calculated_hash != stored_hash {
            return false;
        }
    }

    return true;
}