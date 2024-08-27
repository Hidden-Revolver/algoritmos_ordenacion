
/// Funcion Quicksort con metodo de particion de lomuto
/// FIXME No funciona correctamente, debe realizarse un analisis al respecto
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
pub fn lomuto_partition_quicksort(arr:&mut [i32],start:usize,end:usize){
    if start>=end{
        return;
    }
    let p=lomuto_partition(arr,start,end-1);
    if p>0{
        lomuto_partition_quicksort(arr,start,p-1);
    }
    lomuto_partition_quicksort(arr,p+1,end);
}
fn lomuto_partition(arr:&mut [i32],start:usize,end:usize) -> usize{
    let p_pivot = arr[end];
    let mut i = start;

    for j in start..end {
        if arr[j] <= p_pivot {
            arr.swap(i, j);
            i=i+1;
        }
    }

    arr.swap(i, end);
    return i;
}
pub fn hoare_partition_quicksort<T:Ord>(arr:&mut [T],start:usize,end:usize){

}
