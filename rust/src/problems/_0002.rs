#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    // Fora do enunciado do problema
    pub fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        Self::iter(v.into_iter())
    }

    fn iter(mut v: std::vec::IntoIter<i32>) -> Option<Box<ListNode>> {
        match v.next() {
            Some(n) => Some(
                Box::from(
                    ListNode {
                        val: n,
                        next: Self::iter(v)
                    }
                )
            ),
            None => None
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn add(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, vai_um: i32) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(l), Some(r)) => Some(
                Box::from(
                    ListNode {
                        val: (l.val+r.val+vai_um) % 10,
                        next: Self::add(l.next, r.next, (l.val+r.val+vai_um)/10)
                    }
                )
            ),
            (Some(l), None) | (None, Some(l)) => Some(
                Box::from(
                    ListNode {
                        val: (l.val+vai_um) % 10,
                        next: Self::add(l.next, None, (l.val+vai_um)/10)
                    }
                )
            ),
            (None, None) => match vai_um {
                1 => Some(
                Box::from(
                    ListNode {
                        val: 1,
                        next: None
                    }
                )
            ),
                _ => None
            }
        }
    }

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::add(l1, l2, 0)
    }

    pub fn print(l: Option<Box<ListNode>>) {
        match l {
            Some(next) => {
                print!("{}", next.val);
                Self::print(next.next)
            },
            None => ()
        }
    }

    pub fn test(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) {
        let sum = Self::add_two_numbers(l1, l2);
        Self::print(sum);
        println!("\n\n")
    }
}