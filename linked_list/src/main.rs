#[derive(Debug)]
enum LinkedList<T> {
    Empty,
    Node { value: T, next: Box<LinkedList<T>> },
}

fn main() {
    let list = LinkedList::Node {
        value: 100,
        next: Box::new(LinkedList::Node {
            value: 102,
            next: Box::new(LinkedList::Node {
                value: 104,
                next: Box::new(LinkedList::Empty),
            }),
        }),
    };
    println!("list_end: {:#?}", list);

    let im_with_you = LinkedList::Node {
        value: String::from("I'm with you"),
        next: Box::new(LinkedList::Empty),
    };

    let sk8er_boy = LinkedList::Node {
        value: (String::from("Sk8er Boy")),
        next: Box::new(im_with_you),
    };

    let complicated = LinkedList::Node {
        value: (String::from("complicated")),
        next: Box::new(sk8er_boy),
    };
    println!("complicated: {:#?}", complicated);
}
