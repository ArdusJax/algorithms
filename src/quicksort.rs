// Running time is n^2 - nlog(n)
//
// Broken down into two functions:
// - partition: helper function that sorts one side or partition of the Vector
// - quicksort: Sorts a Vec using the quicksort algorithm
//
pub fn qs(arr: &mut Vec<i32>, lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    let pivot_idx = partition(arr, lo, hi);
    qs(arr, lo, (pivot_idx - 1) as usize);
    qs(arr, (pivot_idx + 1) as usize, hi);
    dbg!(arr);
}

fn partition(arr: &mut Vec<i32>, lo: usize, hi: usize) -> i32 {
    let (pivot, mut idx, mut i) = (arr[hi], lo - 1, lo);

    while i < hi {
        if arr[i] <= pivot {
            idx += 1;
            let tmp = arr[i];
            arr[i] = arr[idx];
            arr[idx] = tmp;
        }
        i += 1;
    }

    idx += 1;
    arr[hi] = arr[idx];
    arr[idx] = pivot;

    idx as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sort_my_array() {
        let mut data = vec![0, 34, 1, 12, 3, 55, 32, 5, 1, 21, 17];
        let length = data.len() - 1;
        qs(&mut data, 1, length);
    }
}
