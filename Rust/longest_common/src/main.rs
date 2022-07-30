// Write a function to find the longest common prefix string amongst an array of strings.

// If there is no common prefix, return an empty string "".
struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::new();
        }
        let mut min_len = strs[0].len();
        for i in 1..strs.len() {
            if strs[i].len() < min_len {
                min_len = strs[i].len();
            }
        }
        let mut res = String::new();
        for i in 0..min_len {
            let mut flag = true;
            for j in 1..strs.len() {
                if strs[j].as_bytes()[i] != strs[0].as_bytes()[i] {
                    flag = false;
                    break;
                }
            }
            if flag {
                res.push(strs[0].as_bytes()[i] as char);
            } else {
                break;
            }
        }
        res
    }
}

fn main() {
    let strs = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    println!("{:?}", Solution::longest_common_prefix(strs));
}
