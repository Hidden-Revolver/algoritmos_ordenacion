use std::mem;
use std::time::Instant;

use rand::Rng;

mod algorithms;

const LEN: usize = 128;
const _N_ALG: usize = 46;

fn main() {
    //Considerar  generar threads para cada algoritmo, para intentar que todos se hagan por separado
    let inicio = Instant::now();
    let arr = generador_array();
    println!("Tiempo generando array: {:?}", inicio.elapsed());

    let inicio = Instant::now();
    let shuffle_arr = shuffle(arr);
    println!(
        "Shuffle:\n{:?}\nTiempo shuffle: {:?}\nEspacio en memoria del array: {:?}bytes\n",
        shuffle_arr,
        inicio.elapsed(),
        mem::size_of_val(&shuffle_arr)
    );

    let mut aux = shuffle_arr;
    let inicio = Instant::now();
    algorithms::selection_sort::selection_sort(&mut aux, 0, LEN);
    println!(
        "Algoritmo: Selection sort\n  {:?}\nTiempo ordenación: {:?}\n",
        aux,
        inicio.elapsed()
    );
    assert_eq!(aux, arr);
    aux = shuffle_arr;

    let inicio = Instant::now();
    algorithms::insertion_sort::insertion_sort(&mut aux, 0, LEN);
    println!(
        "Algoritmo. Insertion sort\n{:?}\nTiempo ordenación: {:?}\n",
        aux,
        inicio.elapsed()
    );
    assert_eq!(aux, arr);

    aux = shuffle_arr;

    let inicio = Instant::now();
    algorithms::gnome_sort::gnome_sort(&mut aux, 0, LEN);
    println!(
        "Algoritmo: Gnome sort\n{:?}\nTiempo ordenación: {:?}\n",
        aux,
        inicio.elapsed()
    );
    assert_eq!(aux, arr);

    aux = shuffle_arr;

    let inicio = Instant::now();
    algorithms::mergesort::top_down_mergesort(&mut aux, 0, LEN);
    println!(
        "Algoritmo: top-down Merge sort\n{:?}\nTiempo ordenación: {:?}\n",
        aux,
        inicio.elapsed()
    );
    assert_eq!(aux, arr);

    aux = shuffle_arr;

    let inicio = Instant::now();
    algorithms::mergesort::bottom_up_merge_sort(&mut aux, 0, LEN);
    println!(
        "Algoritmo: bottom-up Merge sort\n{:?}\nTiempo ordenación: {:?}\n",
        aux,
        inicio.elapsed()
    );
    assert_eq!(aux, arr);

    aux = shuffle_arr;

    let inicio = Instant::now();
    algorithms::mergesort::in_place_mergesort(&mut aux, 0, LEN);
    println!(
        "Algoritmo: in-place Merge sort\n{:?}\nTiempo ordenación: {:?}\n",
        aux,
        inicio.elapsed()
    );
    assert_eq!(aux, arr);

    aux = shuffle_arr;

    let inicio = Instant::now();
    algorithms::bubble_sort::bubble_sort(&mut aux, 0, LEN);
    println!(
        "Algoritmo: bubble sort\n{:?}\nTiempo ordenación: {:?}\n",
        aux,
        inicio.elapsed()
    );
    assert_eq!(aux, arr);

    aux = shuffle_arr;

    let inicio = Instant::now();
    algorithms::bubble_sort::bubble_sort_optimized_1(&mut aux, 0, LEN);
    println!(
        "Algoritmo: bubble sort (optimized 1)\n{:?}\nTiempo ordenación: {:?}\n",
        aux,
        inicio.elapsed()
    );
    assert_eq!(aux, arr);

    aux = shuffle_arr;

    let inicio = Instant::now();
    algorithms::bubble_sort::bubble_sort_optimized_2(&mut aux, 0, LEN);
    println!(
        "Algoritmo: bubble sort (optimized 2)\n{:?}\nTiempo ordenación: {:?}\n",
        aux,
        inicio.elapsed()
    );
    assert_eq!(aux, arr);

    aux = shuffle_arr;

    let inicio = Instant::now();
    algorithms::cocktail_sort::cocktail_sort(&mut aux, 0, LEN);
    println!(
        "Algoritmo: cocktail sort\n{:?}\nTiempo ordenación: {:?}\n",
        aux,
        inicio.elapsed()
    );
    assert_eq!(aux, arr);

    /*
    aux = shuffle_arr.clone();

    let inicio = Instant::now();
    cocktail_sort::cocktail_sort_optimized(&mut aux, 0, LEN);
    println!("Algoritmo: cocktail sort(optimized)\n{:?}\nTiempo ordenación: {:?}\n", aux, inicio.elapsed());
    assert_eq!(aux, arr);
     */

    aux = shuffle_arr;

    let inicio = Instant::now(); // Causa overflow algunas veces
    algorithms::quicksort::lomuto_partition_quicksort(&mut aux, 0, LEN - 1);
    println!(
        "Algoritmo: quicksort(lomuto partition scheme)\n{:?}\nTiempo ordenación: {:?}\n",
        aux,
        inicio.elapsed()
    );
    assert_eq!(aux, arr);

    aux = shuffle_arr;

    let inicio = Instant::now();
    algorithms::quicksort::hoare_partition_quicksort(&mut aux, 0, LEN - 1);
    println!(
        "Algoritmo: quicksort(hoare partition scheme)\n{:?}\nTiempo ordenación: {:?}\n",
        aux,
        inicio.elapsed()
    );
    assert_eq!(aux, arr);

    aux = shuffle_arr;

    let inicio = Instant::now();
    algorithms::bitonic_merge_sort::bitonic_sort_power_of_two(&mut aux, 0, LEN - 1);
    println!(
        "Algoritmo: bitonic merge sort (power of two length lists)\n{:?}\nTiempo ordenación: {:?}\n",
        aux,
        inicio.elapsed()
    );
    assert_eq!(aux, arr);

    aux = shuffle_arr;

    let inicio = Instant::now();
    algorithms::bitonic_merge_sort::bitonic_sort_not_power_of_two(&mut aux, 0, LEN - 1);
    println!(
        "Algoritmo: bitonic merge sort (not power of two length lists)\n{:?}\nTiempo ordenación: {:?}\n",
        aux,
        inicio.elapsed()
    );
    assert_eq!(aux, arr);
}

fn shuffle(mut arr: [i32; LEN]) -> [i32; LEN] {
    let mut rng = rand::thread_rng();
    for i in 0..LEN {
        let indx: usize = rng.gen_range(0..LEN);
        arr.swap(indx, i);
    }
    arr
}

fn generador_array() -> [i32; LEN] {
    let mut arr: [i32; LEN] = [0; LEN];
    for i in 0..LEN {
        arr[i] = i as i32;
    }
    arr
}
