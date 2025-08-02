/// Find the max in a unimodal array.
/// An array is called unimodal iff it can be split into an
/// increasing sequence followed by a decreasing sequence.
/// Returns the index of max if found, else None
pub fn find_max_unimodal_array(nums: &[i32]) -> Option<usize> {
    if nums.is_empty() {
        return Option::None;
    }

    if nums.len() == 1 {
        return Option::Some(0);
    }

    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        let mid: usize = (left + right) / 2;

        if mid < (nums.len() - 1) && nums[mid + 1] > nums[mid] {
            left = mid + 1
        } else if mid > 0 && nums[mid - 1] > nums[mid] {
            right = mid - 1
        } else {
            return Some(mid);
        }
    }

    Option::None
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_sorted_array() {
        let actual = find_max_unimodal_array(&[1, 2, 3, 4, 5]);
        assert_eq!(Some(4), actual);
    }

    #[test]
    fn test_reverse_sorted_array() {
        let actual = find_max_unimodal_array(&[5, 4, 3, 2, 1]);
        assert_eq!(Some(0), actual);
    }

    #[test]
    fn test_with_peak_in_middle() {
        let actual = find_max_unimodal_array(&[4, 5, 6, 3, 2, 1]);
        assert_eq!(Some(2), actual);
    }

    #[test]
    fn test_with_peak_elsewhere() {
        let actual = find_max_unimodal_array(&[4, 5, 6, 7, 2, 1]);
        assert_eq!(Some(3), actual);
    }

    #[test]
    fn test_with_multiple_peaks() {
        let actual = find_max_unimodal_array(&[4, 5, 3, 7, 8, 1]);
        let peaks: HashSet<&Option<usize>> = HashSet::from_iter([Some(1), Some(4)].iter());
        assert_eq!(peaks.contains(&actual), true);
    }
}
