use simple_linked_list::SimpleLinkedList;

pub fn main() -> () {
    let mut list: SimpleLinkedList<i32> = SimpleLinkedList::new();

    for i in 1..4 {
        list.push(i);
    }

    println!("List head: {:?}", list.peek());

    let vec: Vec<i32> = list.into();
    println!("Linked list as vector: {:?}", vec);
}
