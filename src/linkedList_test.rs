use std::mem;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct List {
    head: Link,
}
#[derive(Debug)]
#[derive(PartialEq)]
enum Link {
    Empty,
    More(Box<Node>),
}
#[derive(Debug)]
#[derive(PartialEq)]
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
        println!("{:?}", new_node);


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

    pub fn push_end(&mut self, elem: i32) {
        let new_node = Box::new(Node{
            elem: elem,
            next: Link::Empty,
        });
        let mut last =  &mut self.head;
        println!("{:?}", *last );
        while *last != Link::Empty {
            match last{
                Link::More(node)=>{
                    last = &mut (node.next);
                    println!(" value of last : {:?}", last );
                },
                Link::Empty =>println!("none") 
            }
            
        }
        mem::replace(last, Link::More(new_node));
        println!(" value of last : {:?}", last );
        

    }
}

pub fn run(){
    let mut list  = List::new();
    list.push(1);
    list.push(2);
    
    println!("{:?}", list );
    list.pop();
    println!("{:?}", list );
    list.push_end(5);
    println!("{:?}", list );
    list.push_end(34);
    println!("{:?}", list );

}