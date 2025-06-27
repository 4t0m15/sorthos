// Spaghetti Sort (Gravity Sort) implementation
// A sorting algorithm that simulates the behavior of spaghetti falling under gravity
pub fn spaghetti_sort(arr: Vec<i32>) -> Vec<i32> {
    if arr.is_empty() {
        return arr;
    }
    
    // Handle negative numbers by offsetting
    let min_val = *arr.iter().min().unwrap();
    let offset = if min_val < 0 { -min_val } else { 0 };
    
    let positive_arr: Vec<i32> = arr.iter().map(|x| x + offset).collect();
    let max_val = *positive_arr.iter().max().unwrap();
    
    if max_val == 0 {
        return arr;
    }
    
    // Create a matrix to represent the spaghetti
    let mut matrix = vec![vec![false; positive_arr.len()]; max_val as usize];
    
    // Mark the spaghetti positions
    for (col, &height) in positive_arr.iter().enumerate() {
        for row in 0..height as usize {
            matrix[row][col] = true;
        }
    }
    
    // Apply gravity - let the spaghetti fall
    for col in 0..positive_arr.len() {
        let mut bottom = (max_val as usize).saturating_sub(1);
        
        for row in (0..max_val as usize).rev() {
            if matrix[row][col] {
                if row != bottom {
                    matrix[bottom][col] = true;
                    matrix[row][col] = false;
                }
                if bottom > 0 {
                    bottom -= 1;
                }
            }
        }
    }
    
    // Count the heights after gravity
    let mut result = Vec::new();
    for col in 0..positive_arr.len() {
        let mut height = 0;
        for row in 0..max_val as usize {
            if matrix[row][col] {
                height += 1;
            }
        }
        result.push(height - offset);
    }
    
    result
}

// Alternative implementation using counting sort approach
pub fn spaghetti_sort_optimized(arr: Vec<i32>) -> Vec<i32> {
    if arr.is_empty() {
        return arr;
    }
    
    // Handle negative numbers
    let min_val = *arr.iter().min().unwrap();
    let offset = if min_val < 0 { -min_val } else { 0 };
    
    let positive_arr: Vec<i32> = arr.iter().map(|x| x + offset).collect();
    let max_val = *positive_arr.iter().max().unwrap();
    
    if max_val == 0 {
        return arr;
    }
    
    // Use counting sort approach for efficiency
    let mut count = vec![0; (max_val + 1) as usize];
    
    for &num in &positive_arr {
        count[num as usize] += 1;
    }
    
    let mut result = Vec::new();
    for (value, &freq) in count.iter().enumerate() {
        for _ in 0..freq {
            result.push(value as i32 - offset);
        }
    }
    
    result
}