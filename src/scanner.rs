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