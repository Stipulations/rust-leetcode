impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut a, mut b) = if nums1.len() > nums2.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };

        let total = a.len() + b.len();
        let half = total / 2;

        let (mut left, mut right) = (0, a.len());

        while left <= right {
            let i = (left + right) / 2;
            let j = half - i;

            let a_l = if i > 0 { a[i - 1] } else { i32::MIN };
            let a_r = if i < a.len() { a[i] } else { i32::MAX };

            let b_l = if j > 0 { b[j - 1] } else { i32::MIN };
            let b_r = if j < b.len() { b[j] } else { i32::MAX };

            if a_l <= b_r && b_l <= a_r {
                if total % 2 == 1 {
                    return b_r.min(a_r) as f64;
                }
                return (a_l.max(b_l) as f64 + a_r.min(b_r) as f64) / 2.0;
            } else if a_l > b_r {
                right = i - 1;
            } else {
                left = i + 1;
            }
        }

        unreachable!()
    }
}