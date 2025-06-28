#[path = "../Sorting/bubble_sort.rs"]
mod bubble_sort;

#[path = "../Sorting/cocktail_sort_visual.rs"]
mod cocktail_sort_visual;
#[path = "../Sorting/counting_sort_visual.rs"]
mod counting_sort_visual;
#[path = "../Sorting/gnome_sort_visual.rs"]
mod gnome_sort_visual;
#[path = "../Sorting/heap_sort_visual.rs"]
mod heap_sort_visual;
#[path = "../Sorting/insertion_sort.rs"]
mod insertion_sort;

#[path = "../Sorting/merge_sort_visual.rs"]
mod merge_sort_visual;

#[path = "../Sorting/quicksort_visual.rs"]
mod quicksort_visual;
#[path = "../Sorting/radix_sort_visual.rs"]
mod radix_sort_visual;
#[path = "../Sorting/selection_sort.rs"]
mod selection_sort;
#[path = "../Sorting/shell_sort_visual.rs"]
mod shell_sort_visual;
#[path = "../Sorting/sort_utils.rs"]
mod sort_utils;

#[path = "../Sorting/timsort.rs"]
mod timsort;

use crate::models::SortBar;
pub use bubble_sort::bubble_sort;

pub use cocktail_sort_visual::cocktail_sort_visual;
pub use counting_sort_visual::counting_sort_visual;
use eframe::egui::Color32;
pub use gnome_sort_visual::gnome_sort_visual;
pub use heap_sort_visual::heap_sort_visual;
pub use insertion_sort::insertion_sort;

pub use merge_sort_visual::merge_sort_visual;

pub use quicksort_visual::quick_sort_visual;
pub use radix_sort_visual::radix_sort_visual;
use rand::{thread_rng, Rng};
pub use selection_sort::selection_sort;
pub use shell_sort_visual::shell_sort_visual;

use std::fmt;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
pub use timsort::tim_sort;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortingAlgorithm {
    #[default]
    Bubble,
    Selection,
    Insertion,
    QuickVisual,
    MergeSort,
    HeapSort,
    CountingSort,
    RadixSort,
    ShellSort,
    CocktailSort,
    GnomeSort,

    TimSort,
    BlockMergeSort,
    BogoSort,
}

impl SortingAlgorithm {
    pub fn all() -> &'static [SortingAlgorithm] {
        &[
            SortingAlgorithm::Bubble,
            SortingAlgorithm::Selection,
            SortingAlgorithm::Insertion,
            SortingAlgorithm::QuickVisual,
            SortingAlgorithm::MergeSort,
            SortingAlgorithm::HeapSort,
            SortingAlgorithm::CountingSort,
            SortingAlgorithm::RadixSort,
            SortingAlgorithm::ShellSort,
            SortingAlgorithm::CocktailSort,
            SortingAlgorithm::GnomeSort,
            SortingAlgorithm::TimSort,
            SortingAlgorithm::BlockMergeSort,
            SortingAlgorithm::BogoSort,
        ]
    }
}

impl fmt::Display for SortingAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            SortingAlgorithm::Bubble => "Bubble Sort",
            SortingAlgorithm::Selection => "Selection Sort",
            SortingAlgorithm::Insertion => "Insertion Sort",
            SortingAlgorithm::QuickVisual => "Quick Sort (Visual)",
            SortingAlgorithm::MergeSort => "Merge Sort",
            SortingAlgorithm::HeapSort => "Heap Sort",
            SortingAlgorithm::CountingSort => "Counting Sort",
            SortingAlgorithm::RadixSort => "Radix Sort",
            SortingAlgorithm::ShellSort => "Shell Sort",
            SortingAlgorithm::CocktailSort => "Cocktail Sort",
            SortingAlgorithm::GnomeSort => "Gnome Sort",

            SortingAlgorithm::TimSort => "Tim Sort",
            SortingAlgorithm::BlockMergeSort => "Block Merge Sort",
            SortingAlgorithm::BogoSort => "Bogo Sort",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    Compare(usize, usize),
    Swap(usize, usize),
    SetColor(usize, Color32),
    Overwrite(usize, SortBar), // The new operation to directly place a bar
    Done,
}

/// Copies sorted `values` back into `bars` and repaints them white.

/// Returns `true` if the slice is in non‑decreasing order.

