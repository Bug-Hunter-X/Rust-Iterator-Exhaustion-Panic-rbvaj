fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut iter = vec.iter();

    println!("First element: {}", iter.next().unwrap());
    println!("Second element: {}", iter.next().unwrap());

    // Correct handling of iterator exhaustion:
    if let Some(third_element) = iter.next() {
        println!("Third element: {}", third_element);
    } else {
        println!("Iterator exhausted.");
    }
}