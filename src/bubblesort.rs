pub fn bubbles(mut a: [i32; 12]) -> [i32; 12] {
    for i in 0..a.len() {
        for j in 0..a.len() - 1 - i {
            if a[j] > a[j + 1] {
                let tmp = a[j];
                a[j] = a[j + 1];
                a[j + 1] = tmp;
            }
        }
    }
    a
}

pub fn bubbles_vec(v: &mut Vec<i32>) -> &mut Vec<i32> {
    for i in 0..v.len() {
        for j in 0..v.len() - 1 - i {
            if v[j] > v[j + 1] {
                let tmp = v[j];
                v[j] = v[j + 1];
                v[j + 1] = tmp;
            }
        }
    }
    v
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubbles() {
        let t: [i32; 12] = [8, 0, 10, 1, 3, 11, 4, 5, 2, 6, 7, 9];
        let result = bubbles(t);
        dbg!(result);
    }

    #[test]
    fn test_bubbles_vec() {
        let mut v = vec![9, 2, 0, 1, 4, 10, 3, 5, 8, 6, 7, 11];
        let result = bubbles_vec(&mut v);
        dbg!(result);
    }
}
