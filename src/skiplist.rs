use rand::rngs::ThreadRng;
use rand_distr::{Distribution, Geometric};
use std::{cell::RefCell, rc::Rc};

// SkipList struct
pub struct SkipList<T: Ord> {
    head: Rc<Node<T>>,
    tail: Rc<Node<T>>,
    geometric: Geometric,
    rng: ThreadRng,
    height: usize,
}

// internal representation of a node in SkipList
pub struct Node<T: Ord> {
    val: Option<Rc<T>>,
    prev: RefCell<Vec<Rc<Node<T>>>>,
    next: RefCell<Vec<Rc<Node<T>>>>,
    is_head: bool,
    is_tail: bool,
}

impl<T: Ord> Node<T> {
    // create node with value
    fn new(x: T) -> Rc<Node<T>> {
        Rc::new(Node {
            val: Some(Rc::new(x)),
            prev: RefCell::new(Vec::new()),
            next: RefCell::new(Vec::new()),
            is_head: false,
            is_tail: false,
        })
    }

    // create head node
    fn new_head() -> Rc<Node<T>> {
        Rc::new(Node {
            val: None,
            prev: RefCell::new(Vec::new()),
            next: RefCell::new(Vec::new()),
            is_head: true,
            is_tail: false,
        })
    }

    // create tail node
    fn new_tail() -> Rc<Node<T>> {
        Rc::new(Node {
            val: None,
            prev: RefCell::new(Vec::new()),
            next: RefCell::new(Vec::new()),
            is_head: false,
            is_tail: true,
        })
    }

    // get direct predecessor
    pub fn pre(&self) -> Option<Rc<Node<T>>> {
        if self.is_head {
            return None;
        }

        let r = Rc::clone(&self.prev.borrow()[0]);
        if r.is_head {
            return None;
        }

        Some(r)
    }

    // get direct successor
    pub fn suc(&self) -> Option<Rc<Node<T>>> {
        if self.is_tail {
            return None;
        }

        let r = Rc::clone(&self.next.borrow()[0]);
        if r.is_tail {
            return None;
        }

        Some(r)
    }

    // get value of node
    pub fn val(&self) -> Rc<T> {
        if let Some(x) = &self.val {
            return Rc::clone(x);
        }
        panic!("No value in Node!"); // should never happen has there is no pub access to head / tail
    }
}

impl<T: Ord> SkipList<T> {
    // create new empty SkipList
    pub fn new() -> SkipList<T> {
        // new head / tail
        let head: Rc<Node<T>> = Node::new_head();
        let tail: Rc<Node<T>> = Node::new_tail();

        // add pointers between head and tail
        head.next.borrow_mut().push(Rc::clone(&tail));
        tail.prev.borrow_mut().push(Rc::clone(&head));

        // create SkipList
        SkipList {
            head,
            tail,
            geometric: Geometric::new(0.5).unwrap(),
            rng: rand::thread_rng(),
            height: 1,
        }
    }

    // search for largest element <= val in SkipList
    // returns list of pointers to nodes where the search path descended list level
    // only used for internal operations
    fn search_history(&self, val: &T) -> Vec<Rc<Node<T>>> {
        let mut history: Vec<Rc<Node<T>>> = Vec::new();

        // current node of search
        let mut cur = Rc::clone(&self.head);
        // current list index / height of search
        let mut list_idx = isize::try_from(self.height - 1).unwrap();

        while list_idx >= 0 {
            // get next node of current node
            let next: Rc<Node<T>> =
                Rc::clone(&cur.next.borrow()[usize::try_from(list_idx).unwrap()]);

            // check if next is tail
            if next.is_tail {
                // descend
                history.push(Rc::clone(&cur));
                list_idx -= 1;
                continue;
            }

            // get value of next (should always go into if)
            if let Some(x) = &next.val {
                // check if next value is larger
                if **x > *val {
                    // descend
                    history.push(Rc::clone(&cur));
                    list_idx -= 1;
                    continue;
                }
            }

            // go to next node
            cur = next;
        }

        // reverse history to increase interpretability
        history.reverse();
        history
    }

    // find value and return it if exists
    pub fn find(&self, val: &T) -> Option<Rc<T>> {
        // get history to largest element <= val
        let history = self.search_history(val);

        // get element on lowest level
        let first = &history[0];

        // check if it is head node
        if first.is_head {
            return None;
        }

        // check if value in node equals searched value
        if let Some(x) = &first.val {
            if **x == *val {
                return Some(Rc::clone(x));
            }
        }

        None
    }

    // insert value into SkipList (even if it already exists)
    pub fn insert(&mut self, val: T) {
        self.insert_ret_node(val);
    }

