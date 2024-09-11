//FIXME necesita arreglarse heapsort ya esta casi
pub fn heapsort(arr: &mut [i32], start:usize, end:usize){
    for i in (start..(end/2)).rev(){
        heapify(arr,end,i);
    }
    for i in (start..end-1).rev(){
        arr.swap(start,i);
        heapify(arr,i,start);
    }
}

fn heapify(arr:&mut[i32], n:usize, i:usize){
    let mut large=i;
    let left=2*i+1;
    let right=2*i+2;
    if left<n && arr[left]>arr[large]{
        large=left;
    }
    if right<n && arr[right]>arr[large]{
        large=right;
    }
    if large !=i{
        arr.swap(i,large);
        heapify(arr,n,large);
    }
}