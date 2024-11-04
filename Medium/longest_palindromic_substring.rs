impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }

        let (mut start, mut max_len) = (0, 0);
        let s_bytes = s.as_bytes();

        for i in 0..s.len() {
            let mut left = i;
            let mut right = i;
            while left > 0 && right < s.len() - 1 && s_bytes[left - 1] == s_bytes[right + 1] {
                left -= 1;
                right += 1;
            }
            let len = right - left + 1;
            if len > max_len {
                max_len = len;
                start = left;
            }

            if i < s.len() - 1 && s_bytes[i] == s_bytes[i + 1] {
                let mut left = i;
                let mut right = i + 1;
                while left > 0 && right < s.len() - 1 && s_bytes[left - 1] == s_bytes[right + 1] {
                    left -= 1;
                    right += 1;
                }
                let len = right - left + 1;
                if len > max_len {
                    max_len = len;
                    start = left;
                }
            }
        }

        s[start..start + max_len].to_string()
    }
}