#![allow(dead_code)]

use std::cmp::Ordering;

struct BinarySearchTree<T>
where
    T: Ord,
{
    value: Option<T>,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree {
            value: None,
            left: None,
            right: None,
        }
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        BinarySearchTreeIter::new(self)
    }

    fn search(&self, value: &T) -> bool {
        match &self.value {
            None => false,
            Some(key) => match key.cmp(value) {
                Ordering::Equal => true,
                Ordering::Greater => match &self.left {
                    None => false,
                    Some(node) => node.search(value),
                },
                Ordering::Less => match &self.right {
                    None => false,
                    Some(node) => node.search(value),
                },
            },
        }
    }

    fn insert(&mut self, value: T) {
        match &self.value {
            None => {
                self.value = Some(value);
            }
            Some(key) => {
                let target_node = if value < *key {
                    &mut self.left
                } else {
                    &mut self.right
                };
                match target_node {
                    None => {
                        let mut node = BinarySearchTree::new();
                        node.insert(value);
                        *target_node = Some(Box::new(node));
                    }
                    Some(node) => {
                        node.insert(value);
                    }
                }
            }
        }
    }

    fn minimum(&self) -> Option<&T> {
        match &self.left {
            None => self.value.as_ref(),
            Some(node) => node.minimum(),
        }
    }

    fn maximum(&self) -> Option<&T> {
        match &self.right {
            None => self.value.as_ref(),
            Some(node) => node.maximum(),
        }
    }
}

struct BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    // 最後尾がその時点での最小ノードになるような管理をする
    stack: Vec<&'a BinarySearchTree<T>>, // Iterよりも長生きする必要があるためライフタイム指定子をつける
}

impl<'a, T> BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    fn new(tree: &BinarySearchTree<T>) -> BinarySearchTreeIter<T> {
        let mut iter = BinarySearchTreeIter { stack: vec![tree] };
        iter.stack_push_left();
        iter
    }

    // とりあえず左側の親ノードをベクトルの最後尾に追加していく（左側の各右側も無視）
    fn stack_push_left(&mut self) {
        while let Some(child) = &self.stack.last().unwrap().left {
            self.stack.push(child);
        }
    }
}

impl<'a, T> Iterator for BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.stack.is_empty() {
            None
        } else {
            let node = self.stack.pop().unwrap(); // 最後尾からデータを取る
                                                  // その過程で右側にデータがあれば取る
            if node.right.is_some() {
                self.stack.push(node.right.as_ref().unwrap());
                // 今追加した右側のノードについて左側のノードをベクトルの最後尾にいれる
                self.stack_push_left();
            }

            node.value.as_ref()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BinarySearchTree;

    fn tree() -> BinarySearchTree<&'static str> {
        let mut tree = BinarySearchTree::new();
        tree.insert("hello there");
        tree.insert("general kenobi");
        tree.insert("you are a bold one");
        tree.insert("kill him");
        tree.insert("back away...I will deal with this jedi slime myself");
        tree.insert("your move");
        tree.insert("you fool");
        tree
    }

    #[test]
    fn test_search() {
        let tree = tree();
        assert!(tree.search(&"hello there"));
        assert!(tree.search(&"you are a bold one"));
        assert!(tree.search(&"general kenobi"));
        assert!(tree.search(&"you fool"));
        assert!(tree.search(&"kill him"));
        assert!(
            !tree.search(&"but i was going to tosche station to pick up some power converters",)
        );
        assert!(!tree.search(&"only a sith deals in absolutes"));
        assert!(!tree.search(&"you underestimate my power"));
    }

    #[test]
    fn test_maximum_and_minimum() {
        let tree = tree();
        assert_eq!(*tree.maximum().unwrap(), "your move");
        assert_eq!(
            *tree.minimum().unwrap(),
            "back away...I will deal with this jedi slime myself"
        );

        let mut tree2: BinarySearchTree<i32> = BinarySearchTree::new();
        assert!(tree2.maximum().is_none());
        assert!(tree2.minimum().is_none());
        tree2.insert(0);
        assert_eq!(*tree2.minimum().unwrap(), 0);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(-5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 5);
    }

    #[test]
    fn test_iterator() {
        let tree = tree();
        let mut iter = tree.iter();
        assert_eq!(
            iter.next().unwrap(),
            &"back away...I will deal with this jedi slime myself"
        );
        assert_eq!(iter.next().unwrap(), &"general kenobi");
        assert_eq!(iter.next().unwrap(), &"hello there");
        assert_eq!(iter.next().unwrap(), &"kill him");
        assert_eq!(iter.next().unwrap(), &"you are a bold one");
        assert_eq!(iter.next().unwrap(), &"you fool");
        assert_eq!(iter.next().unwrap(), &"your move");
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
