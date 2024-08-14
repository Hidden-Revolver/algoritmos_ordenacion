use std::mem;
use std::time::Instant;

use rand::Rng;

mod insertion_sort;
mod selection_sort;
mod mergesort;
mod heapsort;
mod quicksort;
mod shellsort;
mod gnome_sort;
mod counting_sort;
mod bucket_sort;
mod radix_sort;
mod bubble_sort;
mod comb_sort;
mod exchange_sort;
mod timsort;
mod pidgeonhole_sort;
mod cycle_sort;
mod cockatil_sort;
mod strand_sort;
mod bitonic_sort;
mod pancake_sort;
mod bogo_sort;
mod sleep_sort;
mod stooge_sort;
mod tree_sort;
mod brick_sort;
mod introsort;
mod tournament_sort;
mod block_sort;
mod smoothsort;
mod patience_sort;
mod cube_sort;
mod library_sort;
mod odd_even_sort;
mod spread_sort;
mod burstsort;
mod flashsort;
mod postman_sort;
mod bead_sort;
mod merge_insertion_sort;
mod i_cant_believe_it_can_sort_sort;
mod spaghetti_sort;
mod network_sort;
mod bitonic_merge_sort;
mod slow_sort;
mod franceschinis_method_sort;
mod thorups_algorithm_sort;

const LEN: usize=100;
const N_ALG:usize=46;

fn main() { //Considerar  generar threads para cada algoritmo, para intentar que todos se hagan por separado 
    let inicio = Instant::now();
    let arr = generador_array();
    println!("Tiempo generando array: {:?}", inicio.elapsed());

    let inicio = Instant::now();
    let shuffle_arr = shuffle(arr);
    println!("Shuffle:\n{:?}\nTiempo shuffle: {:?}\nEspacio en memoria del array: {:?}bytes\n", shuffle_arr, inicio.elapsed(),mem::size_of_val(&shuffle_arr));
    
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
    mergesort::top_down_mergesort(&mut aux, 0, LEN);
    println!("Algoritmo: top-down Merge sort\n{:?}\nTiempo ordenación: {:?}\n", aux, inicio.elapsed());
    assert_eq!(aux, arr);

    aux = shuffle_arr.clone();

    let inicio = Instant::now(); // Causa un overflow del stack, ya que al utilizar el tipo de dato vec!, que es un tipo dinamico(se utiliza para asignar el tamaño de los vectores de forma dinamica), gasta mas recursos del stack.
    mergesort::bottom_up_merge_sort(&mut aux, 0, LEN);
    println!("Algoritmo: bottom-up Merge sort\n{:?}\nTiempo ordenación: {:?}\n", aux, inicio.elapsed());
    assert_eq!(aux, arr);

    aux = shuffle_arr.clone();

    let inicio = Instant::now(); // Causa un overflow del stack, ya que al utilizar el tipo de dato vec!, que es un tipo dinamico(se utiliza para asignar el tamaño de los vectores de forma dinamica), gasta mas recursos del stack.
    mergesort::in_place_mergesort(&mut aux, 0, LEN);
    println!("Algoritmo: in-place Merge sort\n{:?}\nTiempo ordenación: {:?}\n", aux, inicio.elapsed());
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
