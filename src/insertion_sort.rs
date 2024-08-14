/**
Este algoritmo utiliza la implementación clasica del insertion sort
 **/
pub fn insertion_sort(arr: &mut [i32], start: usize, end: usize) {
    for i in start..end {
        let t = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > t {
            arr[j] = arr[j - 1];
            j = j - 1;
        }
        arr[j] = t;
    }
}