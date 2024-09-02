use crate::LEN;

///
///
/// # Arguments
///
/// * `arr`:
/// * `start`:
/// * `end`:
///
/// returns: ()
///
/// # Examples
///
/// ```
///
/// ```
pub fn top_down_mergesort<T: Ord + Copy>(arr: &mut [T], start: usize, end: usize) {
    if end - start > 1 {
        let mid = (start + end) / 2;
        top_down_mergesort(arr, start, mid);
        top_down_mergesort(arr, mid, end);
        merge(arr, start, mid, end);
    }
}

///
///
/// # Arguments
///
/// * `arr`:
/// * `start`:
/// * `end`:
///
/// returns: ()
///
/// # Examples
///
/// ```
///
/// ```
pub fn bottom_up_merge_sort<T: Ord + Copy>(arr: &mut [T], start: usize, end: usize) {
    let mut width = 1;
    let n = end - start;

    while width < n {
        let mut i = start;
        while i < end {
            let left = i;
            let mid = i + width;
            let right = (i + 2 * width).min(end);

            if mid < end {
                merge(arr, left, mid, right);
            }

            i = i + (2 * width);
        }
        width = width * 2;
    }
}

///
///
/// # Arguments
///
/// * `arr`:
/// * `start`:
/// * `end`:
///
/// returns: ()
///
/// # Examples
///
/// ```
///
/// ```
pub fn in_place_mergesort<T: Ord>(arr: &mut [T], start: usize, end: usize) {
    let mut width = 1;
    while width < end {
        let mut i = start;
        while i < end {
            let left = i;
            let mid = i + width;
            let right = if i + 2 * width > end {
                end
            } else {
                i + 2 * width
            };
            in_place_merge(arr, left, mid, right);
            i = i + (2 * width);
        }
        width = width * 2;
    }
}

fn merge<T: Ord + Copy>(arr: &mut [T], start: usize, mid: usize, end: usize) {
    let left_len = mid - start;
    let right_len = end - mid;

    let mut left = [arr[start]; LEN]; // La longitud de los arrays a la izquierda y a la derecha debe ser LEN, si no el algoritmo bottom_up falla por intentar acceder a un indice mas grande que el tamaño del array
    let mut right = [arr[mid]; LEN];

    for i in 0..left_len {
        left[i] = arr[start + i];
    }
    for i in 0..right_len {
        right[i] = arr[mid + i];
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = start;

    while i < left_len && j < right_len {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i = i + 1;
        } else {
            arr[k] = right[j];
            j = j + 1;
        }
        k = k + 1;
    }

    while i < left_len {
        arr[k] = left[i];
        i = i + 1;
        k = k + 1;
    }

    while j < right_len {
        arr[k] = right[j];
        j = j + 1;
        k = k + 1;
    }
}

fn in_place_merge<T: Ord>(arr: &mut [T], left: usize, mid: usize, right: usize) {
    let mut i = left;
    let mut j = mid;
    while i < j && j < right {
        if arr[i] <= arr[j] {
            i = i + 1;
        } else {
            let mut k = j;
            while k > i {
                arr.swap(k, k - 1);
                k = k - 1;
            }
            i = i + 1;
            j = j + 1;
        }
    }
}
