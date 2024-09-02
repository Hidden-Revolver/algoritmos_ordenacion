//FIXME necesita arreglarse heapsort
pub fn heapsort(arr: &mut [i32], start:usize, end:usize){
    for i in (end/2)-1..=start{
        heapify(arr,end,i);
    }
    for i in end-1..=start{
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