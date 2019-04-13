fn swap(array: &mut [i32], i: usize, j: usize)  {
    let temp = array[i];
    array[i] = array[j];
    array[j] = temp;
}

fn bubble_sort(array: &mut [i32]) {
    for i in 0..(array.len() - 1) {
        for j in i..(array.len()) {
            if array[i] > array[j] {
                swap(array, i, j);
            }
        }
    }
}

fn merge(a1: &Vec<i32>, a2: &Vec<i32>) -> Vec<i32> {
    let mut merged = vec![0; a1.len() + a2.len()];

    let (mut i, mut j, mut k) = (0, 0, 0);

    while (i < a1.len()) && (j < a2.len()) {
        if a1[i] <= a2[j] {
            merged[k] = a1[i];
            i += 1;
        } else {
            merged[k] = a2[j];
            j += 1;
        }

        k += 1;
    }

    while i < a1.len() {
        merged[k] = a1[i];
        i += 1;
        k += 1;
    }
    while j < a2.len() {
        merged[k] = a2[j];
        j += 1;
        k += 1;
    }

    return merged;
}

fn merge_sort_helper(array: &[i32], low: usize, high: usize) -> Vec<i32> {
    let slice_length = high - low;

    if slice_length < 2 {
        let mut copied = vec![0; slice_length];
        for i in 0..slice_length {
            copied[i] = array[low + i];
        }
        return copied;
    } else if slice_length == 2 {
        if array[low] > array[low + 1] {
            return vec![array[low + 1], array[low]];
        } else {
            return vec![array[low], array[low + 1]];
        }
    } else {
        let middle = (low + high) / 2;
        let low_halve = merge_sort_helper(array, low, middle);
        let high_halve = merge_sort_helper(array, middle, high);

        return merge(&low_halve, &high_halve);
    }
}

fn merge_sort(array: &[i32]) -> Vec<i32> {
    return merge_sort_helper(&array, 0, array.len());
}

fn main() {
    let numbers = [12, 24, 1, 2, 0, -54, 2, 35, 2, 1, -9999, 18];
    print!("Original:\n{:?}\n\n", numbers);

    let mut bubble_sorted = numbers;
    bubble_sort(&mut bubble_sorted);
    print!("Bubble Sort:\n{:?}\n\n", bubble_sorted);

    let merge_sorted = merge_sort(&numbers);
    println!("Merge Sort:\n{:?}\n\n", merge_sorted);
}
