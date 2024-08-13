/**
Este algoritmo utiliza la implementación clasica del insertion sort
**/
pub fn insertion_sort (arr: &mut [i32], ind1:usize, ind2:usize){
    for i in ind1..ind2{
        let t = arr[i];
        let mut j=i;
        while j>0 && arr[j-1]>t{
            arr[j]=arr[j-1];
            j=j-1;
        }
        arr[j]=t;
    }
}