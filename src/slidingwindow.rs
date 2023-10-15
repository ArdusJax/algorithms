// Sliding window algorithm
#[allow(dead_code)]
fn find_length(nums: Vec<i32>, k: i32) -> i32 {
    let (mut left, mut curr, mut ans): (i32,i32,i32,) = (0,0,0);
    for right in 0..nums.len() {
        curr += nums[right];
        while curr > k {
            curr -= nums[left as usize];
            left += 1;
        }
        ans = std::cmp::max(ans, right as i32 - left as i32 + 1);
    }
    ans
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn find_my_length() {
        let data = vec![3,1,2,7,4,2,1,1,5];
        let ans = find_length(data, 8);
        assert_eq!(ans,4);
    }
}
