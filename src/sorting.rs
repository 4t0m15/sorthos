use crate::models::SortBar;
use eframe::egui::Color32;
use std::sync::mpsc;
use std::thread;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortingAlgorithm {
    Bubble,
    Selection,
    Insertion,
    Quick,
    Merge,
}

impl SortingAlgorithm {
    pub fn all() -> &'static [SortingAlgorithm] {
        &[
            SortingAlgorithm::Bubble,
            SortingAlgorithm::Selection,
            SortingAlgorithm::Insertion,
            SortingAlgorithm::Quick,
            SortingAlgorithm::Merge,
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
        match algorithm {
            SortingAlgorithm::Bubble => bubble_sort(&mut bars, &tx),
            SortingAlgorithm::Selection => selection_sort(&mut bars, &tx),
            SortingAlgorithm::Insertion => insertion_sort(&mut bars, &tx),
            SortingAlgorithm::Quick => quick_sort(&mut bars, 0, bars.len().saturating_sub(1), &tx),
            SortingAlgorithm::Merge => merge_sort(&mut bars, 0, bars.len().saturating_sub(1), &tx),
        }
        let _ = tx.send(Operation::Done);
    });
}

fn bubble_sort(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<Operation>) {
    let n = bars.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            let _ = tx.send(Operation::Compare(j, j + 1));
            thread::sleep(std::time::Duration::from_millis(10));
            
            if bars[j].value > bars[j + 1].value {
                let _ = tx.send(Operation::Swap(j, j + 1));
                bars.swap(j, j + 1);
                thread::sleep(std::time::Duration::from_millis(10));
            }
            
            let _ = tx.send(Operation::SetColor(j, Color32::WHITE));
            let _ = tx.send(Operation::SetColor(j + 1, Color32::WHITE));
        }
    }
}

fn selection_sort(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<Operation>) {
    let n = bars.len();
    for i in 0..n {
        let mut min_idx = i;
        for j in i + 1..n {
            let _ = tx.send(Operation::Compare(min_idx, j));
            thread::sleep(std::time::Duration::from_millis(10));
            
            if bars[j].value < bars[min_idx].value {
                min_idx = j;
            }
            
            let _ = tx.send(Operation::SetColor(min_idx, Color32::WHITE));
            let _ = tx.send(Operation::SetColor(j, Color32::WHITE));
        }
        
        if min_idx != i {
            let _ = tx.send(Operation::Swap(i, min_idx));
            bars.swap(i, min_idx);
            thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}

fn insertion_sort(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<Operation>) {
    let n = bars.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 {
            let _ = tx.send(Operation::Compare(j - 1, j));
            thread::sleep(std::time::Duration::from_millis(10));
            
            if bars[j - 1].value > bars[j].value {
                let _ = tx.send(Operation::Swap(j - 1, j));
                bars.swap(j - 1, j);
                thread::sleep(std::time::Duration::from_millis(10));
                j -= 1;
            } else {
                break;
            }
            
            let _ = tx.send(Operation::SetColor(j, Color32::WHITE));
            let _ = tx.send(Operation::SetColor(j + 1, Color32::WHITE));
        }
    }
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

fn merge_sort(bars: &mut Vec<SortBar>, left: usize, right: usize, tx: &mpsc::Sender<Operation>) {
    if left < right {
        let mid = left + (right - left) / 2;
        merge_sort(bars, left, mid, tx);
        merge_sort(bars, mid + 1, right, tx);
        merge(bars, left, mid, right, tx);
    }
}

fn merge(bars: &mut Vec<SortBar>, left: usize, mid: usize, right: usize, tx: &mpsc::Sender<Operation>) {
    let left_part: Vec<SortBar> = bars[left..=mid].to_vec();
    let right_part: Vec<SortBar> = bars[mid + 1..=right].to_vec();
    
    let mut i = 0;
    let mut j = 0;
    let mut k = left;
    
    while i < left_part.len() && j < right_part.len() {
        let _ = tx.send(Operation::Compare(k, k));
        thread::sleep(std::time::Duration::from_millis(10));
        
        if left_part[i].value <= right_part[j].value {
            bars[k] = left_part[i].clone();
            i += 1;
        } else {
            bars[k] = right_part[j].clone();
            j += 1;
        }
        
        let _ = tx.send(Operation::SetColor(k, Color32::GREEN));
        thread::sleep(std::time::Duration::from_millis(10));
        let _ = tx.send(Operation::SetColor(k, Color32::WHITE));
        
        k += 1;
    }
    
    while i < left_part.len() {
        bars[k] = left_part[i].clone();
        let _ = tx.send(Operation::SetColor(k, Color32::GREEN));
        thread::sleep(std::time::Duration::from_millis(10));
        let _ = tx.send(Operation::SetColor(k, Color32::WHITE));
        i += 1;
        k += 1;
    }
    
    while j < right_part.len() {
        bars[k] = right_part[j].clone();
        let _ = tx.send(Operation::SetColor(k, Color32::GREEN));
        thread::sleep(std::time::Duration::from_millis(10));
        let _ = tx.send(Operation::SetColor(k, Color32::WHITE));
        j += 1;
        k += 1;
    }
}
