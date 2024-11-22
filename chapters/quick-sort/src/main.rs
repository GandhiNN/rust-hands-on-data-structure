fn median_of_three<T: Ord>(arr: &[T]) -> usize {
    let first_element = &arr[0];
    let middle_element = &arr[arr.len() / 2];
    let last_element = &arr[arr.len() - 1];

    if ((first_element < middle_element) && (middle_element < last_element))
        || ((last_element < middle_element) && (middle_element < first_element))
    {
        arr.len() / 2
    } else if ((middle_element < first_element) && (first_element < last_element))
        || ((last_element < first_element) && (first_element < middle_element))
    {
        0
    } else {
        arr.len() - 1
    }
}

fn quicksort<T: Ord>(mut arr: Vec<T>) -> Vec<T> {
    if arr.len() <= 1 {
        return arr;
    }

    let pivot_index = median_of_three(&arr);
    let pivot = arr.remove(pivot_index);
    let mut left = vec![];
    let mut right = vec![];

    for item in arr {
        if item <= pivot {
            left.push(item);
        } else {
            right.push(item);
        }
    }
    let mut sorted_left = quicksort(left);
    let mut sorted_right = quicksort(right);

    sorted_left.push(pivot);
    sorted_left.append(&mut sorted_right);

    sorted_left
}

fn main() {
    let arr = vec![132, 22, 190, 23, 24, 25, 33, 0, 2, 3];

    println!("Array before quicksorted: {:?}", arr);
    println!("Array after quicksorted: {:?}", quicksort(arr));
}
