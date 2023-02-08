fn solution(s: &str) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let mut i = 2;
    
    while i <= s.len() {
        res.push(s[i-2..i].to_string());
        i += 2;
    }
    if s.len() % 2 == 1 {
        res.push(s[(s.len() - 1)..].to_string() + &String::from("_"));
    }

    res
}

fn main() {
    assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
    assert_eq!(solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
    assert_eq!(solution(""), [] as [&str; 0]);

    // let r = solution("abcdef");

    // for (i, item) in r.iter().enumerate() {
    //     print!("{i}:{item}\n");
    // }
}