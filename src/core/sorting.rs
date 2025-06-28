#[path = "../Sorting/bubble_sort.rs"]
mod bubble_sort;
#[path = "../Sorting/burstsort.rs"]
mod burstsort;
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
#[path = "../Sorting/Introsort.rs"]
mod introsort;
#[path = "../Sorting/merge_sort_visual.rs"]
mod merge_sort_visual;
#[path = "../Sorting/quadsort.rs"]
mod quadsort;
#[path = "../Sorting/quicksort.rs"]
mod quicksort_numeric;
#[path = "../Sorting/quicksort_visual.rs"]
mod quicksort_visual;
#[path = "../Sorting/radix_sort_visual.rs"]
mod radix_sort_visual;
#[path = "../Sorting/selection_sort.rs"]
mod selection_sort;
#[path = "../Sorting/shell_sort_visual.rs"]
mod shell_sort_visual;
#[path = "../Sorting/spaghettisort.rs"]
mod spaghettisort;
#[path = "../Sorting/timsort.rs"]
mod timsort;

use crate::models::SortBar;
use eframe::egui::Color32;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::fmt;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub use bubble_sort::bubble_sort;
pub use burstsort::burst_sort;
pub use cocktail_sort_visual::cocktail_sort_visual;
pub use counting_sort_visual::counting_sort_visual;
pub use gnome_sort_visual::gnome_sort_visual;
pub use heap_sort_visual::heap_sort_visual;
pub use insertion_sort::insertion_sort;
pub use introsort::introsort;
pub use merge_sort_visual::merge_sort_visual;
pub use quadsort::quad_sort;
pub use quicksort_numeric::quick_sort as quick_sort_numeric;
pub use quicksort_visual::quick_sort_visual;
pub use radix_sort_visual::radix_sort_visual;
pub use selection_sort::selection_sort;
pub use shell_sort_visual::shell_sort_visual;
pub use spaghettisort::spaghetti_sort;
pub use spaghettisort::spaghetti_sort_optimized;
pub use timsort::tim_sort;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortingAlgorithm {
    Bubble,
    Selection,
    Insertion,
    Quick,
    QuickVisual,
    MergeSort,
    HeapSort,
    CountingSort,
    RadixSort,
    ShellSort,
    CocktailSort,
    GnomeSort,
    Burst,
    Intro,
    Quad,
    QuickNumeric,
    Spaghetti,
    SpaghettiOpt,
    TimSort,
    BlockMergeSort,
    BogoSort,
    BozoSort,
    StoogeSort,
    SlowSort,
}

impl SortingAlgorithm {
    pub fn all() -> &'static [SortingAlgorithm] {
        &[
            SortingAlgorithm::Bubble,
            SortingAlgorithm::Selection,
            SortingAlgorithm::Insertion,
            SortingAlgorithm::Quick,
            SortingAlgorithm::QuickVisual,
            SortingAlgorithm::MergeSort,
            SortingAlgorithm::HeapSort,
            SortingAlgorithm::CountingSort,
            SortingAlgorithm::RadixSort,
            SortingAlgorithm::ShellSort,
            SortingAlgorithm::CocktailSort,
            SortingAlgorithm::GnomeSort,
            SortingAlgorithm::Burst,
            SortingAlgorithm::Intro,
            SortingAlgorithm::Quad,
            SortingAlgorithm::QuickNumeric,
            SortingAlgorithm::Spaghetti,
            SortingAlgorithm::SpaghettiOpt,
            SortingAlgorithm::TimSort,
            SortingAlgorithm::BlockMergeSort,
            SortingAlgorithm::BogoSort,
            SortingAlgorithm::BozoSort,
            SortingAlgorithm::StoogeSort,
            SortingAlgorithm::SlowSort,
        ]
    }
}

