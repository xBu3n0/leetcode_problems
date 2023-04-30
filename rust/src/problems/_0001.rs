use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut my_hash : HashMap<i32, i32> = HashMap::new();
        
        for (i, el) in nums.iter().enumerate() {
            match my_hash.get(el) {
                Some(index) => return vec![*index, i as i32],
                None => _ = my_hash.insert(target-el, i as i32)
            }
        }

        return vec![-1, -1]
    }

    pub fn test(nums: Vec<i32>, target: i32) {
        let v: Vec<i32> = Self::two_sum(nums, target);
        let t: String = v.into_iter().map(|i: i32| i.to_string()).collect::<Vec<String>>().join(", ");
        
        println!("[{t}]")
    }
}