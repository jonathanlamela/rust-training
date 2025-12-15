fn main() {
    let mut my_array = [10, 30, 20, 25, 50];

    my_array.sort_by(|a, b| a.cmp(b));

    println!("{:?}", my_array);
}
