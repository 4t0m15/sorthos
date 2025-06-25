#[path = "Sorting/pancakesort.rs"]
mod pancakesort;

fn main() {
    println!("Hello, world!");
    let sorted_array = pancakesort::pancake_sort(vec![3, 1, 4, 1, 5, 9]);
    println!("Sorted array: {:?}", sorted_array);
}