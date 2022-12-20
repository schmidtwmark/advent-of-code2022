use std::borrow::Borrow;
use std::fmt::{Debug, Display};

use aoc::Solver;
use hashbrown::HashMap;
use itertools::Itertools;
use log::debug;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct ListNode<T>
where
    T: Display + Copy + Clone,
{
    val: T,
    next: Option<*mut ListNode<T>>,
    prev: Option<*mut ListNode<T>>,
}

fn to_vec(root: *mut ListNode<i32>, count: usize) -> Vec<i32> {
    let mut node = root;
    (0..count)
        .map(|_| unsafe {
            let val = (*node).val;
            node = (*node).next.unwrap();
            val
        })
        .collect_vec()
}

fn get_nth(root: *mut ListNode<i32>, n: i32) -> *mut ListNode<i32> {
    let mut node = root;
    for _ in 0..n {
        unsafe {
            node = (*node).next.unwrap();
        }
    }
    node
}
fn get_nth_rev(root: *mut ListNode<i32>, n: i32) -> *mut ListNode<i32> {
    let mut node = root;
    for _ in 0..n {
        unsafe {
            node = (*node).prev.unwrap();
        }
    }
    node
}

fn mix(root: *mut ListNode<i32>, nums: &mut Vec<(i32, Box<ListNode<i32>>)>) {
    let count = nums.len();
    for (num, node) in nums {
        assert_eq!(num, &node.val);
        let mut current: *mut ListNode<i32> = &mut **(node);
        match num.signum() {
            1 => {
                for _ in 0..*num {
                    // swap with next
                    unsafe {
                        let mut next_node = (*current).next.unwrap();
                        let mut prev_node = (*current).prev.unwrap();
                        let mut next_next_node = (*next_node).next.unwrap();
                        debug!("swapping {} with {}", (*current).val, (*next_node).val);

                        (*current).next = (*next_node).next;
                        (*current).prev = Some(next_node);
                        (*next_next_node).prev = Some(current);

                        (*next_node).next = Some(current);
                        (*next_node).prev = Some(prev_node);

                        (*prev_node).next = Some(next_node);

                        // current = next_next_node;
                    }
                }
            }
            -1 => {
                for _ in 0..num.abs() {
                    // swap with prev
                    unsafe {
                        let mut next_node = (*current).next.unwrap();
                        let mut prev_node = (*current).prev.unwrap();
                        let mut prev_prev_node = (*prev_node).prev.unwrap();

                        debug!("swapping {} with {}", (*current).val, (*prev_node).val);
                        (*current).next = Some(prev_node);
                        (*current).prev = (*prev_node).prev;
                        (*prev_prev_node).next = Some(current);

                        (*prev_node).next = Some(next_node);
                        (*prev_node).prev = Some(current);

                        (*next_node).prev = Some(prev_node);
                        // current = prev_prev_node;
                    }
                }
            }
            _ => {}
        }
        debug!("After mixing {}: {:?}", num, to_vec(root, count));
    }
}

struct Solution {}
impl Solver<'_, i32> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> i32 {
        let mut last: Option<*mut ListNode<i32>> = None;
        let mut root: Option<*mut ListNode<i32>> = None;

        let mut nums = Vec::new();

        let mut count = 0;

        for line in lines {
            let num = line.parse::<i32>().unwrap();
            count += 1;
            let node = Box::new(ListNode {
                val: num,
                next: None,
                prev: last,
            });
            nums.push((num, node));

            let node_ptr: *mut ListNode<i32> = &mut *(nums.last_mut().unwrap().1);

            if let Some(previous) = last {
                unsafe {
                    (*previous).next = Some(node_ptr);
                }
            } else {
                root = Some(node_ptr);
            }
            last = Some(node_ptr);
        }

        assert_eq!(count, nums.len());
        assert!(root.is_some());
        assert!(last.is_some());

        unsafe {
            (*last.unwrap()).next = root;
            (*root.unwrap()).prev = last;
        }

        let root = root.unwrap();
        let last = last.unwrap();

        debug!("Before mixing: {:?}", to_vec(root, count));

        // root.prev = Some(last.clone());

        mix(root, &mut nums);
        debug!("After mixing: {:?}", to_vec(root, count));

        let zero = &mut *(nums.iter_mut().find(|(num, _)| *num == 0).unwrap().1);

        // Numbers are not guaranteed to be unique

        [1000, 2000, 3000]
            .iter()
            .map(|x| {
                let val = unsafe { (*get_nth(zero, *x)).val };
                debug!("{}th after 0: {}", x, val);
                val
            })
            .sum()
    }

    fn solve_part_two(&self, lines: &[&str]) -> i32 {
        Default::default()
    }
}

fn main() {
    let sample = include_str!("../../samples/20.txt");
    let input = include_str!("../../inputs/20.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 3), // TODO: Fill in expected sample result
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, Default::default()), // TODO: Fill in expected sample result
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
