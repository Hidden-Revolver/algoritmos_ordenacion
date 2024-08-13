// FIXME Este metodo necesita cambios para evitar un overflow del stack
/*
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
*/
/**
arr tiene la lista de numeros a ordenar; arr_op es una lista para trabajar
**/
//FIXME este código no ordena por alguna razón. debería arreglarlo
pub fn top_down_mergesort(arr:&mut[i32],arr_op:&mut[i32],ind1:usize,ind2:usize){
    copiar_array(arr,arr_op,ind1,ind2);
    top_down_split_merge(arr_op,arr,ind1,ind2);
}
fn top_down_merge(arr:&mut[i32],arr_op:&mut[i32],ind1:usize,ind_med:usize,ind2:usize){
    let mut i=ind1;let mut j=ind_med;
    for k in ind1..ind2{
        if i<ind_med &&(j<ind2 || arr[i]<arr[j]) {
            arr_op[k]=arr[i];
            i=i+1;
        }else{
            arr_op[k]=arr[j];
            j=j+1;
        }
    }
}
fn top_down_split_merge(arr:&mut[i32],arr_op:&mut[i32],ind1:usize,ind2:usize){
    if ind2-ind1<=1 {
        return;
    }
    let ind_med=(ind1+ind2)/2;
    top_down_split_merge(arr,arr_op,ind1,ind_med);
    top_down_split_merge(arr,arr_op,ind_med,ind2);
    top_down_merge(arr,arr_op,ind1,ind_med,ind2);
}
fn copiar_array(arr:&mut[i32],arr_op:&mut[i32],ind1:usize,ind2:usize){
    for i in ind1..ind2{
        arr_op[i]=arr[i];
    }
}
