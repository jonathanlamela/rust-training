fn main() {
    let my_first_range = 0..8;
    for number in my_first_range {
        println!("The number is: {}", number);
    }

    let my_slice = [34, 50, 25, 100, 65];
    let min = min_value(&my_slice);
    println!("The minimum value is: {}", min);
}

fn min_value(arr: &[i32]) -> i32 {
    let mut min = arr[0];
    for &value in arr.iter() {
        if value < min {
            min = value;
        }
    }
    min
}
