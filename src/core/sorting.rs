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
                // TimSort implementation would go here
            }
            SortingAlgorithm::BlockMergeSort => {
                // Block Merge Sort implementation would go here
            }
            SortingAlgorithm::BogoSort => {
                // BogoSort implementation would go here
            }
            SortingAlgorithm::BozoSort => {
                // BozoSort implementation would go here
            }
            SortingAlgorithm::StoogeSort => {
                // StoogeSort implementation would go here
            }
            SortingAlgorithm::SlowSort => {
                // SlowSort implementation would go here
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