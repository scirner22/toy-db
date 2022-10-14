use std::cmp::Ordering;

#[derive(Debug)]
enum Color {
    Red,
    Black,
}

#[derive(Debug)]
enum BinaryTree<T> {
    Node {
        data: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
        color: Color,
    },
    Sentinel,
}

impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        BinaryTree::Sentinel
    }
}

impl<T> BinaryTree<T>
where
    T: Ord,
{
    fn new(data: T) -> Self {
        BinaryTree::Node {
            data,
            left: Default::default(),
            right: Default::default(),
            color: Color::Red,
        }
    }

    fn color(&self) -> &Color {
        match self {
            BinaryTree::Node {
                data: _,
                left: _,
                right: _,
                color,
            } => color,
            BinaryTree::Sentinel => &Color::Black,
        }
    }

    fn left(mut self, node: BinaryTree<T>) -> Self {
        match self {
            BinaryTree::Node {
                data: _,
                ref mut left,
                right: _,
                color: _,
            } => *left = Box::new(node),
            BinaryTree::Sentinel => self = node,
        }

        self
    }

    fn right(mut self, node: BinaryTree<T>) -> Self {
        match self {
            BinaryTree::Node {
                data: _,
                left: _,
                ref mut right,
                color: _,
            } => *right = Box::new(node),
            BinaryTree::Sentinel => self = node,
        }

        self
    }

    fn data(&self) -> Option<&T> {
        match self {
            BinaryTree::Node {
                data,
                left: _,
                right: _,
                color: _,
            } => Some(data),
            BinaryTree::Sentinel => None,
        }
    }

    fn left_tree(&self) -> Option<&BinaryTree<T>> {
        match self {
            BinaryTree::Node {
                data: _,
                left,
                right: _,
                color: _,
            } => Some(left),
            BinaryTree::Sentinel => None,
        }
    }

    fn right_tree(&self) -> Option<&BinaryTree<T>> {
        match self {
            BinaryTree::Node {
                data: _,
                left: _,
                right,
                color: _,
            } => Some(right),
            BinaryTree::Sentinel => None,
        }
    }

    pub fn insert(&mut self, new: T) {
        let mut node = self;

        loop {
            match node {
                BinaryTree::Node {
                    ref data,
                    ref mut left,
                    ref mut right,
                    color: _,
                } => match new.cmp(data) {
                    Ordering::Less => node = left,
                    Ordering::Greater => node = right,
                    Ordering::Equal => return,
                },
                BinaryTree::Sentinel => {
                    *node = BinaryTree::new(new);
                    return;
                }
            }
        }
    }

    pub fn contains(&self, needle: T) -> bool {
        let mut node = self;

        loop {
            match node {
                BinaryTree::Node {
                    data,
                    left,
                    right,
                    color: _,
                } => match needle.cmp(data) {
                    Ordering::Less => node = left,
                    Ordering::Greater => node = right,
                    Ordering::Equal => return true,
                },
                BinaryTree::Sentinel => {
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

        match tree {
            BinaryTree::Node {
                data,
                left,
                right,
                color: _,
            } => assert_eq!(data, 1),
            BinaryTree::Sentinel => unreachable!("{:#?}", &tree),
        }
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
        let left_tree = match &tree {
            BinaryTree::Node {
                data: _,
                left,
                right: _,
                color: _,
            } => left,
            BinaryTree::Sentinel => unreachable!("{:#?}", tree),
        };
        let right_tree = match &tree {
            BinaryTree::Node {
                data: _,
                left: _,
                right,
                color: _,
            } => right,
            BinaryTree::Sentinel => unreachable!("{:#?}", tree),
        };

        assert_eq!(left_tree.left_tree().unwrap().data().unwrap().to_owned(), 2);
        assert_eq!(left_tree.data().unwrap().to_owned(), 5);
        assert_eq!(
            left_tree.right_tree().unwrap().data().unwrap().to_owned(),
            7
        );
        assert_eq!(tree.data().unwrap().to_owned(), 10);
        assert!(right_tree.left_tree().unwrap().data().is_none());
        assert_eq!(right_tree.data().unwrap().to_owned(), 20);
        assert_eq!(
            right_tree.right_tree().unwrap().data().unwrap().to_owned(),
            25
        );
    }

    #[test]
    fn contains() {
        let tree = BinaryTree::new(10)
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
