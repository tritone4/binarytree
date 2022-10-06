#[derive(Debug, PartialEq, Eq)]
pub enum BinaryTree {
    Nil,
    Node {
        val: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}

#[macro_export]
macro_rules! bin_tree {
    (val: $val:expr, left: $lrft:expr, right: $right:expr $(,)) => {
        BinaryTree::Node {
            val: $val,
            left: Box::new($left),
            right: Box::new(&right),
        }
    };
    
}