pub fn start_sort(
    algorithm: SortingAlgorithm,
    mut bars: Vec<SortBar>,
    tx: mpsc::Sender<Operation>,
) {
    println!(
        "[DEBUG] start_sort: Spawning thread for algorithm: {:?}",
        algorithm
    );
    thread::spawn(move || {
        let len = bars.len();

        // Handle edge cases - arrays with 0 or 1 elements are already sorted
        if len <= 1 {
            let _ = tx.send(Operation::Done);
            println!("[DEBUG] start_sort: Array len <= 1, sending Done.");
            return;
        }

        match algorithm {
            SortingAlgorithm::Bubble => {
                println!("[DEBUG] start_sort: Running bubble_sort");
                bubble_sort(&mut bars, &tx)
            }
            SortingAlgorithm::Selection => {
                println!("[DEBUG] start_sort: Running selection_sort");
                selection_sort(&mut bars, &tx)
            }
            SortingAlgorithm::Insertion => {
                println!("[DEBUG] start_sort: Running insertion_sort");
                insertion_sort(&mut bars, &tx)
            }
            SortingAlgorithm::QuickVisual => {
                println!("[DEBUG] start_sort: Running quick_sort_visual");
                quick_sort_visual(&mut bars, &tx)
            }
            SortingAlgorithm::MergeSort => {
                println!("[DEBUG] start_sort: Running merge_sort_visual");
                merge_sort_visual(&mut bars, &tx)
            }
            SortingAlgorithm::HeapSort => {
                println!("[DEBUG] start_sort: Running heap_sort_visual");
                heap_sort_visual(&mut bars, &tx)
            }
            SortingAlgorithm::CountingSort => {
                println!("[DEBUG] start_sort: Running counting_sort_visual");
                counting_sort_visual(&mut bars, &tx)
            }
            SortingAlgorithm::RadixSort => {
                println!("[DEBUG] start_sort: Running radix_sort_visual");
                radix_sort_visual(&mut bars, &tx)
            }
            SortingAlgorithm::ShellSort => {
                println!("[DEBUG] start_sort: Running shell_sort_visual");
                shell_sort_visual(&mut bars, &tx)
            }
            SortingAlgorithm::CocktailSort => {
                println!("[DEBUG] start_sort: Running cocktail_sort_visual");
                cocktail_sort_visual(&mut bars, &tx)
            }
            SortingAlgorithm::GnomeSort => {
                println!("[DEBUG] start_sort: Running gnome_sort_visual");
                gnome_sort_visual(&mut bars, &tx)
            }

            SortingAlgorithm::TimSort => {
                println!("[DEBUG] start_sort: Running tim_sort");
                tim_sort(&mut bars, &tx);
            }
            SortingAlgorithm::BlockMergeSort => {
                println!("[DEBUG] start_sort: Running block_merge_sort");
                block_merge_sort(&mut bars, &tx);
            }
            SortingAlgorithm::BogoSort => {
                println!("[DEBUG] start_sort: Running bogo_sort");
                bogo_sort(&mut bars, &tx);
            }
        }
        let _ = tx.send(Operation::Done);
        println!("[DEBUG] start_sort: Sorting thread finished, sent Done.");
    });
}

// ---------- Block Merge Sort (bottom‑up visual merge sort) ----------
fn merge_subarrays(
    bars: &mut Vec<SortBar>,
    left: usize,
    mid: usize,
    right: usize,
    tx: &mpsc::Sender<Operation>,
) {
    // Copy values into a temp vec
    let temp: Vec<usize> = bars[left..=right].iter().map(|b| b.value).collect();

    let mut i = 0;
    let mut j = mid - left + 1;
    let mut k = left;

    while i <= mid - left && j < temp.len() {
        let idx_left = left + i;
        let idx_right = left + j;

        let _ = tx.send(Operation::Compare(idx_left, idx_right));
        thread::sleep(Duration::from_millis(6));

        if temp[i] <= temp[j] {
            // Already in correct place
            bars[k].value = temp[i];
            let _ = tx.send(Operation::SetColor(k, Color32::WHITE));
            i += 1;
        } else {
            // value at j is smaller – perform “insertion‑style” swaps to move it
            let _ = tx.send(Operation::Swap(idx_left, idx_right));
            bars[k].value = temp[j];
            let _ = tx.send(Operation::SetColor(k, Color32::WHITE));
            j += 1;
        }
        k += 1;
    }

    // Copy remaining elements
    while i <= mid - left {
        bars[k].value = temp[i];
        let _ = tx.send(Operation::SetColor(k, Color32::WHITE));
        i += 1;
        k += 1;
    }
    while j < temp.len() {
        bars[k].value = temp[j];
        let _ = tx.send(Operation::SetColor(k, Color32::WHITE));
        j += 1;
        k += 1;
    }
}

pub fn block_merge_sort(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<Operation>) {
    let n = bars.len();
    let mut curr_size = 1;
    while curr_size < n {
        let mut left_start = 0;
        while left_start < n - curr_size {
            let mid = left_start + curr_size - 1;
            let right_end = std::cmp::min(left_start + 2 * curr_size - 1, n - 1);
            merge_subarrays(bars, left_start, mid, right_end, tx);
            left_start += 2 * curr_size;
        }
        curr_size *= 2; // move to next segment size
    }

    for idx in 0..n {
        let _ = tx.send(Operation::SetColor(idx, Color32::WHITE));
    }
}

// ---------- Bogo Sort ----------
pub fn bogo_sort(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<Operation>) {
    use std::time::Duration;
    let mut rng = thread_rng();
    let len = bars.len();

    fn bars_are_sorted(bars: &[SortBar]) -> bool {
        bars.windows(2).all(|w| w[0].value <= w[1].value)
    }

    while !bars_are_sorted(bars) {
        // Fisher-Yates shuffle with visual feedback
        for i in (1..len).rev() {
            let j = rng.gen_range(0..=i);
            if i != j {
                bars.swap(i, j);
                let _ = tx.send(Operation::Swap(i, j));
                let _ = tx.send(Operation::SetColor(i, Color32::YELLOW));
                let _ = tx.send(Operation::SetColor(j, Color32::YELLOW));
                thread::sleep(Duration::from_millis(15));
                let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
                let _ = tx.send(Operation::SetColor(j, Color32::WHITE));
            }
        }
        thread::sleep(Duration::from_millis(60));
    }

    // Final sorted state
    for i in 0..len {
        let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
    }
}

// (Removed Bozo Sort, Stooge Sort, and Slow Sort implementations)
