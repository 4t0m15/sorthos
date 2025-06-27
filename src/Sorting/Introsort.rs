// Introsort (Introspective Sort) implementation
// A hybrid sorting algorithm that combines quicksort, heapsort, and insertion sort
pub fn introsort(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }
    
    let len = arr.len();
    let max_depth = (len as f64).log2().floor() as usize * 2;
    introsort_util(&mut arr, 0, len, max_depth);
    arr
}

fn introsort_util(arr: &mut Vec<i32>, start: usize, end: usize, max_depth: usize) {
    let size = end - start;
    
    if size <= 1 {
        return;
    }
    
    // Use insertion sort for small arrays
    if size <= 16 {
        insertion_sort_range(arr, start, end);
        return;
    }
    
    // Use heapsort if recursion depth is too deep
    if max_depth == 0 {
        heapsort_range(arr, start, end);
        return;
    }
    
    // Use quicksort for normal cases
    let pivot = partition_range(arr, start, end);
    introsort_util(arr, start, pivot, max_depth - 1);
    introsort_util(arr, pivot + 1, end, max_depth - 1);
}

fn insertion_sort_range(arr: &mut Vec<i32>, start: usize, end: usize) {
    for i in start + 1..end {
        let key = arr[i];
        let mut j = i;
        
        while j > start && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        
        arr[j] = key;
    }
}

fn heapsort_range(arr: &mut Vec<i32>, start: usize, end: usize) {
    let size = end - start;
    
    // Build max heap
    for i in (0..size / 2).rev() {
        heapify(arr, start, size, i);
    }
    
    // Extract elements from heap one by one
    for i in (1..size).rev() {
        arr.swap(start, start + i);
        heapify(arr, start, i, 0);
    }
}

fn heapify(arr: &mut Vec<i32>, start: usize, size: usize, root: usize) {
    let mut largest = root;
    let left = 2 * root + 1;
    let right = 2 * root + 2;
    
    if left < size && arr[start + left] > arr[start + largest] {
        largest = left;
    }
    
    if right < size && arr[start + right] > arr[start + largest] {
        largest = right;
    }
    
    if largest != root {
        arr.swap(start + root, start + largest);
        heapify(arr, start, size, largest);
    }
}

fn partition_range(arr: &mut Vec<i32>, start: usize, end: usize) -> usize {
    let pivot = arr[end - 1];
    let mut i = start;
    
    for j in start..end - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, end - 1);
    i
}