impl fmt::Display for SortingAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            SortingAlgorithm::Bubble => "Bubble Sort",
            SortingAlgorithm::Selection => "Selection Sort",
            SortingAlgorithm::Insertion => "Insertion Sort",
            SortingAlgorithm::Quick => "Quick Sort",
            SortingAlgorithm::QuickVisual => "Quick Sort (Visual)",
            SortingAlgorithm::MergeSort => "Merge Sort",
            SortingAlgorithm::HeapSort => "Heap Sort",
            SortingAlgorithm::CountingSort => "Counting Sort",
            SortingAlgorithm::RadixSort => "Radix Sort",
            SortingAlgorithm::ShellSort => "Shell Sort",
            SortingAlgorithm::CocktailSort => "Cocktail Sort",
            SortingAlgorithm::GnomeSort => "Gnome Sort",
            SortingAlgorithm::Burst => "Burst Sort",
            SortingAlgorithm::Intro => "Intro Sort",
            SortingAlgorithm::Quad => "Quad Sort",
            SortingAlgorithm::QuickNumeric => "Quick Sort (Numeric)",
            SortingAlgorithm::Spaghetti => "Spaghetti Sort",
            SortingAlgorithm::SpaghettiOpt => "Spaghetti Sort (Optimized)",
            SortingAlgorithm::TimSort => "Tim Sort",
            SortingAlgorithm::BlockMergeSort => "Block Merge Sort",
            SortingAlgorithm::BogoSort => "Bogo Sort",
            SortingAlgorithm::BozoSort => "Bozo Sort",
            SortingAlgorithm::StoogeSort => "Stooge Sort",
            SortingAlgorithm::SlowSort => "Slow Sort",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    Compare(usize, usize),
    Swap(usize, usize),
    SetColor(usize, Color32),
    Done,
}

/// Copies sorted `values` back into `bars` and repaints them white.
fn copy_values_to_bars(bars: &mut [SortBar], values: &[usize], tx: &mpsc::Sender<Operation>) {
    for (i, &val) in values.iter().enumerate() {
        bars[i].value = val;
        let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
    }
}

/// Returns `true` if the slice is in non‑decreasing order.
fn is_sorted(slice: &[usize]) -> bool {
    slice.windows(2).all(|w| w[0] <= w[1])
}

pub fn start_sort(
    algorithm: SortingAlgorithm,
    mut bars: Vec<SortBar>,
    tx: mpsc::Sender<Operation>,
) {
    thread::spawn(move || {
        let len = bars.len();

        // Handle edge cases - arrays with 0 or 1 elements are already sorted
        if len <= 1 {
            let _ = tx.send(Operation::Done);
            return;
        }

        match algorithm {
            SortingAlgorithm::Bubble => bubble_sort(&mut bars, &tx),
            SortingAlgorithm::Selection => selection_sort(&mut bars, &tx),
            SortingAlgorithm::Insertion => insertion_sort(&mut bars, &tx),
            SortingAlgorithm::Quick => {
                if len > 1 {
                    quick_sort(&mut bars, 0, len - 1, &tx);
                }
            }
            SortingAlgorithm::QuickVisual => quick_sort_visual(&mut bars, &tx),
            SortingAlgorithm::MergeSort => merge_sort_visual(&mut bars, &tx),
            SortingAlgorithm::HeapSort => heap_sort_visual(&mut bars, &tx),
            SortingAlgorithm::CountingSort => counting_sort_visual(&mut bars, &tx),
            SortingAlgorithm::RadixSort => radix_sort_visual(&mut bars, &tx),
            SortingAlgorithm::ShellSort => shell_sort_visual(&mut bars, &tx),
            SortingAlgorithm::CocktailSort => cocktail_sort_visual(&mut bars, &tx),
            SortingAlgorithm::GnomeSort => gnome_sort_visual(&mut bars, &tx),
            SortingAlgorithm::Burst => {
                let values: Vec<i32> = bars.iter().map(|b| b.value as i32).collect();
                let sorted = burst_sort(values);
                for (i, val) in sorted.into_iter().enumerate().take(bars.len()) {
                    bars[i].value = val as usize;
                    let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
                }
            }
            SortingAlgorithm::Intro => {
                let values: Vec<i32> = bars.iter().map(|b| b.value as i32).collect();
                let sorted = introsort(values);
                for (i, val) in sorted.into_iter().enumerate().take(bars.len()) {
                    bars[i].value = val as usize;
                    let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
                }
            }
            SortingAlgorithm::Quad => {
                let values: Vec<i32> = bars.iter().map(|b| b.value as i32).collect();
                let sorted = quad_sort(values);
                for (i, val) in sorted.into_iter().enumerate().take(bars.len()) {
                    bars[i].value = val as usize;
                    let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
                }
            }
            SortingAlgorithm::QuickNumeric => {
                let values: Vec<i32> = bars.iter().map(|b| b.value as i32).collect();
                let sorted = quick_sort_numeric(values);
                for (i, val) in sorted.into_iter().enumerate().take(bars.len()) {
                    bars[i].value = val as usize;
                    let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
                }
            }
            SortingAlgorithm::Spaghetti => {
                let values: Vec<i32> = bars.iter().map(|b| b.value as i32).collect();
                let sorted = spaghetti_sort(values);
                for (i, val) in sorted.into_iter().enumerate().take(bars.len()) {
                    bars[i].value = val as usize;
                    let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
                }
            }
            SortingAlgorithm::SpaghettiOpt => {
                let values: Vec<i32> = bars.iter().map(|b| b.value as i32).collect();
                let sorted = spaghetti_sort_optimized(values);
                for (i, val) in sorted.into_iter().enumerate().take(bars.len()) {
                    bars[i].value = val as usize;
                    let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
                }
            }
            SortingAlgorithm::TimSort => {
                tim_sort(&mut bars, &tx);
            }
            SortingAlgorithm::BlockMergeSort => {
                block_merge_sort(&mut bars, &tx);
            }
            SortingAlgorithm::BogoSort => {
                bogo_sort(&mut bars, &tx);
            }
            SortingAlgorithm::BozoSort => {
                bozo_sort(&mut bars, &tx);
            }
            SortingAlgorithm::StoogeSort => {
                stooge_sort(&mut bars, &tx);
            }
            SortingAlgorithm::SlowSort => {
                slow_sort(&mut bars, &tx);
            }
        }
        let _ = tx.send(Operation::Done);
    });
}

