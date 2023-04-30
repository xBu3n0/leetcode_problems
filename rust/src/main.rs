mod problems;

use crate::problems::*;

fn main() {
    // Problema 1
    println!("\nProblema 1");
    _0001::Solution::test(vec![1, 2, 3, 4], 3);
    
    // Problema 2
    println!("\nProblema 2");
    let l1: Option<Box<_0002::ListNode>> = _0002::ListNode::from_vec(vec![1, 2, 3, 4, 5]);
    let l2: Option<Box<_0002::ListNode>> = _0002::ListNode::from_vec(vec![1, 2, 3, 4, 5]);
    _0002::Solution::test(l1, l2);

    // _0003::Solution::test(vec![1, 2, 3, 4], 3);
    // _0004::Solution::test(vec![1, 2, 3, 4], 3);
    
}