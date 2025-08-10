use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::{Rc, Weak};

pub struct Node<T: std::fmt::Display + std::cmp::Eq> {
    parent: Weak<RefCell<Self>>,
    size: usize,
    this: T,
}

/// Given some number of components, create a set of nodes.
pub fn make_set<T>(components: Vec<T>) -> Vec<Rc<RefCell<Node<T>>>>
where
    T: std::fmt::Display + std::cmp::Eq,
{
    let mut nodes = Vec::with_capacity(components.len());
    for element in components {
        let ptr = Rc::new(RefCell::new(Node {
            parent: Weak::new(),
            size: 1,
            this: element,
        }));

        ptr.borrow_mut().parent = Rc::downgrade(&ptr);
        nodes.push(ptr);
    }

    nodes
}

/// Given a node, find the representative of the group the node belongs to.
pub fn find<T>(node: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>>
where
    T: std::fmt::Display + std::cmp::Eq,
{
    let mut representative = node.clone();
    loop {
        let parent_rc = representative
            .borrow()
            .parent
            .upgrade()
            .expect("Parent pointers should always be valid.");

        // If the parent's contents match the child's contents, we've reached the root.
        if parent_rc.borrow().this == representative.borrow().this {
            break;
        };

        representative = parent_rc;
    }

    // path compresssion
    let mut current = node.clone();
    loop {
        if current.borrow().parent.upgrade().unwrap().borrow().this == current.borrow().this {
            break;
        }

        let prev = current.clone();
        let parent_rc = current
            .borrow()
            .parent
            .upgrade()
            .expect("Parent pointers should always be valid.");
        current = parent_rc;

        prev.borrow_mut().parent = Rc::downgrade(&representative);
    }

    representative
}

/// Merge two nodes into the same set.
pub fn union<T>(node_a: Rc<RefCell<Node<T>>>, node_b: Rc<RefCell<Node<T>>>)
where
    T: std::fmt::Display + std::cmp::Eq,
{
    if node_a.borrow().size < node_b.borrow().size {
        node_b.borrow_mut().size += node_a.borrow().size;
        let parent = find(node_b.clone());
        node_a.borrow_mut().parent = Rc::downgrade(&parent);
    } else {
        node_a.borrow_mut().size += node_b.borrow().size;
        node_b.borrow_mut().parent = node_a.borrow().parent.clone();
    }
}

impl<T: std::fmt::Display + std::cmp::Eq> Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} <- {}",
            self.parent
                .upgrade()
                .expect("Parents should always be valid.")
                .borrow()
                .this,
            self.this
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_i32_set() {
        let set = make_set(vec![1, 2, 3, 4]);

        // mark set[1] as being connected to set[0]
        union(set[0].clone(), set[1].clone());
        assert_eq!(1, find(set[0].clone()).borrow().this);
        assert_eq!(1, find(set[1].clone()).borrow().this);

        union(set[0].clone(), set[2].clone());
        assert_eq!(1, find(set[2].clone()).borrow().this);

        // set[3] should still be an outlier here
        assert_eq!(4, find(set[3].clone()).borrow().this);
    }

    // Testcases from the book "Algorithm Design and Applications" by Goodrich/Tamassia
    #[test]
    fn ada_example_r7p1() {
        let set = make_set(vec!["A", "B", "C", "D", "E", "F", "G"]);

        // friendship ties
        // [(A,B), (B,C), (C,A), (D,E), (F,G)]
        union(set[0].clone(), set[1].clone());
        union(set[1].clone(), set[2].clone());
        union(set[2].clone(), set[1].clone());
        union(set[3].clone(), set[4].clone());
        union(set[5].clone(), set[6].clone());

        // group 1
        assert_eq!("A", find(set[0].clone()).borrow().this);
        assert_eq!("A", find(set[1].clone()).borrow().this);
        assert_eq!("A", find(set[2].clone()).borrow().this);

        // group 2
        assert_eq!("D", find(set[3].clone()).borrow().this);
        assert_eq!("D", find(set[4].clone()).borrow().this);

        // group 3
        assert_eq!("F", find(set[5].clone()).borrow().this);
        assert_eq!("F", find(set[6].clone()).borrow().this);
    }
}
