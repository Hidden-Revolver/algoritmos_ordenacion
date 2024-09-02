/// Funcion Quicksort con metodo de particion de lomuto
///
/// # Arguments
///
/// * `arr`: array a ordenar
/// * `start`: Indice de inicio
/// * `end`: Indice final
///
/// returns: ()
///
/// # Examples
///
/// ```
///
/// ```
pub fn lomuto_partition_quicksort<T: Ord>(arr: &mut [T], start: usize, end: usize) {
    if end > start {
        let p_pivot: usize = lomuto_partition(arr, start, end);
        lomuto_partition_quicksort(arr, start, p_pivot - 1); // FIXME Algunas veces causa overflow al hacer p_pivot-1, pero al hacer que p_pivot siempre sea mayor que 0, el algoritmo deja de organizar correctamente
        lomuto_partition_quicksort(arr, p_pivot + 1, end);
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
/// returns: usize
///
/// # Examples
///
/// ```
///
/// ```
fn lomuto_partition<T: Ord>(arr: &mut [T], start: usize, end: usize) -> usize {
    let mut tmp_ind = start;
    for i in start..end {
        if arr[i] <= arr[end] {
            arr.swap(tmp_ind, i);
            tmp_ind += 1;
        }
    }
    arr.swap(end, tmp_ind);
    tmp_ind
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
pub fn hoare_partition_quicksort<T: Ord>(arr: &mut [T], start: usize, end: usize) {
    if end > start {
        let p_pivot: usize = hoare_partition(arr, start, end);
        hoare_partition_quicksort(arr, start, p_pivot);
        hoare_partition_quicksort(arr, p_pivot + 1, end);
    }
}

/// Método de partición de hoare, mínimamente modificado, sirve para obtener el punto de pivote que luego se puede utilizar en Quicksort
///
/// # Arguments
///
/// * `arr`: Array de números a ordenar.
/// * `start`: Indice de inicio por el que empezar a ordenar.
/// * `end`: Indice que indica hasta donde ordenar del Array.
///
/// returns: usize
///
/// # Examples
///
/// ```rust
/// let mut arr:[i32;10] = [1,2,3,4,5,6,7,8,9,10];
/// let start:usize = 0;
/// let start:usize = 10;
/// let p_pivot:usize = hoare_partition(arr,start,end);
/// ```
fn hoare_partition<T: Ord>(arr: &mut [T], start: usize, end: usize) -> usize {
    let mut tmp_ind_1: usize = start;
    let mut tmp_ind_2: usize = end;
    loop {
        while arr[tmp_ind_1] < arr[start] {
            tmp_ind_1 += 1;
        }
        while arr[tmp_ind_2] > arr[start] {
            tmp_ind_2 -= 1;
        }
        if tmp_ind_1 >= tmp_ind_2 {
            return tmp_ind_2;
        }
        arr.swap(tmp_ind_1, tmp_ind_2);
    }
}
