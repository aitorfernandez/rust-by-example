//
// O(n^2) "kind" of complexity

pub fn bubble_sort<T: PartialOrd + std::fmt::Debug>(v: &mut [T]) {
    for i in 0..v.len() {
        let mut sorted = true;
        for j in 0..(v.len() - 1) - i {
            // - i (index first loop) because the last one is already sorted
            if v[j] > v[j + 1] {
                v.swap(j + 1, j);
                sorted = false;
            }
        }
        println!("{v:?}");
        if sorted {
            return;
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn bubble_sort_test() {
        let mut v = vec![1, 5, 4, 9, 6, 2];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 2, 4, 5, 6, 9]);
    }
}
