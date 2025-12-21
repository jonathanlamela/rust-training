fn main() {
    let num1 = 10;
    let num2 = num1;
    println!("num1: {}, num2: {}", num1, num2);

    let vec1 = vec![1, 2, 3];
    #[allow(unused_variables)]
    let vec2 = vec1; // vec1 is moved to vec2 here
    // println!("vec1: {:?}", vec1); // This line would cause a compile-time

    let vec3 = vec![4, 5, 6];
    let vec4 = vec3.clone(); // vec3 is cloned to vec4
    println!("vec3: {:?}, vec4: {:?}", vec3, vec4);
}
