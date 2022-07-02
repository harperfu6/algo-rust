use std::mem;

struct Node<T> {
    keys: Vec<T>,
    children: Vec<Node<T>>,
}
// Why to need a different Struct for props...
// Check - http://smallcultfollowing.com/babysteps/blog/2018/11/01/after-nll-interprocedural-conflicts/#fnref:improvement
struct BTreeProps {
    degree: usize,
    max_keys: usize,
    mid_key_index: usize,
}

struct BTree<T>
where
    T: Ord,
{
    root: Node<T>,
    props: BTreeProps,
}

impl<T> Node<T> {
    fn new(degree: usize, _keys: Option<Vec<T>>, _children: Option<Vec<Node<T>>>) -> Self {
        Node {
            keys: match _keys {
                None => Vec::with_capacity(degree - 1),
                Some(_keys) => _keys,
            },
            children: match _children {
                None => Vec::with_capacity(degree - 1),
                Some(_children) => _children,
            },
        }
    }
}

impl BTreeProps {
    fn new(degree: usize) -> Self {
        BTreeProps {
            degree,
            max_keys: degree - 1,
            mid_key_index: (degree - 1) / 2,
        }
    }

    fn is_maxed_out<T: Ord>(&self, node: &Node<T>) -> bool {
        node.keys.len() == self.max_keys
    }
}

impl<T> BTree<T>
where
    T: Ord,
{
    fn new(branch_factor: usize) -> Self {
        let degree = branch_factor * 2;
        BTree {
            root: Node::new(degree, None, None),
            props: BTreeProps::new(degree),
        }
    }

    fn insert(&mut self, key: T) {
        if self.props.is_maxed_out(&self.root) {
            // Create an empty root and split the old root
            let mut new_root = Node::new(self.props.degree, None, None);
            mem::swap(&mut new_root, &mut self.root);
            self.root.children.insert(0, new_root);
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_search() {
        assert_eq!(1, 1);
    }
}
