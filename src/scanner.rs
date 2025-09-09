pub fn is_beginning_digit(c: char) -> bool {
    c >= '1' && c <= '9'
}

pub fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

pub fn is_bin_digit(c: char) -> bool {
    c == '0' || c == '1'
}

pub fn is_oct_digit(c: char) -> bool {
    c >= '0' && c <= '7'
}

pub fn is_hex_digit(c: char) -> bool {
    is_digit(c) || (c >= 'A' && c <= 'F') || (c >= 'a' && c <= 'f')
}

pub fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') ||
    (c >= 'A' && c <= 'Z') ||
    c == '_'
}

pub fn is_alphanumeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

pub fn substring(string: &String, a: usize, b: usize) -> String {
    string.chars().skip(a).take(b - a).collect()
}

fn char_check_str(c: &str, function: fn(char) -> bool) -> bool {
   c.chars().nth(0).map(|x| function(x)).unwrap_or(false)
}

pub fn is_str_beginning_digit(c: &str) -> bool {
    char_check_str(c, is_beginning_digit)
}

pub fn is_str_digit(c: &str) -> bool {
    char_check_str(c, is_digit)
}

pub fn is_str_bin_digit(c: &str) -> bool {
    char_check_str(c, is_bin_digit)
}

pub fn is_str_oct_digit(c: &str) -> bool {
    char_check_str(c, is_oct_digit)
}

pub fn is_str_hex_digit(c: &str) -> bool {
    char_check_str(c, is_hex_digit)
}

pub fn is_str_alpha(c: &str) -> bool {
    char_check_str(c, is_alpha)
}

pub fn is_str_alphanumeric(c: &str) -> bool {
    char_check_str(c, is_alphanumeric)
}