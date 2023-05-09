use regex::Regex;

pub fn is_friend_code(code: &str) -> bool {
    let re = Regex::new(r"/^\d{4}-\d{4}-\d{4}$/g").unwrap();
    return re.is_match(code)
}

pub fn check_friend_code_hash(code: &str) -> bool {
    let re = Regex::new(r"/^[A-Za-z0-9]{10}$/;").unwrap();
    return re.is_match(code)
}