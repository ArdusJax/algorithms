// search an array of ten elements of i32
pub fn search_array_of_ten_elements(collection: [i32; 10], element: i32) -> bool {
    let (mut start, mut end) = (0, collection.len());

    while start < end {
        let mid = start + (end - start) / 2;
        if collection[mid] == element {
            return true;
        } else if collection[mid] > element {
            end = mid;
        } else {
            start = mid + 1;
        }
    }
    false
}

pub fn search_vector(v: &Vec<i32>, element: i32) -> bool {
    let (mut start, mut end) = (0, v.len());

    while start < end {
        let mid = start + (end - start) / 2;

        if v[mid] == element {
            return true;
        } else if v[mid] > element {
            end = mid;
        } else {
            start = mid + 1;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::binarysearch::{search_array_of_ten_elements, search_vector};

    #[test]
    fn test_ten_element_array() {
        let a: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert!(search_array_of_ten_elements(a, 3));
        assert!(search_array_of_ten_elements(a, 9));
        assert!(search_array_of_ten_elements(a, 0));
        assert!(!search_array_of_ten_elements(a, 10));
    }

    #[test]
    fn test_vector_search() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert!(search_vector(&v, 3));
        assert!(search_vector(&v, 9));
        assert!(search_vector(&v, 0));
        assert!(!search_vector(&v, 10));
    }
}
