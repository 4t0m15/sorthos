#[path = "../Sorting/bubble_sort.rs"]
mod bubble_sort;
#[path = "../Sorting/selection_sort.rs"]
mod selection_sort;
#[path = "../Sorting/insertion_sort.rs"]
mod insertion_sort;

use crate::models::SortBar;
use eframe::egui::Color32;
use std::sync::mpsc;
use std::thread;

pub use bubble_sort::bubble_sort;
pub use selection_sort::selection_sort;
pub use insertion_sort::insertion_sort;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortingAlgorithm {
    Bubble,
    Selection,
    Insertion,
    Quick,
}

impl SortingAlgorithm {
    pub fn all() -> &'static [SortingAlgorithm] {
        &[
            SortingAlgorithm::Bubble,
            SortingAlgorithm::Selection,
            SortingAlgorithm::Insertion,
            SortingAlgorithm::Quick,
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