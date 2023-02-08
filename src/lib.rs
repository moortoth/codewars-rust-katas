/// https://www.codewars.com/kata/5277c8a221e209d3f6000b56
/// 
/// Write a function that takes a string of braces, and determines if the order of the braces is valid. It should return true if the string is valid, and false if it's invalid.
/// 
/// This Kata is similar to the Valid Parentheses Kata, but introduces new characters: brackets [], and curly braces {}. Thanks to @arnedag for the idea!
/// 
/// All input strings will be nonempty, and will only consist of parentheses, brackets and curly braces: ()[]{}.
/// What is considered Valid?
/// 
/// A string of braces is considered valid if all braces are matched with the correct brace.
/// Examples
/// 
/// "(){}[]"   =>  True
/// "([{}])"   =>  True
/// "(}"       =>  False
/// "[(])"     =>  False
/// "[({})](]" =>  False

pub fn valid_braces(s: &str) -> bool {
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

/// https://www.codewars.com/kata/54bf1c2cd5b56cc47f0007a1
///
/// Write a function that will return the count of distinct case-insensitive alphabetic characters and numeric digits that occur more than once in the input string. The input string can be assumed to contain only alphabets (both uppercase and lowercase) and numeric digits.
/// Example
///
/// "abcde" -> 0 # no characters repeats more than once
/// "aabbcde" -> 2 # 'a' and 'b'
/// "aabBcde" -> 2 # 'a' occurs twice and 'b' twice (`b` and `B`)

pub fn count_duplicates(text: &str) -> u32 {
    use std::collections::HashMap;
    let mut distinct_chars: HashMap<u8, u32> = HashMap::new();

    for &iter in text.as_bytes() {
        let lcc = iter.to_ascii_lowercase();
        distinct_chars.entry(lcc).and_modify(|x| *x=*x+1 ).or_insert(1);
    }

    distinct_chars.iter().filter(|x| *x.1 > 1u32).count() as u32
}

/// https://www.codewars.com/kata/56747fd5cb988479af000028
/// 
/// You are going to be given a word. Your job is to return 
/// the middle character of the word. If the word's length is odd, 
/// return the middle character. If the word's length is even, return the middle 2 characters.

pub fn get_middle(s:&str) -> &str {
    if s.len() % 2 == 1 {
        &s[s.len()/2 .. s.len()/2 + 1]
    }
    else { 
        &s[s.len()/2 - 1 .. s.len()/2 + 1] 
    }
}

/// https://www.codewars.com/kata/515de9ae9dcfc28eb6000001
///
/// Complete the solution so that it splits the string into pairs of two characters. 
/// If the string contains an odd number of characters then it should replace 
/// the missing second character of the final pair with an underscore ('_').
///
/// Examples:
///
/// * 'abc' =>  ['ab', 'c_']
/// * 'abcdef' => ['ab', 'cd', 'ef']

pub fn split_strings(s: &str) -> Vec<String> {
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

/// https://www.codewars.com/kata/57eb8fcdf670e99d9b000272
/// 
/// Given a string of words, you need to find the highest scoring word.
/// 
/// Each letter of a word scores points according to its position in the alphabet: 
/// a = 1, b = 2, c = 3 etc.
/// For example, the score of abad is 8 (1 + 2 + 1 + 4).
/// You need to return the highest scoring word as a string.
/// If two words score the same, return the word that appears earliest in the original string.
/// All letters will be lowercase and all inputs will be valid.

pub fn high(input: &str) -> &str {
    let mut h_rate = 0;
    let mut res = input;

    for part in input.split(' ') {
        let mut rate: i32 = 0;
        
        for &c in part.as_bytes() {
            rate += (c - b'a' + 1) as i32;
        }

        if h_rate < rate {
            res = part;
            h_rate = rate;
        }
    }

    res
}

/// https://www.codewars.com/kata/51e0007c1f9378fa810002a9
/// 
/// Write a simple parser that will parse and run Deadfish.
/// Deadfish has 4 commands, each 1 character long:
///     i increments the value (initially 0)
///     d decrements the value
///     s squares the value
///     o outputs the value into the return array
/// Invalid characters should be ignored.
/// 
/// parse("iiisdoso") => [ 8, 64 ] 

pub fn deadfish_parse(code: &str) -> Vec<i32> {
    let mut res = vec!();
    let mut curr = 0;

    for c in code.as_bytes() {
        match c {
            b'i' => {curr += 1},
            b'd' => {curr -= 1},
            b's' => {curr *= curr},
            b'o' => {res.push(curr)}
            _ => {},
        }
    }

    res
    
}

/// https://www.codewars.com/kata/554ca54ffa7d91b236000023
///
/// Given a list and a number, create a new list that contains 
/// each number of list at most N times, without reordering.
///
/// For example if the input number is 2, and the input list is 
/// [1,2,3,1,2,1,2,3], you take [1,2,3,1,2], drop the next [1,2] 
/// since this would lead to 1 and 2 being in the result 3 times, and then take 3,
/// which leads to [1,2,3,1,2,3].
///
/// With list [20,37,20,21] and number 1, the result would be [20,37,21]. 

pub fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    use std::collections::HashMap;

    let mut occ: HashMap<u8, i32> = HashMap::new();

    for &el in lst {
        *occ.entry(el).or_insert(1) += 1;
    }

    todo!()
}

/// https://www.codewars.com/kata/5626b561280a42ecc50000d1
/// 
/// We need a function to collect these numbers, that may 
/// receive two integers aaa, bbb that defines the range [a,b][a, b][a,b] (inclusive)
/// and outputs a list of the sorted numbers in the range that fulfills the property described above.
/// 
/// Examples
/// 
///1, 10  --> [1, 2, 3, 4, 5, 6, 7, 8, 9]
///1, 100 --> [1, 2, 3, 4, 5, 6, 7, 8, 9, 89]

pub fn sum_dig_pow(a: u64, b: u64) -> Vec<u64> {
    let mut curr_num = a;
    let mut res = vec![];

    while curr_num <= b {
        let mut digits: Vec<u64> = vec![];
        let mut rem = curr_num;

        while rem > 0 {
            digits.push(rem % 10);
            rem /= 10;
        }

        digits.reverse();

        let mut sum:u64 = 0u64;
        
        for (i, dig) in digits.iter().enumerate() {
            let mut exp = 0;
            let mut dig_expd = 1;
            
            while exp < i + 1 {
                dig_expd *= dig;
                exp += 1;
            }

            sum += dig_expd;
        }

        if sum == curr_num {
            res.push(curr_num);
            println!("{curr_num} ");
        }

        curr_num += 1;
    }

    res
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_braces() {
        assert_eq!(valid_braces("[(])"), false);
        assert_eq!(valid_braces("[]"), true);
        assert_eq!(valid_braces("[()]"), true);
        assert_eq!(valid_braces("[({})]"), true);
        assert_eq!(valid_braces("[({"), false);
        assert_eq!(valid_braces("})]"), false);
        assert_eq!(valid_braces("[({})]}]"), false);
    }

    #[test]
    fn test_split_strings() {
        assert_eq!(split_strings("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(split_strings("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(split_strings(""), [] as [&str; 0]);
    }

    #[test]
    fn test_count_duplicates() {
        assert_eq!(count_duplicates("abcde"), 0);
        assert_eq!(count_duplicates("abcdea"), 1);
        assert_eq!(count_duplicates("indivisibility"), 1);
    }

    #[test]
    fn test_get_middle() {
        assert_eq!(get_middle("test"),"es");
        assert_eq!(get_middle("testing"),"t");
        assert_eq!(get_middle("middle"),"dd");
        assert_eq!(get_middle("A"),"A");
        assert_eq!(get_middle("of"),"of");
    }

    #[test]
    fn test_high() {
        assert_eq!(high("man i need a taxi up to ubud"), "taxi");               
        assert_eq!(high("what time are we climbing up the volcano"), "volcano");
        assert_eq!(high("take me to semynak"), "semynak");                      
        assert_eq!(high("massage yes massage yes massage"), "massage");         
        assert_eq!(high("take two bintang and a dance please"), "bintang"); 
        assert_eq!(high("aa b"), "aa");         
        assert_eq!(high("b aa"), "b");     
        assert_eq!(high("bb d"), "bb");                            
        assert_eq!(high("d bb"), "d"); 
        assert_eq!(high("aaa b"), "aaa");                                     
    }

    #[test]
    fn test_deadfish_parse() {
        assert_eq!(deadfish_parse("iiisdoso"), vec![8, 64]);
        assert_eq!(deadfish_parse("iiisdosodddddiso"), vec![8, 64, 3600]);
    }

    #[test]
    fn test_sum_dig_pow() {
        assert_eq!(sum_dig_pow(1, 10), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(sum_dig_pow(1, 100), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 89]);
        assert_eq!(sum_dig_pow(89, 89), vec![89]);
        assert_eq!(sum_dig_pow(10, 100), vec![89]);
        assert_eq!(sum_dig_pow(90, 100), vec![]);
        assert_eq!(sum_dig_pow(89, 135), vec![89, 135]);
    }
}
