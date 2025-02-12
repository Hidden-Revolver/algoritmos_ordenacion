use crate::LEN;

pub fn sort(arr: &mut [i32; LEN], start: usize, end: usize) { //TODO MAL 
    let mut max:i32=arr[start];
    for i in start+1..end{
        if arr[i]>max { 
            max= arr[i];
        }
    }
    print!("{:?}",max);
    let mut beads=vec![vec![0;end-start]; max as usize]; // Utilizar vec en vez de array me duele, pero es necesario por el tipo de algoritmo de ordenado
    println!("{:?}",beads);
    for i in start..end {
        for j in 0..arr[i] as usize {
            beads[i][j]=1;
        }
    }
    for j in 0..max as usize {
        let mut sum: usize =0;
        for i in 0..end{
            sum+=beads[i][j];
            beads[i][j]=0;
        }
        for i in (end-sum..end-1).rev(){
            arr[i]= (j + 1) as i32;
        }
    }
}