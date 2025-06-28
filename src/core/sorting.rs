#[path = "../Sorting/bubble_sort.rs"]
mod bubble_sort;
#[path = "../Sorting/selection_sort.rs"]
mod selection_sort;
#[path = "../Sorting/insertion_sort.rs"]
mod insertion_sort;
#[path = "../Sorting/burstsort.rs"]
mod burstsort;
#[path = "../Sorting/Introsort.rs"]
mod introsort;
#[path = "../Sorting/quadsort.rs"]
mod quadsort;
#[path = "../Sorting/quicksort.rs"]
mod quicksort_numeric;
#[path = "../Sorting/spaghettisort.rs"]
mod spaghettisort;

use crate::models::SortBar;
use eframe::egui::Color32;
use std::sync::mpsc;
use std::thread;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::time::Duration;

pub use bubble_sort::bubble_sort;
pub use selection_sort::selection_sort;
pub use insertion_sort::insertion_sort;
pub use burstsort::burst_sort;
pub use introsort::introsort;
pub use quadsort::quad_sort;
pub use quicksort_numeric::quick_sort as quick_sort_numeric;
pub use spaghettisort::spaghetti_sort;
pub use spaghettisort::spaghetti_sort_optimized;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortingAlgorithm {
    Bubble,
    Selection,
    Insertion,
    Quick,
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

pub fn start_sort(algorithm: SortingAlgorithm, mut bars: Vec<SortBar>, tx: mpsc::Sender<Operation>) {
    thread::spawn(move || {
        let len = bars.len();
        match algorithm {
            SortingAlgorithm::Bubble => bubble_sort(&mut bars, &tx),
            SortingAlgorithm::Selection => selection_sort(&mut bars, &tx),
            SortingAlgorithm::Insertion => insertion_sort(&mut bars, &tx),
            SortingAlgorithm::Quick => quick_sort(&mut bars, 0, len.saturating_sub(1), &tx),
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
    if low < high {
        let pi = partition(bars, low, high, tx);
        if pi > 0 {
            quick_sort(bars, low, pi - 1, tx);
        }
        quick_sort(bars, pi + 1, high, tx);
    }
}

fn partition(bars: &mut Vec<SortBar>, low: usize, high: usize, tx: &mpsc::Sender<Operation>) -> usize {
    let pivot = bars[high].value;
    let mut i = low;

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

// ---------- TimSort (visualised via insertion‑sort steps for demo purposes) ----------
pub fn tim_sort(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<Operation>) {
    let n = bars.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 {
            // Highlight comparison
            let _ = tx.send(Operation::Compare(j - 1, j));
            thread::sleep(Duration::from_millis(6));

            if bars[j - 1].value > bars[j].value {
                // Swap and animate
                let _ = tx.send(Operation::Swap(j - 1, j));
                bars.swap(j - 1, j);
                thread::sleep(Duration::from_millis(6));
            } else {
                // Reset colours and break early
                let _ = tx.send(Operation::SetColor(j - 1, Color32::WHITE));
                let _ = tx.send(Operation::SetColor(j, Color32::WHITE));
                break;
            }
            j -= 1;
        }
    }
    // Paint all bars white when done
    for idx in 0..n {
        let _ = tx.send(Operation::SetColor(idx, Color32::WHITE));
    }
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