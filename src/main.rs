extern crate crossbeam;
extern crate rand;
extern crate time;
extern crate num_cpus;

use rand::Rng;

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

fn merge(a1: &[i32], a2: &[i32]) -> Vec<i32> {
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

    merged
}

fn merge_sort(array: &[i32]) -> Vec<i32> {
    let length = array.len();

    if length < 2 {
        array.to_vec()
    } else if length == 2 {
        if array[0] > array[1] {
            vec![array[1], array[0]]
        } else {
            array.to_vec()
        }
    } else {
        let middle = length / 2;
        let low_slice = &array[0..middle];
        let high_slice = &array[middle..length];
        let low_halve = merge_sort(low_slice);
        let high_halve = merge_sort(high_slice);
        merge(&low_halve, &high_halve)
    }
}

fn merge_sort_threaded(array: &[i32], nthreads: usize) -> Vec<i32> {
    let length = array.len();

    if length < 2 {
        array.to_vec()
    } else if length == 2 {
        if array[0] > array[1] {
            vec![array[1], array[0]]
        } else {
            array.to_vec()
        }
    } else {
        let middle = length / 2;
        let low_slice = &array[0..middle];
        let high_slice = &array[middle..length];
        let low_halve: Vec<i32>;
        let high_halve: Vec<i32>;

        if nthreads > 0 {
            let result = crossbeam::scope(
                |scope| {
                    let high_nthreads = nthreads / 2;
                    let handle = scope.spawn(
                        move |_| {merge_sort_threaded(low_slice, high_nthreads)}
                    );
                    let high_halve = merge_sort_threaded(
                        high_slice,
                        nthreads - high_nthreads
                    );
                    let low_halve = handle.join().unwrap();

                    (low_halve, high_halve)
                }
            ).unwrap();
            low_halve = result.0;
            high_halve = result.1;
        } else {
            low_halve = merge_sort_threaded(low_slice, nthreads);
            high_halve = merge_sort_threaded(high_slice, nthreads);
        }

        merge(&low_halve, &high_halve)
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut numbers = [0; 1000000];
    for i in 0..numbers.len() {
        numbers[i] = rng.gen();
    }
    let mut before: u64;
    let mut after: u64;
    let mut in_ms: u64;

    if numbers.len() < 10000 {
        let mut _bubble_sorted = numbers;
        before = time::precise_time_ns();
        bubble_sort(&mut _bubble_sorted);
        after = time::precise_time_ns();
        in_ms = (after - before) / 1000000;
        println!("Bubble Sort: {}ms", in_ms);
    }

    before = time::precise_time_ns();
    let _merge_sorted = merge_sort(&numbers);
    after = time::precise_time_ns();
    in_ms = (after - before) / 1000000;
    println!("Merge Sort: {}ms", in_ms);

    before = time::precise_time_ns();
    let _merge_sorted_threaded = merge_sort_threaded(
        &numbers,
        num_cpus::get()
    );
    after = time::precise_time_ns();
    in_ms = (after - before) / 1000000;
    println!("Merge Sort (threaded): {}ms", in_ms);

    before = time::precise_time_ns();
    let _stdlib_sorted = numbers.sort();
    after = time::precise_time_ns();
    in_ms = (after - before) / 1000000;
    println!("stdlib: {}ms", in_ms);

    before = time::precise_time_ns();
    let _stdlib_sorted_unstable = numbers.sort_unstable();
    after = time::precise_time_ns();
    in_ms = (after - before) / 1000000;
    println!("stdlib (unstable): {}ms", in_ms);
}
