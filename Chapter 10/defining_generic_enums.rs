
#[derive(Debug)]
enum ListNode<Value> {
    Node(Value),
    Empty
}

fn main() {

    //generic enums in Rust have specific syntax

    let non_empty_node = ListNode::Node::<i64>(123);
    
    println!("Picking from a ListNode {:?}", non_empty_node)
}