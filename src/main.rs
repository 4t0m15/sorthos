// ---------- TimSort (visual, fully instrumented) ----------
pub fn tim_sort(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<Operation>) {
    use std::thread::sleep;
    use std::time::Duration;

    const RUN: usize = 32;
    const DELAY: Duration = Duration::from_millis(10);

    let n = bars.len();

    /* -------- Phase A – build 32‑element runs via insertion sort -------- */
    for start in (0..n).step_by(RUN) {
        let end = usize::min(start + RUN, n);
        for i in (start + 1)..end {
            let mut j = i;
            while j > start {
                // 1 · Flash comparison pair
                tx.send(Operation::Compare(j - 1, j)).ok();
                sleep(DELAY);

                if bars[j - 1].value > bars[j].value {
                    // 2 · Swap and flash
                    tx.send(Operation::Swap(j - 1, j)).ok();
                    bars.swap(j - 1, j);
                }
                // 3 · Clear colours
                tx.send(Operation::SetColor(j - 1, Color32::WHITE)).ok();
                tx.send(Operation::SetColor(j,     Color32::WHITE)).ok();
                sleep(DELAY);

                if bars[j - 1].value <= bars[j].value {
                    break;
                }
                j -= 1;
            }
        }
    }

    /* -------- Phase B – bottom‑up merges of the runs -------- */
    let mut size = RUN;
    while size < n {
        for left in (0..n).step_by(2 * size) {
            let mid   = usize::min(left + size, n);
            let right = usize::min(left + 2 * size, n);
            merge_visual(bars, left, mid, right, tx);
        }
        size *= 2;
    }

    /* -------- Phase C – final cleanup -------- */
    for idx in 0..n {
        tx.send(Operation::SetColor(idx, Color32::WHITE)).ok();
    }
}

/// Merge two consecutive sorted runs `[left, mid)` and `[mid, right)`
/// while emitting Compare/Swap/SetColor events.
fn merge_visual(
    bars: &mut Vec<SortBar>,
    left: usize,
    mid: usize,
    right: usize,
    tx: &mpsc::Sender<Operation>,
) {
    use std::thread::sleep;
    use std::time::Duration;

    const DELAY: Duration = Duration::from_millis(10);

    // Snapshot the region’s values
    let temp: Vec<usize> = bars[left..right].iter().map(|b| b.value).collect();

    let (mut i, mut j, mut k) = (left, mid, 0);

    while i < mid && j < right {
        tx.send(Operation::Compare(i, j)).ok();
        sleep(DELAY);

        if temp[i - left] <= temp[j - left] {
            bars[left + k].value = temp[i - left];
            tx.send(Operation::SetColor(left + k, Color32::WHITE)).ok();
            i += 1;
        } else {
            tx.send(Operation::Swap(i, j)).ok();
            bars[left + k].value = temp[j - left];
            tx.send(Operation::SetColor(left + k, Color32::WHITE)).ok();
            j += 1;
        }
        sleep(DELAY);
        k += 1;
    }

    while i < mid {
        bars[left + k].value = temp[i - left];
        tx.send(Operation::SetColor(left + k, Color32::WHITE)).ok();
        i += 1;
        k += 1;
    }
    while j < right {
        bars[left + k].value = temp[j - left];
        tx.send(Operation::SetColor(left + k, Color32::WHITE)).ok();
        j += 1;
        k += 1;
    }
}