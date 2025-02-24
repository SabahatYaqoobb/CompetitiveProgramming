fn woodcutters(trees: &mut [(i32, i32)]) -> i32 {
    let n = trees.len();
    let mut cutted = 2; // First tree always cuts left, last one cuts right.

    for i in 1..n-1 {
        let (x, h) = trees[i];
        
        if trees[i-1].0 < x - h { // Can cut left
            cutted += 1;
        } else if x + h < trees[i+1].0 { // Can cut right
            cutted += 1;
            trees[i].0 = x + h; // Move the tree position after cutting
        }
    }

    cutted
}

fn main() {
    let mut test1 = [(1, 2), (2, 1), (5, 10), (10, 9), (19, 1)];
    assert_eq!(woodcutters(&mut test1), 3);

    let mut test2 = [(1, 2), (2, 1), (5, 10), (10, 9), (20, 1)];
    assert_eq!(woodcutters(&mut test2), 4);
}
