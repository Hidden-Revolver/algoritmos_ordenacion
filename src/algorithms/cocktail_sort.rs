
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
pub fn cocktail_sort<T: Ord>(arr: &mut [T], start: usize, end: usize) {
    let mut ord: bool = false;
    while !ord {
        ord=true;
        for i in start + 1..end {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                ord = false;
            }
        }
        if ord{
            break;
        }
        ord=true;
        for i in (start+1..end).rev(){
            if arr[i-1]>arr[i] {
                arr.swap(i-1,i);
                ord=false
            }
        }
    }
}
/**
FIXME Esta funcion no funciona como deberia asi que se programará en un futuro
**/
pub fn _cocktail_sort_optimized<T:Ord>(arr:&mut[T],mut start:usize,mut end:usize){
    while start<=end {
        let mut aux_start:usize=start;
        let mut aux_end:usize=end;
        for i in start+1..end{
            if arr[i-1]>arr[i] {
                arr.swap(i-1,i);
                aux_end=i;
            }
        }
        end=aux_end-1;
        for i in (start+1..end).rev(){
            if arr[i-1]>arr[i] {
                arr.swap(i-1,i);
                aux_start=i-1;
            }
        }
        start=aux_start+1;
    }
}