fn main() {
    let my_array = vec![7, 12, 9, 4, 11];
    let mut min_val = my_array[0];

    for elem in my_array {
        if elem < min_val {
            min_val = elem;
        }
    }

    println!("Lowest value: {}", min_val);
}
