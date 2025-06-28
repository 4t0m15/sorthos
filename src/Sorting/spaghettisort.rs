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
        return vec![0 - offset; arr.len()];
    }

    // Create a matrix to represent the gravity sort
    // Rows represent height levels, columns represent array positions
    let mut matrix = vec![vec![false; positive_arr.len()]; max_val as usize];

    // Mark the beads - fill from bottom up (gravity convention)
    for (col, &height) in positive_arr.iter().enumerate() {
        for level in 0..height as usize {
            matrix[max_val as usize - 1 - level][col] = true;
        }
    }

    // Apply gravity - beads fall down and settle to the right
    for level in (0..max_val as usize).rev() {
        // Count beads at this level
        let mut bead_count = 0;
        for col in 0..positive_arr.len() {
            if matrix[level][col] {
                matrix[level][col] = false; // Remove bead
                bead_count += 1;
            }
        }

        // Place beads at rightmost positions at this level
        for i in 0..bead_count {
            let col = positive_arr.len() - 1 - i;
            matrix[level][col] = true;
        }
    }

    // Count the heights after gravity (from bottom up)
    let mut result = Vec::new();
    for col in 0..positive_arr.len() {
        let mut height = 0;
        for level in (0..max_val as usize).rev() {
            if matrix[level][col] {
                height += 1;
            } else {
                break; // No more beads above
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
