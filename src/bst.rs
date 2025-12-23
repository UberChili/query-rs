use std::fmt::Debug;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Node {
    data: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(data: i32) -> Node {
        Node {
            data,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct Tree {
    pub root: Option<Box<Node>>,
    pub elems: i32,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            root: None,
            elems: 0,
        }
    }
}