    // insert value into SkipList and return created node (internal method)
    fn insert_ret_node(&mut self, val: T) -> Rc<Node<T>> {
        let history = self.search_history(&val);

        // geometric distributed random variable for height generation
        let mut node_height: usize = self.geometric.sample(&mut self.rng).try_into().unwrap();
        node_height += 1; // add 1 as we require minimal height of 1

        // create node
        let node = Node::new(val);

        for i in 0..node_height {
            // check if height already exists

            if i < self.height {
                // height exists

                // predecessor and successor node on level i
                let prev = Rc::clone(&history[i]);
                let next = Rc::clone(&prev.next.borrow_mut()[i]);

                // let predecessor and successor point on new node
                prev.next.borrow_mut()[i] = Rc::clone(&node);
                next.prev.borrow_mut()[i] = Rc::clone(&node);

                // let new node point on predecessor and successor
                node.prev.borrow_mut().push(Rc::clone(&prev));
                node.next.borrow_mut().push(Rc::clone(&next));
            } else {
                // height does not exist yet

                // add new level that points onto new node
                self.head.next.borrow_mut().push(Rc::clone(&node));
                self.tail.prev.borrow_mut().push(Rc::clone(&node));

                // let new node point onto head and tail
                node.prev.borrow_mut().push(Rc::clone(&self.head));
                node.next.borrow_mut().push(Rc::clone(&self.tail));

                // increase height
                self.height += 1;
            }
        }

        node
    }

    // delete an element of SkipList and return it if exists
    pub fn delete(&mut self, val: &T) -> Option<Rc<T>> {
        // get history to largest element <= val
        let history = self.search_history(val);

        // get node on lowest level
        let node = &history[0];

        // check if value inside node is unequal the given value
        if let Some(x) = &node.val {
            if **x != *val {
                return None;
            }
        } else {
            return None;
        }

        // remove node
        self.remove(node);

        // get value inside (should always go into if)
        if let Some(x) = &node.val {
            // return pointer to value inside
            return Some(Rc::clone(x));
        }
        None
    }

    // remove a node from the SkipList
    pub fn remove(&mut self, node: &Rc<Node<T>>) {
        // get predecessor and successor list as mutable
        let prev = node.prev.borrow_mut();
        let next = node.next.borrow_mut();

        // height of node
        let node_height = prev.len();

        // go through height reversed
        for i in (0..node_height).rev() {
            // check if predecessor is head and successor is tail
            if prev[i].is_head && next[i].is_tail {
                // remove level and decrease height
                prev[i].next.borrow_mut().pop();
                next[i].prev.borrow_mut().pop();
                self.height -= 1;
            } else {
                // let predecessor and successor repoint
                prev[i].next.borrow_mut()[i] = Rc::clone(&next[i]);
                next[i].prev.borrow_mut()[i] = Rc::clone(&prev[i]);
            }
        }
    }

    // create an iterator to iterate through SkipList (lowest level)
    pub fn iter(&self) -> NodeIterator<T> {
        NodeIterator {
            cur: Rc::clone(&self.head.next.borrow()[0]),
        }
    }

    // insert element or replace it if it exists already, if it replaces it, it is also returned
    pub fn insert_or_replace(&mut self, val: T) -> Option<Rc<T>> {
        // insert node
        let node = self.insert_ret_node(val);

        // get direct prev
        let prev = Rc::clone(&node.prev.borrow()[0]);

        // see if they have both the same value
        if node.val == prev.val {
            // remove prev
            self.remove(&prev);

            // should go into if
            if let Some(x) = &prev.val {
                return Some(Rc::clone(x));
            }
        }

        None
    }

    // find node that matches value and return it if it exists
    pub fn find_node(&self, val: &T) -> Option<Rc<Node<T>>> {
        // get history to largest element <= val
        let history = self.search_history(val);

        // get element on lowest level
        let first = &history[0];

        // check if it is head node
        if first.is_head {
            return None;
        }

        // check if value in node equals searched value
        if let Some(x) = &first.val {
            if **x == *val {
                return Some(Rc::clone(first));
            }
        }

        None
    }
}

// iterator struct for going through list of nodes (on lowest level)
pub struct NodeIterator<T>
where
    T: Ord,
{
    cur: Rc<Node<T>>,
}

impl<T: Ord> IntoIterator for SkipList<T> {
    type Item = Rc<T>;
    type IntoIter = NodeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        return NodeIterator { cur: self.head };
    }
}

impl<T: Ord> Iterator for NodeIterator<T> {
    type Item = Rc<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = &self.cur.val {
            let val = Rc::clone(x);
            let next = Rc::clone(&self.cur.next.borrow()[0]);
            self.cur = next;
            return Some(val);
        }
        return None;
    }
}
