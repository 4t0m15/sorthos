// Quad Sort - A hybrid stable sorting algorithm
// This is a simplified implementation of the quad sort concept
pub fn quad_sort(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }
    
    quad_sort_recursive(&mut arr, 0, arr.len());
    arr
}

fn quad_sort_recursive(arr: &mut Vec<i32>, start: usize, end: usize) {
    let size = end - start;
    
    if size <= 1 {
        return;
    }
    
    // Use insertion sort for small arrays
    if size <= 32 {
        insertion_sort_range(arr, start, end);
        return;
    }
    
    // Divide into 4 parts
    let quarter = size / 4;
    let mid1 = start + quarter;
    let mid2 = start + 2 * quarter;
    let mid3 = start + 3 * quarter;
    
    // Recursively sort each quarter
    quad_sort_recursive(arr, start, mid1);
    quad_sort_recursive(arr, mid1, mid2);
    quad_sort_recursive(arr, mid2, mid3);
    quad_sort_recursive(arr, mid3, end);
    
    // Merge the sorted quarters
    merge_four_parts(arr, start, mid1, mid2, mid3, end);
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

fn merge_four_parts(arr: &mut Vec<i32>, start: usize, mid1: usize, mid2: usize, mid3: usize, end: usize) {
    // First merge pairs
    let temp1 = merge_two_sorted(&arr[start..mid1], &arr[mid1..mid2]);
    let temp2 = merge_two_sorted(&arr[mid2..mid3], &arr[mid3..end]);
    
    // Then merge the results
    let final_result = merge_two_sorted(&temp1, &temp2);
    
    // Copy back to original array
    for (i, &val) in final_result.iter().enumerate() {
        arr[start + i] = val;
    }
}

fn merge_two_sorted(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut i = 0;
    let mut j = 0;
    
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }
    
    while i < left.len() {
        result.push(left[i]);
        i += 1;
    }
    
    while j < right.len() {
        result.push(right[j]);
        j += 1;
    }
    
    result
}