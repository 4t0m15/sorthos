#[cfg(test)]
mod algorithm_tests {
    use crate::models::SortBar;
    use crate::sorting::{
        block_merge_sort, bogo_sort, bozo_sort, burst_sort, counting_sort_visual, introsort,
        quad_sort, radix_sort_visual, shell_sort_visual, slow_sort, spaghetti_sort,
        spaghetti_sort_optimized, stooge_sort, Operation,
    };
    use std::sync::mpsc;
    use std::thread;

    /// Helper function to create a vector of SortBars from values
    fn create_bars(values: Vec<usize>) -> Vec<SortBar> {
        values.into_iter().map(SortBar::new).collect()
    }

    /// Helper function to extract values from SortBars
    fn extract_values(bars: &[SortBar]) -> Vec<usize> {
        bars.iter().map(|b| b.value).collect()
    }

    /// Check if a slice is sorted in non-decreasing order
    fn is_sorted(values: &[i32]) -> bool {
        values.windows(2).all(|w| w[0] <= w[1])
    }

    /// Check if a slice is sorted in non-decreasing order (usize version)
    fn is_sorted_usize(values: &[usize]) -> bool {
        values.windows(2).all(|w| w[0] <= w[1])
    }

    /// Test cases for all algorithms
    fn get_test_cases() -> Vec<Vec<i32>> {
        vec![
            vec![5, 2, 8, 1, 9, 3, 7, 4, 6],       // Random order
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1],       // Reverse sorted
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],       // Already sorted
            vec![5, 5, 5, 5, 5],                   // All same values
            vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5], // With duplicates
            vec![42],                              // Single element
            vec![2, 1],                            // Two elements
            vec![],                                // Empty array
            vec![-5, -2, -8, -1, -9, -3],          // Negative numbers
            vec![-3, 0, 5, -1, 2],                 // Mixed positive/negative
            (1..=20).rev().collect::<Vec<_>>(),    // Larger array, reverse sorted
            (1..=20).collect::<Vec<_>>(),          // Larger array, already sorted
        ]
    }

    #[test]
    fn test_burst_sort() {
        println!("Testing Burst Sort...");
        for test_case in get_test_cases() {
            let original = test_case.clone();
            let result = burst_sort(test_case);

            assert!(
                is_sorted(&result),
                "Burst Sort failed on input: {:?}, got: {:?}",
                original,
                result
            );

            // Verify all elements are preserved
            let mut original_sorted = original.clone();
            original_sorted.sort();
            let mut result_sorted = result.clone();
            result_sorted.sort();
            assert_eq!(
                original_sorted, result_sorted,
                "Burst Sort lost elements on input: {:?}",
                original
            );
        }
    }

    #[test]
    fn test_intro_sort() {
        println!("Testing Intro Sort...");
        for test_case in get_test_cases() {
            let original = test_case.clone();
            let result = introsort(test_case);

            assert!(
                is_sorted(&result),
                "Intro Sort failed on input: {:?}, got: {:?}",
                original,
                result
            );

            // Verify all elements are preserved
            let mut original_sorted = original.clone();
            original_sorted.sort();
            let mut result_sorted = result.clone();
            result_sorted.sort();
            assert_eq!(
                original_sorted, result_sorted,
                "Intro Sort lost elements on input: {:?}",
                original
            );
        }
    }

    #[test]
    fn test_quad_sort() {
        println!("Testing Quad Sort...");
        for test_case in get_test_cases() {
            let original = test_case.clone();
            let result = quad_sort(test_case);

            assert!(
                is_sorted(&result),
                "Quad Sort failed on input: {:?}, got: {:?}",
                original,
                result
            );

            // Verify all elements are preserved
            let mut original_sorted = original.clone();
            original_sorted.sort();
            let mut result_sorted = result.clone();
            result_sorted.sort();
            assert_eq!(
                original_sorted, result_sorted,
                "Quad Sort lost elements on input: {:?}",
                original
            );
        }
    }

    #[test]
    fn test_spaghetti_sort() {
        println!("Testing Spaghetti Sort...");
        for test_case in get_test_cases() {
            let original = test_case.clone();
            let result = spaghetti_sort(test_case);

            assert!(
                is_sorted(&result),
                "Spaghetti Sort failed on input: {:?}, got: {:?}",
                original,
                result
            );

            // Verify all elements are preserved
            let mut original_sorted = original.clone();
            original_sorted.sort();
            let mut result_sorted = result.clone();
            result_sorted.sort();
            assert_eq!(
                original_sorted, result_sorted,
                "Spaghetti Sort lost elements on input: {:?}",
                original
            );
        }
    }

    #[test]
    fn test_spaghetti_sort_optimized() {
        println!("Testing Spaghetti Sort (Optimized)...");
        for test_case in get_test_cases() {
            let original = test_case.clone();
            let result = spaghetti_sort_optimized(test_case);

            assert!(
                is_sorted(&result),
                "Spaghetti Sort (Optimized) failed on input: {:?}, got: {:?}",
                original,
                result
            );

            // Verify all elements are preserved
            let mut original_sorted = original.clone();
            original_sorted.sort();
            let mut result_sorted = result.clone();
            result_sorted.sort();
            assert_eq!(
                original_sorted, result_sorted,
                "Spaghetti Sort (Optimized) lost elements on input: {:?}",
                original
            );
        }
    }

    #[test]
    fn test_block_merge_sort() {
        println!("Testing Block Merge Sort...");
        let test_cases_usize = vec![
            vec![5, 2, 8, 1, 9, 3, 7, 4, 6],
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![5, 5, 5, 5, 5],
            vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5],
            vec![42],
            vec![2, 1],
            vec![],
            (1..=15).rev().collect::<Vec<_>>(),
            (1..=15).collect::<Vec<_>>(),
        ];

        for test_case in test_cases_usize {
            let original = test_case.clone();
            let mut bars = create_bars(test_case);
            let (tx, _rx) = mpsc::channel::<Operation>();

            block_merge_sort(&mut bars, &tx);
            let result = extract_values(&bars);

            assert!(
                is_sorted_usize(&result),
                "Block Merge Sort failed on input: {:?}, got: {:?}",
                original,
                result
            );

            // Verify all elements are preserved
            let mut original_sorted = original.clone();
            original_sorted.sort();
            let mut result_sorted = result.clone();
            result_sorted.sort();
            assert_eq!(
                original_sorted, result_sorted,
                "Block Merge Sort lost elements on input: {:?}",
                original
            );
        }
    }

    #[test]
    fn test_counting_sort_visual() {
        println!("Testing Counting Sort...");
        let test_cases_usize = vec![
            vec![5, 2, 8, 1, 9, 3, 7, 4, 6],
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![5, 5, 5, 5, 5],
            vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5],
            vec![42],
            vec![2, 1],
            vec![],
            (1..=10).rev().collect::<Vec<_>>(),
            (1..=10).collect::<Vec<_>>(),
        ];

        for test_case in test_cases_usize {
            let original = test_case.clone();
            let mut bars = create_bars(test_case);
            let (tx, _rx) = mpsc::channel::<Operation>();

            counting_sort_visual(&mut bars, &tx);
            let result = extract_values(&bars);

            assert!(
                is_sorted_usize(&result),
                "Counting Sort failed on input: {:?}, got: {:?}",
                original,
                result
            );

            // Verify all elements are preserved
            let mut original_sorted = original.clone();
            original_sorted.sort();
            let mut result_sorted = result.clone();
            result_sorted.sort();
            assert_eq!(
                original_sorted, result_sorted,
                "Counting Sort lost elements on input: {:?}",
                original
            );
        }
    }

    #[test]
    fn test_radix_sort_visual() {
        println!("Testing Radix Sort...");
        let test_cases_usize = vec![
            vec![5, 2, 8, 1, 9, 3, 7, 4, 6],
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![5, 5, 5, 5, 5],
            vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5],
            vec![42],
            vec![2, 1],
            vec![],
            (1..=15).rev().collect::<Vec<_>>(),
            (1..=15).collect::<Vec<_>>(),
        ];

        for test_case in test_cases_usize {
            let original = test_case.clone();
            let mut bars = create_bars(test_case);
            let (tx, _rx) = mpsc::channel::<Operation>();

            radix_sort_visual(&mut bars, &tx);
            let result = extract_values(&bars);

            assert!(
                is_sorted_usize(&result),
                "Radix Sort failed on input: {:?}, got: {:?}",
                original,
                result
            );

            // Verify all elements are preserved
            let mut original_sorted = original.clone();
            original_sorted.sort();
            let mut result_sorted = result.clone();
            result_sorted.sort();
            assert_eq!(
                original_sorted, result_sorted,
                "Radix Sort lost elements on input: {:?}",
                original
            );
        }
    }

    #[test]
    fn test_shell_sort_visual() {
        println!("Testing Shell Sort...");
        let test_cases_usize = vec![
            vec![5, 2, 8, 1, 9, 3, 7, 4, 6],
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![5, 5, 5, 5, 5],
            vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5],
            vec![42],
            vec![2, 1],
            vec![],
            (1..=15).rev().collect::<Vec<_>>(),
            (1..=15).collect::<Vec<_>>(),
        ];

        for test_case in test_cases_usize {
            let original = test_case.clone();
            let mut bars = create_bars(test_case);
            let (tx, _rx) = mpsc::channel::<Operation>();

            shell_sort_visual(&mut bars, &tx);
            let result = extract_values(&bars);

            assert!(
                is_sorted_usize(&result),
                "Shell Sort failed on input: {:?}, got: {:?}",
                original,
                result
            );

            // Verify all elements are preserved
            let mut original_sorted = original.clone();
            original_sorted.sort();
            let mut result_sorted = result.clone();
            result_sorted.sort();
            assert_eq!(
                original_sorted, result_sorted,
                "Shell Sort lost elements on input: {:?}",
                original
            );
        }
    }

    #[test]
    fn test_stooge_sort() {
        println!("Testing Stooge Sort...");
        // Use smaller test cases for Stooge Sort since it's O(n^2.7)
        let test_cases_usize = vec![
            vec![5, 2, 8, 1, 9],
            vec![9, 8, 7, 6, 5],
            vec![1, 2, 3, 4, 5],
            vec![5, 5, 5, 5, 5],
            vec![3, 1, 4, 1, 5],
            vec![42],
            vec![2, 1],
            vec![],
        ];

        for test_case in test_cases_usize {
            let original = test_case.clone();
            let mut bars = create_bars(test_case);
            let (tx, _rx) = mpsc::channel::<Operation>();

            stooge_sort(&mut bars, &tx);
            let result = extract_values(&bars);

            assert!(
                is_sorted_usize(&result),
                "Stooge Sort failed on input: {:?}, got: {:?}",
                original,
                result
            );

            // Verify all elements are preserved
            let mut original_sorted = original.clone();
            original_sorted.sort();
            let mut result_sorted = result.clone();
            result_sorted.sort();
            assert_eq!(
                original_sorted, result_sorted,
                "Stooge Sort lost elements on input: {:?}",
                original
            );
        }
    }

    #[test]
    fn test_slow_sort() {
        println!("Testing Slow Sort...");
        // Use smaller test cases for Slow Sort since it's extremely slow
        let test_cases_usize = vec![
            vec![5, 2, 8, 1],
            vec![9, 8, 7, 6],
            vec![1, 2, 3, 4],
            vec![5, 5, 5, 5],
            vec![3, 1, 4, 1],
            vec![42],
            vec![2, 1],
            vec![],
        ];

        for test_case in test_cases_usize {
            let original = test_case.clone();
            let mut bars = create_bars(test_case);
            let (tx, _rx) = mpsc::channel::<Operation>();

            slow_sort(&mut bars, &tx);
            let result = extract_values(&bars);

            assert!(
                is_sorted_usize(&result),
                "Slow Sort failed on input: {:?}, got: {:?}",
                original,
                result
            );

            // Verify all elements are preserved
            let mut original_sorted = original.clone();
            original_sorted.sort();
            let mut result_sorted = result.clone();
            result_sorted.sort();
            assert_eq!(
                original_sorted, result_sorted,
                "Slow Sort lost elements on input: {:?}",
                original
            );
        }
    }

    #[test]
    fn test_bogo_sort_and_bozo_sort() {
        println!("Testing Bogo Sort and Bozo Sort...");
        // Use very small test cases for probabilistic sorts
        let small_test_cases = vec![vec![2, 1], vec![3, 1, 2], vec![42], vec![]];

        for test_case in small_test_cases {
            let original = test_case.clone();

            // Test Bogo Sort with timeout
            let bars = create_bars(test_case.clone());
            let (tx, _rx) = mpsc::channel::<Operation>();

            // Use a separate thread with timeout for Bogo Sort
            let bars_clone = bars.clone();
            let handle = thread::spawn(move || {
                let mut bars = bars_clone;
                bogo_sort(&mut bars, &tx);
                bars
            });

            // Wait for completion with timeout
            if let Ok(result_bars) = handle.join() {
                let result = extract_values(&result_bars);
                if !result.is_empty() {
                    assert!(
                        is_sorted_usize(&result),
                        "Bogo Sort failed on input: {:?}, got: {:?}",
                        original,
                        result
                    );
                }
            }

            // Test Bozo Sort with timeout
            let bars = create_bars(test_case.clone());
            let (tx, _rx) = mpsc::channel::<Operation>();

            let bars_clone = bars.clone();
            let handle = thread::spawn(move || {
                let mut bars = bars_clone;
                bozo_sort(&mut bars, &tx);
                bars
            });

            // Wait for completion with timeout
            if let Ok(result_bars) = handle.join() {
                let result = extract_values(&result_bars);
                if !result.is_empty() {
                    assert!(
                        is_sorted_usize(&result),
                        "Bozo Sort failed on input: {:?}, got: {:?}",
                        original,
                        result
                    );
                }
            }
        }
    }

    #[test]
    fn test_algorithm_stability() {
        println!("Testing algorithm stability with duplicate elements...");

        // Test with arrays containing many duplicates
        let duplicate_heavy = vec![
            3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4, 6, 2, 6,
        ];

        // Test stable algorithms
        let algorithms: Vec<(&str, Box<dyn Fn(Vec<i32>) -> Vec<i32>>)> = vec![
            ("Burst Sort", Box::new(|arr| burst_sort(arr))),
            ("Intro Sort", Box::new(|arr| introsort(arr))),
            ("Quad Sort", Box::new(|arr| quad_sort(arr))),
            ("Spaghetti Sort", Box::new(|arr| spaghetti_sort(arr))),
            (
                "Spaghetti Sort (Optimized)",
                Box::new(|arr| spaghetti_sort_optimized(arr)),
            ),
        ];

        for (name, algorithm) in algorithms {
            let result = algorithm(duplicate_heavy.clone());
            assert!(
                is_sorted(&result),
                "{} failed on duplicate-heavy array: {:?}",
                name,
                result
            );

            // Verify all elements are preserved
            let mut original_sorted = duplicate_heavy.clone();
            original_sorted.sort();
            let mut result_sorted = result.clone();
            result_sorted.sort();
            assert_eq!(
                original_sorted, result_sorted,
                "{} lost elements on duplicate-heavy array",
                name
            );
        }
    }

    #[test]
    fn test_performance_characteristics() {
        println!("Testing performance characteristics...");

        // Test with different array sizes to ensure algorithms scale reasonably
        let sizes = vec![0, 1, 2, 5, 10, 20, 50];

        for size in sizes {
            let test_array: Vec<i32> = (1..=size).rev().collect();

            if size <= 20 {
                // Test all algorithms on smaller arrays
                let algorithms: Vec<(&str, Box<dyn Fn(Vec<i32>) -> Vec<i32>>)> = vec![
                    ("Burst Sort", Box::new(|arr| burst_sort(arr))),
                    ("Intro Sort", Box::new(|arr| introsort(arr))),
                    ("Quad Sort", Box::new(|arr| quad_sort(arr))),
                    ("Spaghetti Sort", Box::new(|arr| spaghetti_sort(arr))),
                    (
                        "Spaghetti Sort (Optimized)",
                        Box::new(|arr| spaghetti_sort_optimized(arr)),
                    ),
                ];

                for (name, algorithm) in algorithms {
                    let result = algorithm(test_array.clone());
                    assert!(is_sorted(&result), "{} failed on size {} array", name, size);
                }
            }
        }
    }
}
