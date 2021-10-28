fn main() {
    let test_nums_1 = &[1, 2, 3];
    let test_nums_2 = &[1, u32::MAX];
    assert_eq!(sum(test_nums_1), Some(6));
    assert_eq!(sum(test_nums_2), None);
}

fn sum(nums: &[u32]) -> Option<u32> {
    let mut res: u32 = 0;
    for num in nums {
        let (res_tmp, flag) = res.overflowing_add(*num);
        if flag {
            return None;
        }
        res = res_tmp
    }
    Some(res)
}
