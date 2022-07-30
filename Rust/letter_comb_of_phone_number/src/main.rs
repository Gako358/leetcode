// Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.

// A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut result = vec![];
        if digits.is_empty() {
            return result;
        }
        let mut map = vec![
            vec![],
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
            vec!['j', 'k', 'l'],
            vec!['m', 'n', 'o'],
            vec!['p', 'q', 'r', 's'],
            vec!['t', 'u', 'v'],
            vec!['w', 'x', 'y', 'z'],
        ];
        let mut cur = vec![];
        for c in digits.chars() {
            let c = c.to_digit(10).unwrap() as usize - 2;
            if c == usize::MAX {
                continue;
            }
            let mut next = vec![];
            for c in map[c].iter() {
                for s in cur.iter() {
                    next.push(s.clone() + c.to_string().as_str());
                }
            }
            cur = next;
        }
        result.append(&mut cur);
        result
    }
}

fn main() {
    println!("{:?}", Solution::letter_combinations("23".to_string()));
}
