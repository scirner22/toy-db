// TODO move to red-black tree after full binary tree implementation

struct BinaryTree<T> {
    data: T,
    left: Option<Box<BinaryTree<T>>>,
    right: Option<Box<BinaryTree<T>>>,
}

impl<T> BinaryTree<T> {
    fn new(data: T) -> Self {
        BinaryTree {
            data,
            left: None,
            right: None,
        }
    }

    fn left(mut self, node: BinaryTree<T>) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    fn right(mut self, node: BinaryTree<T>) -> Self {
        self.right = Some(Box::new(node));
        self
    }

    pub fn insert(&self, _data: T) -> Self {
        todo!("implement")
    }

    pub fn remove(&self, _data: T) -> Self {
        todo!("implement")
    }

    pub fn contains(&self, _data: T) -> bool {
        todo!("implement")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_node() {
        let tree = BinaryTree::new(1);

        assert_eq!(tree.data, 1);
    }

    #[test]
    fn complex_static_tree() {
        let tree = BinaryTree::new(10)
            .left(
                BinaryTree::new(5)
                    .left(BinaryTree::new(2))
                    .right(BinaryTree::new(7)),
            )
            .right(BinaryTree::new(20).right(BinaryTree::new(25)));
        let left_tree = tree.left.unwrap();
        let right_tree = tree.right.unwrap();

        assert_eq!(left_tree.left.unwrap().data, 2);
        assert_eq!(left_tree.data, 5);
        assert_eq!(left_tree.right.unwrap().data, 7);
        assert_eq!(tree.data, 10);
        assert!(right_tree.left.is_none());
        assert_eq!(right_tree.data, 20);
        assert_eq!(right_tree.right.unwrap().data, 25);
    }
}
