use std::mem;

#[derive(Debug)]
pub struct List {
    head: Link,
}
#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}
#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        println!("{:?}", self.head);

        self.head = Link::More(new_node);
        println!("{:?}", self.head);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

pub fn run(){
    let mut list  = List::new();
    list.push(1);
    list.push(2);
    
    println!("{:?}", list );
    list.pop();
    println!("{:?}", list );
}