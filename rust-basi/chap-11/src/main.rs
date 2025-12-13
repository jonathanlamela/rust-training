fn main() {
    // This value is stored on the heap
    let box1 = Box::new(5);

    // This value is stored on the stack
    let stack1 = 15;

    println!("Box value: {}", box1);
    println!("Stack value: {}", stack1);
}
