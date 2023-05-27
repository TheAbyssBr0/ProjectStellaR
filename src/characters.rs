//! Contains list of password-legal characters in their categories
//! The `Key` struct and a helper method for main generator

/// A key that represents properties of generated password.
/// The desired character groups can be marked as true and 
/// undesired ones can be marked as false.
#[derive(Clone, Copy)]
pub struct Key {
    pub upper: bool,
    pub lower: bool,
    pub num: bool,
    pub sym: bool
}

impl Key {
    /// Returns the default `Key` where all of its values are `true`
    /// ### Returns
    /// A `Key` struct in default state i.e. with all its parameters set to `true`
    pub fn default() -> Self {
        Key {
            upper: true,
            lower: true, 
            num: true, 
            sym: true
        }
    }

    /// Converts 4 bool array into a key struct for making Key from user input
    /// ### Returns
    /// A `Key` struct in the state as specified
    pub fn from_arr(bools: [bool; 4]) -> Self {
        Key {
            upper: bools[0],
            lower: bools[1],
            num: bools[2],
            sym: bools[3]
        }
    }

    /// Makes a string from the `Key` struct
    /// ### Returns
    /// A `String` representation of the state of the `Key` struct
    pub fn to_str(&self) -> String {
        let mut string_return_val = String::new();
        if self.lower {string_return_val.push_str("lowercase ")};
        if self.upper {string_return_val.push_str("uppercase ")};
        if self.num {string_return_val.push_str("numbers ")};
        if self.sym {string_return_val.push_str("symbols")};

        string_return_val
    }
}

/// lower case characters that can be used in passwords
const LOWER_CASE: [char ;26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
                                 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
                                 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

/// upper case characters that can be used in passwords
const UPPER_CASE: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
                                 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
                                 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

/// num case characters that can be used in passwords
const NUM: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

/// symbol case characters that can be used in passwords
const SYMBOL: [char; 33] = ['"', ' ', '!', '#', '$', '%', '&', '\'', '(',
                            ')', '*', '+', ',', '-', '.', '/', ':', ';', 
                            '<', '=', '>', '?', '@', '[', '\\', ']', '^',
                             '_', '`', '{', '|', '}', '~'];

/// Builds a vector of legal characters in spec. to given key
/// ### Params
/// `key`: a key struct containing 4 booleans; turn on if chars
///        of that type are desired
/// ### Retunrs
/// A vector of legal characters according to given key
pub fn get_pass_building_chars(key: &Key) -> Vec<char> {
    let mut return_vec: Vec<char> = Vec::new();

    if key.lower {return_vec.append(&mut LOWER_CASE.to_vec())};
    if key.upper {return_vec.append(&mut UPPER_CASE.to_vec())};
    if key.num   {return_vec.append(&mut NUM.to_vec())};
    if key.sym   {return_vec.append(&mut SYMBOL.to_vec())};

    return_vec
}

