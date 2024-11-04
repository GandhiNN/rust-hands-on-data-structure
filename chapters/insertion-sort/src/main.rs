/// Sorts a mutable slice using in-place insertion sort algorithm.
///
/// Time complexity is `O(n^2)`, where `n` is the number of elements.
/// Space complexity is `O(1)` as it sorts elements in-place.
fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) -> &mut [T] {
    for i in 1..arr.len() {
        let mut j = i;
        let cur = arr[i];

        while j > 0 && cur < arr[j - 1] {
            arr[j] = arr[j - 1];
            j -= 1;
        }

        arr[j] = cur;
    }
    arr
}

fn main() {
    let mut my_array = [64, 34, 25, 12, 22, 11, 90, 5];
    let sorted_array = insertion_sort(&mut my_array);
    println!("{:?}", sorted_array);
    println!("Hello, world!");
}
