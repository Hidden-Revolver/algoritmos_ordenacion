/**
Esta implementación del algoritmo de ordenación Selection sort utiliza una referencia del array original para no tener que hacer una copia redundante de la lista que se quiere ordenar.
El resto de la implementación es prácticamente igual a como sería una implementación estándar de selection sort
**/
pub fn selection_sort<T: Ord>(arr: &mut [T], start: usize, end: usize) {
    for i in start..end-1 {
        let mut indx_menor = i;
        for j in (i+1)..end {
            if arr[j] < arr[indx_menor] {
                indx_menor = j;
            }
        }
        if indx_menor != i {
            arr.swap(i, indx_menor);
        }
    }
}
