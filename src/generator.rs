//! Contains the main generator function for the argon2 stateless
//! password generation. 

use argon2::Config;
use zeroize::Zeroize;
use crate::characters;

// changing this breaks fn chunk_to_arr
const HASH_BYTES_PER_GENERATED_PASS_CHAR: u32 = 4;

/// Converts a chunk of 4 bytes into an array of 4 bytes
/// ONLY WORKS FOR 4 BYTE INPUT TO 4 BYTE OUTPUT
/// ### Parameters
/// - `chunk`: an array-slice of unsigned bytes
/// ### Returns
/// An exact 4-byte array of the slice.
fn chunk_to_arr(chunk :&[u8]) -> [u8; 4] {
    [chunk[0], chunk[1], chunk[2], chunk[3]]
}

/// Guarantees password has the property described by key
/// ### Parameters
/// - `pass`: password to test property of
/// - `key` : struct that describes desired properties of the password
/// ### Returns
/// True if and only if pass has the exact property as key. 
/// Nothing missing, nothing extra.
fn guarantee_pass_property(pass: &str, key: &characters::Key) -> bool {
    let re_lower = regex::Regex::new("[a-z]").unwrap();
    let re_upper = regex::Regex::new("[A-Z]").unwrap();
    let re_num   = regex::Regex::new("[0-9]").unwrap();
    let re_sym   = regex::Regex::new(".*[\" !#$%&'()*+,-./:;<=>?@\\[\\\\\\]^_`{|}~].*").unwrap();

    if key.lower ^ re_lower.find(pass).is_some() {return false}
    if key.upper ^ re_upper.find(pass).is_some() {return false}
    if key.num   ^ re_num  .find(pass).is_some() {return false}
    if key.sym   ^ re_sym  .find(pass).is_some() {return false}

    true
}

/// Helper function for `generate_pass()`. For documentation see: `generate_pass()` documentation.
fn argon2_loop(password: &mut str, salt: &[u8], config: &Config, legal_chars: &[char]) -> String {
    let hash = argon2::hash_raw(password.as_bytes(), salt, &config).unwrap_or_else(|err| {
        println!("Encountered error while generating Argon2 hash: {:}", err);
        std::process::exit(1);
    });

    password.zeroize();

    // takes 4-byte chunks from the raw hash
    // uses map to convert these 4-byte chunks into integers (u32)
    // uses these integers with modulo hash function to get indices of legal characters
    // and finally gets legal chars from indices and collects them into a string
    let hash_to_pass: String = hash.chunks(HASH_BYTES_PER_GENERATED_PASS_CHAR as usize)
        .map(|chunk| u32::from_be_bytes(chunk_to_arr(chunk)))
        .map(|integer_val| integer_val % (legal_chars.len()) as u32)
        .map(|char_index| legal_chars[char_index as usize]).collect();

    hash_to_pass
}

/// Generates a password for given service
/// ### Parameters
/// - `pasword` : String representation of the user's master password
/// - `salt`    : Salt i.e. the service title, password num, etc.
/// - `pass_len`: Length of password that needs to be generated
/// - `key`     : Types of characters that need to be present in generated password
/// ### Returns
/// The generated password
/// ### Side-effect
/// Zeroizes the given password.
/// ### Panics
/// None. But the function exits the execution if the hashing function fails
/// ## Usage:
/// ```
/// let key = characters::Key{upper: true, 
///     lower: true, 
///     num: true, 
///     sym: false
/// };
/// 
/// let mut password = "Hello".to_string();
/// let salt = b"randomsalt";
/// let pass = generator::generate_pass(&mut password, salt, 16, key);
/// assert_eq!(pass, "1pXkcUb4LgtFCkXJ".to_string())
/// ```
pub fn generate_pass(password: &mut str, salt: &[u8], pass_len: u8, key: characters::Key) -> String {
    // Recommended numbers. For more information: look into argon2
    let config = argon2::Config {
        variant: argon2::Variant::Argon2id,
        version: argon2::Version::Version13,
        mem_cost: 16384,
        time_cost: 4,
        lanes: 8,
        thread_mode: argon2::ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: (pass_len) as u32 * HASH_BYTES_PER_GENERATED_PASS_CHAR
    };

    let legal_chars = characters::get_pass_building_chars(&key);
    let mut unguaranteed_pass = argon2_loop(password, salt, &config, &legal_chars);
    loop {
        if guarantee_pass_property((unguaranteed_pass).as_str(), &key) {
            return unguaranteed_pass;
        }
        unguaranteed_pass = argon2_loop(&mut unguaranteed_pass, salt, &config, &legal_chars);
    }
}
