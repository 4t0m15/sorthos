// Pancake Sort
pub fn pancake_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let mut j = arr.len();
    let mut ans = vec![];
    while j > 0 {
        let f = arr.iter().position(|&e| e == (j as i32)).unwrap();
        if (f == j - 1) {
            j -= 1;
            continue;
        }
        ans.push((f + 1) as i32);
        ans.push(j as i32);
        arr[0..(f + 1)].reverse();
        arr[0..j].reverse();
        j -= 1;    
    }
    ans 
}