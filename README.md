# Sorthos - Sorting Algorithms Visualizer

A comprehensive sorting algorithm visualization tool built with Rust and egui. SortHos provides interactive visual representations of various sorting algorithms, allowing users to observe and understand how different sorting techniques work in real-time.

## Features

- **Real-time Visualization**: Watch sorting algorithms work step-by-step with visual feedback
- **Comprehensive Algorithm Collection**: 20+ sorting algorithms implemented and tested
- **Interactive GUI**: User-friendly interface with algorithm selection and customization options
- **Performance Insights**: Compare different algorithms and their time complexities
- **Dark/Light Theme**: Toggle between visual themes for comfortable viewing
- **Educational Tool**: Perfect for learning and teaching sorting algorithm concepts

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo package manager

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd sorthos
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run
```

### Running Tests

Execute the comprehensive test suite:
```bash
cargo test
```

## Implemented Algorithms

All sorting algorithms have been verified and are working correctly:

### 1. **Burst Sort**
- **File**: `src/Sorting/burstsort.rs`
- **Status**: Functional
- **Description**: Hybrid algorithm using counting sort for small ranges and quicksort for large ranges
- **Performance**: O(n + k) for small ranges, O(n log n) for large ranges

### 2. **Intro Sort**
- **File**: `src/Sorting/Introsort.rs`
- **Status**: Functional
- **Description**: Hybrid algorithm combining quicksort, heapsort, and insertion sort
- **Performance**: O(n log n) guaranteed worst-case

### 3. **Quad Sort**
- **File**: `src/Sorting/quadsort.rs`
- **Status**: Functional
- **Description**: Divide-and-conquer algorithm that splits arrays into 4 parts
- **Performance**: O(n log n) average case

### 4. **Spaghetti Sort**
- **File**: `src/Sorting/spaghettisort.rs`
- **Status**: Functional
- **Description**: Gravity-based sorting algorithm simulating falling spaghetti
- **Performance**: O(n × max_value)
- **Note**: Recently fixed gravity simulation logic and height counting

### 5. **Spaghetti Sort (Optimized)**
- **File**: `src/Sorting/spaghettisort.rs`
- **Status**: Functional
- **Description**: Optimized version using counting sort approach
- **Performance**: O(n + k) where k is the range of values

### 6. **Block Merge Sort**
- **File**: `src/core/sorting.rs`
- **Status**: Functional
- **Description**: Bottom-up merge sort implementation with visual feedback
- **Performance**: O(n log n)

### 7. **Bogo Sort**
- **File**: `src/core/sorting.rs`
- **Status**: Functional
- **Description**: Randomized sorting algorithm (shuffle until sorted)
- **Performance**: O((n+1)!) worst case, O(n) best case
- **Note**: Only practical for very small arrays

### 8. **Bozo Sort**
- **File**: `src/core/sorting.rs`
- **Status**: Functional
- **Description**: Randomized sorting algorithm (random swaps until sorted)
- **Performance**: O(n!) expected case
- **Note**: Only practical for very small arrays

### 9. **Stooge Sort**
- **File**: `src/core/sorting.rs`
- **Status**: Functional
- **Description**: Recursive divide-and-conquer with unique 2/3 partitioning
- **Performance**: O(n^2.7) - very slow

### 10. **Slow Sort**
- **File**: `src/core/sorting.rs`
- **Status**: Functional
- **Description**: Intentionally slow recursive algorithm
- **Performance**: O(n^(log n)) - extremely slow

### 11. **Counting Sort**
- **File**: `src/Sorting/counting_sort_visual.rs`
- **Status**: Functional
- **Description**: Non-comparison sorting algorithm with visual feedback
- **Features**: Safety checks for large values to prevent memory issues
- **Performance**: O(n + k) where k is the range of values

### 12. **Radix Sort**
- **File**: `src/Sorting/radix_sort_visual.rs`
- **Status**: Functional
- **Description**: Non-comparison sorting algorithm processing digits
- **Features**: Safety checks and digit-based color coding for visualization
- **Performance**: O(d × (n + k)) where d is number of digits

### 13. **Shell Sort**
- **File**: `src/Sorting/shell_sort_visual.rs`
- **Status**: Functional
- **Description**: Gap-based insertion sort with decreasing gap sizes
- **Features**: Visual gap group highlighting
- **Performance**: O(n^1.25) to O(n^1.5) depending on gap sequence

### Additional Classic Algorithms

The project also includes implementations of fundamental sorting algorithms:
- **Bubble Sort**: Classic comparison-based algorithm
- **Selection Sort**: Simple selection-based sorting
- **Insertion Sort**: Efficient for small datasets
- **Quick Sort**: Divide-and-conquer with partitioning
- **Merge Sort**: Stable divide-and-conquer algorithm
- **Heap Sort**: Heap-based sorting algorithm
- **Cocktail Sort**: Bidirectional bubble sort
- **Gnome Sort**: Simple position-based algorithm
- **Tim Sort**: Hybrid stable sorting algorithm

## Testing and Quality Assurance

### Test Coverage
- **29 comprehensive tests** covering all algorithms
- **Multiple test scenarios**: Random arrays, sorted arrays, reverse sorted, duplicates, edge cases
- **Performance testing**: Various array sizes and complexity verification
- **Stability testing**: Element preservation and duplicate handling
- **Edge case handling**: Empty arrays, single elements, negative numbers

### Test Results
All tests pass successfully, ensuring reliability and correctness of every implemented algorithm.

### Running Specific Tests
```bash
# Run algorithm-specific tests
cargo test algorithm_tests

# Run all tests with output
cargo test -- --nocapture

# Run specific algorithm test
cargo test test_spaghetti_sort
```

## Performance Categories

### Fast Algorithms (Practical for Large Datasets)
- Intro Sort, Quad Sort, Block Merge Sort
- Counting Sort, Radix Sort, Shell Sort, Burst Sort
- **Complexity**: O(n log n) or better

### Moderate Algorithms (Good for Medium Datasets)
- Spaghetti Sort (Optimized), Tim Sort
- **Complexity**: O(n + k) to O(n log n)

### Educational Algorithms (Small Datasets Only)
- Spaghetti Sort, Stooge Sort, Slow Sort
- **Complexity**: O(n²) to O(n^(log n))

### Probabilistic Algorithms (Unpredictable Timing)
- Bogo Sort, Bozo Sort
- **Complexity**: Factorial or worse

## Project Structure

```
sorthos/
├── src/
│   ├── Sorting/           # Individual algorithm implementations
│   ├── core/              # Core functionality and main sorting logic
│   ├── gui/               # User interface components
│   ├── assets/            # Visual assets and resources
│   └── tests.rs           # Comprehensive test suites
├── target/                # Compiled binaries and dependencies
└── Cargo.toml             # Project configuration and dependencies
```

## License

BSD-3-Clause

## Libraries Used

- **egui**: Immediate mode GUI framework
- **eframe**: Application framework for egui
