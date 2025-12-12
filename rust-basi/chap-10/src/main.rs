fn main() {
    println!("Hello, world!");

    let int_sum = sum(5, 10);
    println!("Sum of integers: {}", int_sum);

    let mut v = vec![1, 2, 3];
    for _ in 0..v.len() + 1 {
        if let Some(value) = v.pop() {
            println!("Popped value: {}", value);
        } else {
            println!("Vector is empty!");
        }
    }
}

// A generic function that sums two values of the same type.
// The type T must implement the Add trait and be Copyable.
fn sum<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T> + Copy,
{
    a + b
}
