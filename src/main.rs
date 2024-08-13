use std::time::Instant;

use rand::Rng;

mod insertion_sort;
mod selection_sort;
mod mergesort;
mod heapsort;
mod quicksort;
mod shellsort;
mod gnome_sort;

const LEN: usize = 100;

fn main() {
    let inicio = Instant::now();
    let arr = generador_array();
    println!("Tiempo generando array: {:?}", inicio.elapsed());


    let inicio = Instant::now();
    let shuffle_arr = shuffle(arr);
    println!("{:?}\nTiempo shuffle: {:?}\n", shuffle_arr, inicio.elapsed());

    let mut aux = shuffle_arr.clone();
    let inicio = Instant::now();
    selection_sort::selection_sort(&mut aux, 0, LEN);
    println!("Algoritmo: Selection sort\n  {:?}\nTiempo ordenación: {:?}\n", aux, inicio.elapsed());
    assert_eq!(aux, arr);

    aux = shuffle_arr.clone();

    let inicio = Instant::now();
    insertion_sort::insertion_sort(&mut aux, 0, LEN);
    println!("Algoritmo. Insertion sort\n{:?}\nTiempo ordenación: {:?}\n", aux, inicio.elapsed());
    assert_eq!(aux, arr);

    aux = shuffle_arr.clone();

    let inicio = Instant::now();
    gnome_sort::gnome_sort(&mut aux, 0, LEN);
    println!("Algoritmo: Gnome sort\n{:?}\nTiempo ordenación: {:?}\n", aux, inicio.elapsed());
    assert_eq!(aux, arr);

    aux = shuffle_arr.clone();

    let inicio = Instant::now(); // Causa un overflow del stack, ya que al utilizar el tipo de dato vec!, que es un tipo dinamico(se utiliza para asignar el tamaño de los vectores de forma dinamica), gasta mas recursos del stack.
    mergesort::mergesort(&mut aux, 0, LEN);
    println!("Algoritmo: Merge sort\n{:?}\nTiempo ordenación: {:?}\n", aux, inicio.elapsed());
    assert_eq!(aux, arr);
}

fn shuffle(mut arr: [i32; LEN]) -> [i32; LEN] {
    let mut rng = rand::thread_rng();
    for i in 0..LEN {
        let indx: usize = rng.gen_range(0..LEN);
        arr.swap(indx, i);
    }
    return arr;
}

fn generador_array() -> [i32; LEN] {
    let mut arr: [i32; LEN] = [0; LEN];
    for i in 0..LEN {
        arr[i] = i as i32;
    }
    return arr;
}