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
    ( val: $val:expr, left: $left:expr, right: $right:expr $(,) ) => {
        BinaryTree::Node {
            val: $val,
            left: Box::new($left),
            right: Box::new(&right),
        }
    };

    ( val: $val:expr, right: $right:expr $(,) ) => {
        bin_tree! {
            val: $val,
            left: bin_tree!(),
            right: $right,
        }
    };

    ( val: $val:expr, left: $left:expr $(,) ) => {
        bin_tree! {
            val: $val,
            left: $left,
            right: bin_tree(),
        }
    };

    ( val: $val:expr, $(,) ) => {
        bin_tree! (val: $val, left: bin_tree!(), right: bin_tree!(),)
    };

    () => {
        BinaryTree::Nil
    };
impl<T> BinaryTree<T> {
    /// self の Node または Nil を、to(Self)に置き換える。
    /// to は self に組み込まれる形で move される。
    pub fn replace(&mut self, to: Self) {
        *self = to;
    }
}
