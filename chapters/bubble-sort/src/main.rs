fn bubble_sort_simple(mut arr: Vec<i32>) -> Vec<i32> {
    let n = arr.len();

    for _ in 0..n {
        for j in 0..n - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
    arr
}

fn bubble_sort_enhanced(mut arr: Vec<i32>) -> Vec<i32> {
    let n = arr.len();

    for _ in 0..n {
        let mut still_swapping = false;
        for j in 0..n - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                still_swapping = true; // if there is still swapping operation in place
            }
        }
        if !still_swapping {
            // break the inner loop because the array is already sorted
            break;
        }
    }
    arr
}

fn main() {
    let my_array = vec![64, 34, 25, 12, 22, 11, 90, 5];
    let sorted_arr = bubble_sort_simple(my_array);
    println!("{:?}", sorted_arr);

    let almost_sorted_array = vec![1, 2, 3, 4, 10, 9];
    let res = bubble_sort_enhanced(almost_sorted_array);
    println!("{:?}", res);
}
