pub fn bubble_sort<T: Ord>(arr: &mut [T],start:usize,end:usize){
    let mut ord:bool=false;
    while !ord{
        ord=true;
        for i in start+1..end{
            if arr[i-1]>arr[i]{
                arr.swap(i-1,i);
                ord=false;
            }
        }
    }
}

pub fn bubble_sort_optimized_1<T:Ord>(arr:&mut[T],start:usize,mut end:usize){
    let mut ord:bool=false;
    while!ord{
        ord=true;
        for i in start+1..end {
            if arr[i-1]>arr[i]{
                arr.swap(i-1,i);
                ord=false;
            }
        }
        end -= 1;
    }
}

pub fn bubble_sort_optimized_2<T:Ord>(arr:&mut[T],start:usize,end:usize){
    let mut n=end-start;
    while n>=1 {
        let mut aux=0;
        for i in start+1..n{
            if arr[i-1]>arr[i] {
                arr.swap(i-1,i);
                aux=i;
            }
        }
        n=aux;
    }
}