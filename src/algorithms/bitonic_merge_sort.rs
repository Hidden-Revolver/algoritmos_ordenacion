
pub fn bitonic_sort_not_power_of_two(arr: &mut [i32], start: usize, end: usize) {
    let n = end - start + 1;
    let padded_len = n.next_power_of_two();
    let max_value = *arr[start..=end].iter().max().unwrap();

    // Create a new array with padding
    let mut padded_arr = vec![max_value; padded_len];
    padded_arr[..n].copy_from_slice(&arr[start..=end]);

    for k in 1..=padded_len.trailing_zeros() {
        let k = 1 << k; // 2^k
        for j in (0..padded_len).step_by(k) {
            bitonic_merge(&mut padded_arr, j, k, j & k != 0);
        }
    }
    // Copy sorted elements back to the original array
    arr[start..=end].copy_from_slice(&padded_arr[..n]);
    arr.reverse();
}

pub fn bitonic_sort_power_of_two(arr: &mut [i32], start: usize, end: usize) {
    let n = end - start + 1;
    assert!(n.is_power_of_two(), "Length of the range to be sorted must be a power of two");

    for k in 1..=n.trailing_zeros() {
        let k = 1 << k; // 2^k
        for j in (start..=end).step_by(k) {
            bitonic_merge(arr, j, k, (j - start) & k != 0);
        }
    }
    arr.reverse();
}

fn bitonic_merge(arr: &mut [i32], low: usize, cnt: usize, dir: bool) {
    if cnt > 1 {
        let k = cnt / 2;
        for i in low..(low + k) {
            compare_and_swap(arr, i, i + k, dir);
        }
        bitonic_merge(arr, low, k, dir);
        bitonic_merge(arr, low + k, k, dir);
    }
}

fn compare_and_swap(arr: &mut [i32], i: usize, j: usize, dir: bool) {
    if dir == (arr[i] > arr[j]) {
        arr.swap(i, j);
    }
}