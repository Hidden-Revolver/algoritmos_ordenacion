// FIXME Este metodo necesita cambios para evitar un overflow del stack
pub fn mergesort(arr: &mut [i32], ind1: usize, ind2: usize) {
    if ind1 < ind2 {
        let ind_med = ind1 + (ind2 - ind1) / 2;
        mergesort(arr, ind1, ind_med);
        mergesort(arr, ind_med, ind2);
        merge(arr, ind1, ind_med, ind2);
    }
}
fn merge(arr: &mut [i32], ind1: usize, ind_med: usize, ind3: usize) {
    let n1 = ind_med - ind1 + 1;
    let n2 = ind3 - ind_med;
    let mut arr_izq = vec![0; n1];
    let mut arr_der = vec![0; n2];
    for i in 0..n1 {
        arr_izq[i] = arr[ind1 + i];
    }
    for i in 0..n2 {
        arr_der[i] = arr[ind_med + 1 + i];
    }
    let mut i = 0;
    let mut j = 0;
    let mut k = ind1;
    while i < n1 && j < n2 {
        if arr_izq[i] <= arr_der[j] {
            arr[k] = arr_izq[i];
            i = i + 1;
        } else {
            arr[k] = arr_der[j];
            j = j + 1;
        }
        k = k + 1;
    }
    while i < n1 {
        arr[k] = arr_izq[i];
        i = i + 1;
        k = k + 1;
    }
    while j < n2 {
        arr[k] = arr_der[j];
        j = j + 1;
        k = k + 1;
    }
}