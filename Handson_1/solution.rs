use std::collections::VecDeque;

#[derive(Debug)]
struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

#[derive(Debug)]
struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(parent_id < self.nodes.len(), "Parent node id does not exist");

        if is_left {
            assert!(self.nodes[parent_id].id_left.is_none(), "Parent node already has a left child");
        } else {
            assert!(self.nodes[parent_id].id_right.is_none(), "Parent node already has a right child");
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);
        child_id
    }

    pub fn sum(&self) -> u32 {
        self.rec_sum(Some(0))
    }

    fn rec_sum(&self, node_id: Option<usize>) -> u32 {
        node_id.map_or(0, |id| {
            let node = &self.nodes[id];
            self.rec_sum(node.id_left) + self.rec_sum(node.id_right) + node.key
        })
    }

    pub fn is_bst(&self) -> bool {
        self.rec_is_bst(Some(0), u32::MIN, u32::MAX)
    }

    fn rec_is_bst(&self, node_id: Option<usize>, min: u32, max: u32) -> bool {
        node_id.map_or(true, |id| {
            let node = &self.nodes[id];
            if node.key <= min || node.key >= max {
                return false;
            }
            self.rec_is_bst(node.id_left, min, node.key) && self.rec_is_bst(node.id_right, node.key, max)
        })
    }

    pub fn is_balanced(&self) -> bool {
        self.rec_is_balanced(Some(0)) != -1
    }

    fn rec_is_balanced(&self, node_id: Option<usize>) -> i32 {
        node_id.map_or(0, |id| {
            let node = &self.nodes[id];
            let left_height = self.rec_is_balanced(node.id_left);
            let right_height = self.rec_is_balanced(node.id_right);

            if left_height == -1 || right_height == -1 || (left_height - right_height).abs() > 1 {
                -1
            } else {
                1 + left_height.max(right_height)
            }
        })
    }

    pub fn is_max_heap(&self) -> bool {
        self.iter_is_max_heap(Some(0))
    }

    fn iter_is_max_heap(&self, node_id: Option<usize>) -> bool {
        node_id.map_or(true, |id| {
            let mut queue = VecDeque::new();
            let node = &self.nodes[id];
            let mut last_level = false;

            queue.push_back(node);
            while let Some(curr_node) = queue.pop_front() {
                if last_level && (curr_node.id_left.is_some() || curr_node.id_right.is_some()) {
                    return false;
                }
                if curr_node.id_left.is_none() && curr_node.id_right.is_some() {
                    return false;
                }

                if let Some(id_left) = curr_node.id_left {
                    let left_child = &self.nodes[id_left];
                    if curr_node.id_right.is_none() {
                        last_level = true;
                    }
                    if curr_node.key < left_child.key {
                        return false;
                    }
                    queue.push_back(left_child);
                }

                if let Some(id_right) = curr_node.id_right {
                    let right_child = &self.nodes[id_right];
                    if curr_node.key < right_child.key {
                        return false;
                    }
                    queue.push_back(right_child);
                }

                if curr_node.id_left.is_none() && curr_node.id_right.is_none() {
                    last_level = true;
                }
            }
            true
        })
    }
}

fn main() {
    let mut tree = Tree::with_root(10);
    tree.add_node(0, 1, true); // id 1
    tree.add_node(0, 22, false); // id 2
    println!("Sum: {}", tree.sum());
    println!("Is balanced: {}", tree.is_balanced());
    println!("Is BST: {}", tree.is_bst());
    println!("Is max-heap: {}", tree.is_max_heap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut tree = Tree::with_root(10);
        assert_eq!(tree.sum(), 10);
        tree.add_node(0, 5, true);
        tree.add_node(0, 22, false);
        assert_eq!(tree.sum(), 37);
    }

    #[test]
    fn test_is_bst() {
        let mut tree = Tree::with_root(10);
        assert!(tree.is_bst());
        tree.add_node(0, 8, true);
        tree.add_node(0, 12, false);
        assert!(tree.is_bst());
    }

    #[test]
    fn test_is_balanced() {
        let mut tree = Tree::with_root(10);
        assert!(tree.is_balanced());
        tree.add_node(0, 3, true);
        assert!(tree.is_balanced());
    }

    #[test]
    fn test_is_max_heap() {
        let mut tree = Tree::with_root(10);
        assert!(tree.is_max_heap());
        tree.add_node(0, 9, false);
        assert!(!tree.is_max_heap());
    }
}
