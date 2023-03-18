// Sorting half an array is easyest than sorting all array
//

pub fn merge_sort<T: PartialOrd + std::fmt::Debug>(mut v: Vec<T>) -> Vec<T> {
    if v.len() <= 1 {
        return v;
    }

    println!("{v:?}");

    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);

    println!(">a {a:?}");
    println!(">b {b:?}");

    // bring them together add lowest the front of a or the front of b

    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();

    let mut res: Vec<T> = Vec::new();

    loop {
        match a_peek {
            Some(ref av) => match b_peek {
                Some(ref bv) => {
                    if bv < av {
                        res.push(b_peek.take().unwrap());
                        b_peek = b_it.next();
                    } else {
                        res.push(a_peek.take().unwrap());
                        a_peek = a_it.next();
                    }
                }
                None => {
                    res.push(a_peek.take().unwrap());
                    res.extend(a_it);
                    return res;
                }
            },
            None => {
                if let Some(bv) = b_peek {
                    res.push(bv);
                }
                res.extend(b_it);
                return res;
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn bubble_sort_test() {
        let v = vec![1, 5, 4, 9, 6, 2, 12, 40, 23, 13, 42];
        let v = merge_sort(v);
        assert_eq!(v, vec![1, 2, 4, 5, 6, 9, 12, 13, 23, 40, 42]);
    }
}
