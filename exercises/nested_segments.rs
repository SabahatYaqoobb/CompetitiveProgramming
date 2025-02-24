#[derive(Debug)]
struct Node {
    sum: i32,
    left_edge: usize,
    right_edge: usize,
    left_child: Option<Box<Node>>,
    right_child: Option<Box<Node>>,
}

impl Node {
    fn new(sum: i32, left_edge: usize, right_edge: usize) -> Self {
        Self {
            sum,
            left_edge,
            right_edge,
            left_child: None,
            right_child: None,
        }
    }
}

struct SegmentTree {
    root: Node,
}

impl SegmentTree {
    pub fn new(a: &[i32]) -> Self {
        let n = a.len();
        let mut root = Node::new(0, 0, n - 1);
        Self::build(&mut root, a, 0, n - 1);
        SegmentTree { root }
    }

    fn build(curr_node: &mut Node, a: &[i32], l: usize, r: usize) {
        if l == r {
            curr_node.sum = a[l];
        } else {
            let mid = (l + r) / 2;
            let mut left_child = Node::new(0, l, mid);
            let mut right_child = Node::new(0, mid + 1, r);
            Self::build(&mut left_child, a, l, mid);
            Self::build(&mut right_child, a, mid + 1, r);

            curr_node.sum = left_child.sum + right_child.sum;
            curr_node.left_child = Some(Box::new(left_child));
            curr_node.right_child = Some(Box::new(right_child));
        }
    }

    pub fn range_sum(&self, l: usize, r: usize) -> i32 {
        Self::range_sum_rec(&self.root, l, r)
    }

    fn range_sum_rec(node: &Node, l: usize, r: usize) -> i32 {
        if r < node.left_edge || l > node.right_edge {
            return 0;
        }
        if l <= node.left_edge && r >= node.right_edge {
            return node.sum;
        }

        let left_sum = node.left_child.as_ref().map_or(0, |left| Self::range_sum_rec(left, l, r));
        let right_sum = node.right_child.as_ref().map_or(0, |right| Self::range_sum_rec(right, l, r));
        left_sum + right_sum
    }

    pub fn increment_by_one(&mut self, i: usize) {
        let val = self.range_sum(i, i) + 1;
        Self::update_rec(&mut self.root, i, i, val);
    }

    fn update_rec(curr_node: &mut Node, l: usize, r: usize, t: i32) {
        if curr_node.right_edge < l || curr_node.left_edge > r {
            return;
        }

        if curr_node.left_edge >= l && curr_node.right_edge <= r {
            curr_node.sum = t;
            if let Some(left_child) = &mut curr_node.left_child {
                Self::update_rec(left_child, l, r, t);
            }
            if let Some(right_child) = &mut curr_node.right_child {
                Self::update_rec(right_child, l, r, t);
            }
            return;
        }

        if let Some(left_child) = &mut curr_node.left_child {
            Self::update_rec(left_child, l, r, t);
        }
        if let Some(right_child) = &mut curr_node.right_child {
            Self::update_rec(right_child, l, r, t);
        }

        curr_node.sum = curr_node.left_child.as_ref().map_or(0, |left| left.sum) +
                        curr_node.right_child.as_ref().map_or(0, |right| right.sum);
    }

    pub fn print_tree(&self) {
        Self::print_tree_rec(&self.root, 0);
    }

    fn print_tree_rec(node: &Node, depth: usize) {
        let indentation = "  ".repeat(depth);
        println!("{}Node: sum: {}, Range: [{}, {}]", indentation, node.sum, node.left_edge, node.right_edge);

        if let Some(left_child) = &node.left_child {
            Self::print_tree_rec(left_child, depth + 1);
        }
        if let Some(right_child) = &node.right_child {
            Self::print_tree_rec(right_child, depth + 1);
        }
    }
}

fn segment_tree_nested_segments(input_segments: &[(i32, i32)]) -> Vec<(usize, i32)> {
    let n = input_segments.iter().max_by_key(|&x| x.1).unwrap();
    let a = vec![0; n.1 as usize];
    let mut seg_tree = SegmentTree::new(&a);

    let mut res = Vec::new();
    let mut axis: Vec<(i32, usize, bool)> = input_segments.iter().enumerate().flat_map(|(i, &(l, r))| vec![(l, i, true), (r, i, false)]).collect();
    axis.sort_by(|a, b| a.0.cmp(&b.0));

    for i in axis.iter() {
        if !i.2 {
            let index = i.1;
            let l = input_segments[index].0 as usize;
            let r = i.0 as usize;
            let nested_segments = seg_tree.range_sum(l, r);
            res.push((i.1, nested_segments));
            seg_tree.increment_by_one(input_segments[i.1].0 as usize);
        }
    }

    res.sort_by(|a, b| a.0.cmp(&b.0));
    res
}

fn main() {
    let input_segments1 = vec![(1, 8), (2, 3), (4, 7), (5, 6)];
    let sol1 = segment_tree_nested_segments(&input_segments1);
    for &(_, nested) in &sol1 {
        println!("{}", nested);
    }
}
