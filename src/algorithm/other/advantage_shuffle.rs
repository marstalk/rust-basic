use std::collections::BinaryHeap;
pub struct Solution {}
impl Solution {
    pub fn advantage_shuffle(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums2_heap = BinaryHeap::new();
        for idx in 0..nums2.len() {
            nums2_heap.push((nums2[idx], idx));
        }

        let mut nums1 = nums1;
        nums1.sort();

        let mut res = vec![0; nums1.len()];
        let mut min_idx: i32 = 0;
        let mut max_idx: i32 = nums1.len() as i32 - 1;
        while let Some((n2, idx)) = nums2_heap.pop() {
            println!("{}, {}", n2, idx);
            if nums1[max_idx as usize] > n2 {
                res[idx] = nums1[max_idx as usize];
                max_idx -= 1;
            } else {
                res[idx] = nums1[min_idx as usize];
                min_idx += 1;
            }
            println!("{:?}", res);
        }
        res
    }

    /**
     * use Vec<(idx, val)> instead the help of std::collections::BinaryHeap.
     * custom sort
     */
    pub fn advantage_shuffle2(mut nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0; nums1.len()];
        let mut memo: Vec<(usize, i32)> = nums2
            .iter()
            .enumerate()
            .map(|(idx, &num)| (idx, num))
            .collect();
        memo.sort_by(|a, b| b.1.cmp(&a.1));
        nums1.sort();
        let mut left = 0;
        let mut right: i32 = nums1.len() as i32 - 1;
        for (_, &(res_index, n2)) in memo.iter().enumerate() {
            if nums1[right as usize] <= n2 {
                res[res_index] = nums1[left];
                left += 1;
            } else {
                res[res_index] = nums1[right as usize];
                right -= 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advantage_shuffle2() {
        assert_eq!(
            Solution::advantage_shuffle2(vec![2, 7, 11, 15], vec![1, 10, 4, 11]),
            vec![2, 11, 7, 15]
        );
        assert_eq!(
            Solution::advantage_shuffle2(vec![12, 24, 8, 32], vec![13, 25, 32, 11]),
            vec![24, 32, 8, 12]
        );
    }

    #[test]
    fn test_advantage_shuffle() {
        assert_eq!(
            Solution::advantage_shuffle(vec![2, 7, 11, 15], vec![1, 10, 4, 11]),
            vec![2, 11, 7, 15]
        );
        assert_eq!(
            Solution::advantage_shuffle(vec![12, 24, 8, 32], vec![13, 25, 32, 11]),
            vec![24, 32, 8, 12]
        );
    }
}
