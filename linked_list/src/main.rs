#[derive(Debug)]
enum LinkedListUsingReference<'a, T> {
    Empty,
    Node {
        value: T,
        next: &'a LinkedListUsingReference<'a, T>,
    },
}

#[derive(Debug)]
enum LinkedList<T> {
    Empty,
    Node { value: T, next: Box<LinkedList<T>> },
}

fn main() {
    // Using reference.
    let second_node_ref_list = LinkedListUsingReference::Node {
        value: 2,
        next: &LinkedListUsingReference::Empty,
    };

    let first_node_ref_list = LinkedListUsingReference::Node {
        value: 1,
        next: &second_node_ref_list,
    };

    //drop(first_node_ref_list);
    drop(second_node_ref_list);

    //println!("\nSecond node ref list: {:#?}\n", second_node_ref_list);
    println!("\nFirst node ref list: {:#?}\n", first_node_ref_list);

    // Using integer.
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
    println!("list_end: {:#?}\n", list);

    // Using string.
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
