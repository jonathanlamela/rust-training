fn main() {
    // ...existing code...

    // Declare an immutable array of 5 i32 integers with fixed size
    // Arrays have compile-time known size and are stored on the stack
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    // Print the array using debug formatting ({:?})
    println!("Array: {:?}", my_array);

    // Create an immutable vector (dynamic array) on the heap
    // The vec! macro initializes a vector with the given elements
    let my_vector: Vec<i32> = vec![1, 2, 3, 4, 5];
    // Print the vector using debug formatting
    println!("Vector: {:?}", my_vector);

    // Create a MUTABLE vector (note the 'mut' keyword)
    // This allows us to modify the vector after creation
    let mut my_mutable_vector: Vec<i32> = vec![1, 2, 3, 4];
    // Add element 6 to the end of the vector
    // push() increases the vector's length dynamically
    my_mutable_vector.push(6);
    // Print the modified vector - now contains [1, 2, 3, 4, 6]
    println!("Mutable Vector after push: {:?}", my_mutable_vector);

    // Create two separate fixed-size arrays
    let array_1: [i32; 3] = [1, 2, 3];
    let array_2: [i32; 3] = [4, 5, 6];

    // Create an array containing REFERENCES to slices of the two arrays
    // This is NOT true concatenation - it's an array of slice references
    // Type: array of 2 elements, each element is a slice reference (&[i32])
    let concatenated: [&[i32]; 2] = [array_1.as_slice(), array_2.as_slice()];
    // Prints: [[1, 2, 3], [4, 5, 6]] - shows nested structure
    println!("Concatenated Array: {:?}", concatenated);
}
