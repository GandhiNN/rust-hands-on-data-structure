fn selection_sort_by_swapping(mut arr: Vec<i32>) -> Vec<i32> {
    for _ in 0..arr.len() {
        for j in 0..arr.len() - 1 {
            if arr[j + 1] < arr[j] {
                // reversed bubble sort!
                arr.swap(j, j + 1);
            }
        }
    }
    arr
}

fn main() {
    let my_array: Vec<i32> = vec![64, 34, 25, 5, 22, 11, 90, 12];
    let res = selection_sort_by_swapping(my_array);
    println!("{:?}", res);
}
