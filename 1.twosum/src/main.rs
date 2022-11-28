use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut storage: HashMap<i32, i32> = HashMap::new();

    for (idx, num) in nums.iter().enumerate() {
        match storage.get(&num) {
            Some(n) => {
                return vec![*n, idx as i32];
            }
            None => {
                storage.insert(target - num, idx as i32);
            }
        }
    }
    vec![-1, -1]
}
fn main() {
    let res1: Vec<i32> = two_sum(vec![2, 7, 11, 15], 9);
    let res2: Vec<i32> = two_sum(vec![3, 2, 4], 6);
    let res3: Vec<i32> = two_sum(vec![3, 3], 6);

    println!("res1: {:?} res2: {:?} res3: {:?}", res1, res2, res3);
}
