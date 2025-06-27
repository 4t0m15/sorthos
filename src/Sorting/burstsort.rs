// Burst Sort (simplified version for integer arrays)
// This is a simplified implementation focusing on the core concept
pub fn burst_sort(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }
    
    // For simplicity, we'll use a hybrid approach with insertion sort for small arrays
    const THRESHOLD: usize = 10;
    
    if arr.len() <= THRESHOLD {
        insertion_sort(&mut arr);
        return arr;
    }
    
    // Partition based on digit (simplified)
    let max_val = *arr.iter().max().unwrap();
    let min_val = *arr.iter().min().unwrap();
    let range = max_val - min_val + 1;
    
    if range <= 256 {
        // Use counting sort for small ranges
        counting_sort(&mut arr, min_val, max_val);
    } else {
        // Fall back to quick sort for large ranges
        arr = quick_sort_helper(arr);
    }
    
    arr
}

fn insertion_sort(arr: &mut Vec<i32>) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        
        arr[j] = key;
    }
}

fn counting_sort(arr: &mut Vec<i32>, min_val: i32, max_val: i32) {
    let range = (max_val - min_val + 1) as usize;
    let mut count = vec![0; range];
    
    for &num in arr.iter() {
        count[(num - min_val) as usize] += 1;
    }
    
    let mut index = 0;
    for (i, &freq) in count.iter().enumerate() {
        for _ in 0..freq {
            arr[index] = min_val + i as i32;
            index += 1;
        }
    }
}

fn quick_sort_helper(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }
    
    let len = arr.len();
    let pivot = len / 2;
    let pivot_val = arr[pivot];
    arr.swap(pivot, len - 1);
    
    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= pivot_val {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, len - 1);
    
    let mut left = arr[0..i].to_vec();
    let mut right = arr[i + 1..].to_vec();
    
    left = quick_sort_helper(left);
    right = quick_sort_helper(right);
    
    let mut result = Vec::new();
    result.extend(left);
    result.push(pivot_val);
    result.extend(right);
    
    result
}