# CodeWars--String-incrementer-5-kyu---Passed
Your job is to write a function which increments a string, to create a new string.

If the string already ends with a number, the number should be incremented by 1.
If the string does not end with a number. the number 1 should be appended to the new string.
Examples:

foo -> foo1

foobar23 -> foobar24

foo0042 -> foo0043

foo9 -> foo10

foo099 -> foo100

Attention: If the number has leading zeros the amount of digits should be considered.


TEST CASES:
#[cfg(test)]
mod tests {
    use super::increment_string;
    use rand::{thread_rng, Rng, seq::SliceRandom};
    use regex::Regex;
    use once_cell::sync::Lazy;

    fn reference_solution(s: &str) -> String {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d*$)").unwrap());
        match RE.find(s) {
            None => format!("{}1", s),
            Some(suffix) => RE.replace(s, increment_suffix(suffix.as_str())).to_string()
        }
    }

    fn increment_suffix(s: &str) -> String {
        let mut digit_vec = s.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
        let mut r = 1;
        for i in (0..s.len()).rev() {
            let n = digit_vec[i] + r;
            digit_vec[i] = n % 10;
            r = n / 10;
            if r == 0 { break }
        }
        (1..=r).chain(digit_vec).map(|x| std::char::from_digit(x, 10).unwrap()).collect()
    }
    
    fn dotest(s: &str, expected: &str) {
        let actual = increment_string(s);
        assert!(actual == expected, 
            "Test failed with s = \"{s}\"\nExpected \"{expected}\"\nBut got \"{actual}\"")
    }

    #[test]
    fn fixed_tests() {
        dotest("foo", "foo1");
        dotest("foobar001", "foobar002");
        dotest("foobar1", "foobar2");
        dotest("foobar00", "foobar01");
        dotest("foobar99", "foobar100");
        dotest("foobar099", "foobar100");
        dotest("", "1");
        dotest("foobar000", "foobar001");
        dotest("foobar999", "foobar1000");
        dotest("foobar00999", "foobar01000");
        dotest("foobar001", "foobar002");
        dotest("foobar1", "foobar2");
        dotest("1", "2");
        dotest("009", "010");
        dotest("HereComesATrickyTest99999999999999999999999999999999999999",
                   "HereComesATrickyTest100000000000000000000000000000000000000");
        dotest("HereCome9TrickyTests99999999999999999999999999999999999999",
                   "HereCome9TrickyTests100000000000000000000000000000000000000");
    }
    #[test]
    fn random_tests() {
        let mut rng = thread_rng();
        let all_chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        for _ in 0..300 {
            let mut s = String::from("");
            let mut l = rng.gen_range(0..=100);
            for _ in 0..l {
                s.push(*all_chars.choose(&mut rng).unwrap())
            }
            l = rng.gen_range(0..=101);
            if l > 0 && rng.gen_range(0..10) < 2 {
                s = format!("{}{}", s, "9".repeat(l))
            } else {
                for _ in 0..l {
                    s.push(char::from_digit(rng.gen_range(0..15).min(9), 10).unwrap())
                }
            }
            dotest(&s, &reference_solution(&s));
        }
    }
}
