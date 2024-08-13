/**
Este algoritmo es una variación del bubble sort
**/
pub fn gnome_sort (arr: &mut [i32], ind1:usize, ind2:usize){
    let mut i=ind1+1;
    while i<ind2{
        if arr[i-1]<=arr[i]{
            i=i+1;
        } else{
            arr.swap(i-1,i);
            i=i-1;
            if i==ind1 {
                i=ind1+1;
            }
        }
    }

}