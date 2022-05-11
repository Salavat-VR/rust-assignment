#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}


pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut v1 = list2vec(&l1);
    let mut v2 = list2vec(&l2);
    v1.reverse();
    v2.reverse();
    let mut next = None;
    let mut carry = 0;
    for i in 0..=std::cmp::max(v1.len(), v2.len()) {
        let mut val =
            carry + if i < v1.len() { v1[i] } else { 0 } + if i < v2.len() { v2[i] } else { 0 };
        carry = if val > 9 {
            val -= 10;
            1
        } else {
            0
        };
        if i < std::cmp::max(v1.len(), v2.len()) || val != 0 {
            next = Some(Box::new(ListNode { val, next }));
        }
    }
    next
}

fn list2vec(list: &Option<Box<ListNode>>) -> Vec<i32> {
    if list.is_none() {
        return vec![0];
    }
    let mut ret = Vec::new();
    let mut node = list;
    while let Some(n) = node {
        ret.push(n.val);
        node = &n.next;
    }
    ret
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        // 7243
        let number_1 = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode::new(3))),
                })),
            })),
        }));
        // 564
        let number_2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));

        // 7807
        let right_add_result = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 8,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode::new(7))),
                })),
            })),
        }));
        assert_eq!(add_two_numbers(number_1, number_2), right_add_result);
    }


    #[test]
    fn test_case_2() {
        // 243
        let number_1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(3))),
            })),
        }));

        // 564
        let number_2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));

        // 807
        let right_add_result = Some(Box::new(ListNode {
            val: 8,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode::new(7))),
            })),
        }));

        assert_eq!(add_two_numbers(number_1, number_2), right_add_result);
    }


    #[test]
    fn test_case_3() {
        // 0 + 0 = 0
        let zero = Some(Box::new(ListNode::new(0)));

        assert_eq!(add_two_numbers(zero.clone(), zero.clone()), zero.clone());
    }
}