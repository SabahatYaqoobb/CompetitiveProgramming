use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn find_max_path_sum(node: Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
    match node {
        Some(n) => {
            let n = n.borrow();
            let l = i32::max(0, find_max_path_sum(n.left.clone(), max)); // Recursively find the max path sum for the left child
            let r = i32::max(0, find_max_path_sum(n.right.clone(), max)); // Recursively find the max path sum for the right child
            
            // Update the global max sum
            *max = i32::max(*max, l + r + n.val);
            
            // Return the maximum path sum including the current node
            i32::max(l, r) + n.val
        }
        None => 0,
    }
}

pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max = i32::MIN;
    find_max_path_sum(root, &mut max);
    max
}

fn main() {
    // Test 1: tree: root = [1,2,3]
    let tree1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let test1 = max_path_sum(tree1);
    assert_eq!(test1, 6);

    // Test 2: tree: root = [-10,9,20,null,null,15,7]
    let tree2 = Some(Rc::new(RefCell::new(TreeNode {
        val: -10,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    let test2 = max_path_sum(tree2);
    assert_eq!(test2, 42);
}
