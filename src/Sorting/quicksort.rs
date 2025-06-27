// Quick Sort implementation
pub fn quick_sort(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }
    
    let pivot_index = partition(&mut arr);
    let mut left = arr[0..pivot_index].to_vec();
    let mut right = arr[pivot_index + 1..].to_vec();
    
    left = quick_sort(left);
    right = quick_sort(right);
    
    let mut result = Vec::new();
    result.extend(left);
    result.push(arr[pivot_index]);
    result.extend(right);
    
    result
}

fn partition(arr: &mut Vec<i32>) -> usize {
    let len = arr.len();
    let pivot = arr[len - 1];
    let mut i = 0;
    
    for j in 0..len - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, len - 1);
    i
}