fn valid_braces(s: &str) -> bool {
    let mut v: Vec<u8> = Vec::new();

    for &item in s.as_bytes() {
        match item {
            b'{' |  b'[' | b'(' => { 
                v.push(item)
            },
            cb @ (b'}' |  b']' | b')') => {
                if let Some(ob) = v.pop() {
                    match (ob, cb) {
                        (b'{', b'}') | (b'[', b']') | (b'(', b')') => { },
                        _ => { return false; }
                    }
                }
                else { return false; }
            },
            _ => { }
        }
    }

    v.len() == 0
}

fn main() {
    assert_eq!(valid_braces("[(])"), false);
    assert_eq!(valid_braces("[]"), true);
    assert_eq!(valid_braces("[()]"), true);
    assert_eq!(valid_braces("[({})]"), true);
    assert_eq!(valid_braces("[({"), false);
    assert_eq!(valid_braces("})]"), false);
    assert_eq!(valid_braces("[({})]}]"), false);
}
