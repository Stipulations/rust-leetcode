use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut seen = HashSet::new();
        let mut left = 0;
        let mut max_len = 0;
        let chars: Vec<char> = s.chars().collect();

        for (right, &char) in chars.iter().enumerate() {
            while seen.contains(&char) {
                seen.remove(&chars[left]);
                left += 1;
            }

            seen.insert(char);
            max_len = max_len.max(right - left + 1);
        }
        max_len as i32
    }
}