fn quick_sort(bars: &mut Vec<SortBar>, low: usize, high: usize, tx: &mpsc::Sender<Operation>) {
    if bars.is_empty() || low >= high || low >= bars.len() || high >= bars.len() {
        return;
    }

    let pi = partition(bars, low, high, tx);

    // Recursive calls with proper bounds checking
    if pi > low {
        quick_sort(bars, low, pi - 1, tx);
    }
    if pi + 1 <= high && pi + 1 < bars.len() {
        quick_sort(bars, pi + 1, high, tx);
    }
}

fn partition(
    bars: &mut Vec<SortBar>,
    low: usize,
    high: usize,
    tx: &mpsc::Sender<Operation>,
) -> usize {
    if high >= bars.len() || low >= bars.len() {
        return low; // Return safe index
    }

    let pivot = bars[high].value;
    let mut i: usize = low;

    for j in low..high {
        let _ = tx.send(Operation::Compare(j, high));
        thread::sleep(std::time::Duration::from_millis(10));

        if bars[j].value < pivot {
            let _ = tx.send(Operation::Swap(i, j));
            bars.swap(i, j);
            thread::sleep(std::time::Duration::from_millis(10));
            i += 1;
        }

        let _ = tx.send(Operation::SetColor(j, Color32::WHITE));
        let _ = tx.send(Operation::SetColor(high, Color32::WHITE));
    }

    let _ = tx.send(Operation::Swap(i, high));
    bars.swap(i, high);
    thread::sleep(std::time::Duration::from_millis(10));

    i
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
    let mut rng = thread_rng();
    let mut values: Vec<usize> = bars.iter().map(|b| b.value).collect();
    while !is_sorted(&values) {
        values.shuffle(&mut rng);
    }
    copy_values_to_bars(bars, &values, tx);
}

// ---------- Bozo Sort ----------
pub fn bozo_sort(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<Operation>) {
    let mut rng = thread_rng();
    let mut values: Vec<usize> = bars.iter().map(|b| b.value).collect();
    while !is_sorted(&values) {
        let i = rng.gen_range(0..values.len());
        let j = rng.gen_range(0..values.len());
        values.swap(i, j);
    }
    copy_values_to_bars(bars, &values, tx);
}

// ---------- Stooge Sort ----------
fn stooge_sort_recursive(slice: &mut [usize]) {
    let n = slice.len();
    if n <= 1 {
        return;
    }
    if slice[n - 1] < slice[0] {
        slice.swap(0, n - 1);
    }
    if n > 2 {
        let t = n / 3;
        stooge_sort_recursive(&mut slice[..n - t]);
        stooge_sort_recursive(&mut slice[t..]);
        stooge_sort_recursive(&mut slice[..n - t]);
    }
}

pub fn stooge_sort(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<Operation>) {
    let mut values: Vec<usize> = bars.iter().map(|b| b.value).collect();
    stooge_sort_recursive(&mut values);
    copy_values_to_bars(bars, &values, tx);
}

// ---------- Slow Sort ----------
fn slow_sort_recursive(slice: &mut [usize]) {
    let n = slice.len();
    if n <= 1 {
        return;
    }
    let m = n / 2;
    slow_sort_recursive(&mut slice[..m]);
    slow_sort_recursive(&mut slice[m..]);
    if slice[m - 1] > slice[n - 1] {
        slice.swap(m - 1, n - 1);
    }
    slow_sort_recursive(&mut slice[..n - 1]);
}

pub fn slow_sort(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<Operation>) {
    let mut values: Vec<usize> = bars.iter().map(|b| b.value).collect();
    slow_sort_recursive(&mut values);
    copy_values_to_bars(bars, &values, tx);
}
