// TODO move to red-black tree after full binary tree implementation

struct BinaryTree<T> {
    data: T,
    left: Option<Box<BinaryTree<T>>>,
    right: Option<Box<BinaryTree<T>>>,
}

impl<T> BinaryTree<T>
where
    T: PartialEq,
    T: PartialOrd,
{
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

    pub fn insert(&mut self, data: T) {
        let mut node = self;

        loop {
            if node.data == data {
                return;
            } else if node.data < data {
                if let Some(ref mut right) = node.right {
                    node = right;
                } else {
                    node.right = Some(Box::new(BinaryTree::new(data)));
                    return;
                }
            } else {
                if let Some(ref mut left) = node.left {
                    node = left;
                } else {
                    node.left = Some(Box::new(BinaryTree::new(data)));
                    return;
                }
            }
        }
    }

    // pub fn remove(&mut self, data: T) -> Option<&BinaryTree<T>> {
    // }

    pub fn contains(&mut self, data: T) -> bool {
        let mut node = self;

        loop {
            if node.data == data {
                return true;
            } else if node.data < data {
                if let Some(ref mut right) = node.right {
                    node = right;
                } else {
                    return false;
                }
            } else {
                if let Some(ref mut left) = node.left {
                    node = left;
                } else {
                    return false;
                }
            }
        }
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

    #[test]
    fn contains() {
        let mut tree = BinaryTree::new(10)
            .left(
                BinaryTree::new(5)
                    .left(BinaryTree::new(2))
                    .right(BinaryTree::new(7)),
            )
            .right(BinaryTree::new(20).right(BinaryTree::new(25)));

        assert!(tree.contains(2));
        assert!(tree.contains(5));
        assert!(tree.contains(7));
        assert!(tree.contains(10));
        assert!(tree.contains(20));
        assert!(tree.contains(25));

        assert!(!tree.contains(100));
    }

    #[test]
    fn insert_and_remove() {
        let mut tree = BinaryTree::new(1);

        tree.insert(5);
        tree.insert(0);
        tree.insert(6);
        tree.insert(10);
        tree.insert(7);

        assert!(tree.contains(0));
        assert!(tree.contains(5));
        assert!(tree.contains(6));
        assert!(tree.contains(10));
        assert!(tree.contains(7));
    }
}
