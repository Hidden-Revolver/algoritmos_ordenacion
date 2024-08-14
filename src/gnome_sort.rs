/**
Este algoritmo es una variación del bubble sort
 **/
pub fn gnome_sort(arr: &mut [i32], start: usize, end: usize) {
    let mut i = start + 1;
    while i < end {
        if arr[i - 1] <= arr[i] {
            i = i + 1;
        } else {
            arr.swap(i - 1, i);
            i = i - 1;
            if i == start {
                i = start + 1;
            }
        }
    }
}