fn merge<T: PartialOrd + Copy>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    if a.len() == 0 {
        return b;
    }

    if b.len() == 0 {
        return a;
    }

    let mut r: Vec<T> = Vec::new();
    let mut index_a = 0;
    let mut index_b = 0;

    for _ in 0..a.len() + b.len() {
        let value_a = *a.get(index_a).unwrap();
        let value_b = *b.get(index_b).unwrap();
        match value_a <= value_b {
            true => {
                r.push(value_a);
                index_a += 1;
            }
            false => {
                r.push(value_b);
                index_b += 1;
            }
        }

        if index_a == a.len() {
            r.extend_from_slice(&b[index_b..]);
            break;
        }

        if index_b == b.len() {
            r.extend_from_slice(&a[index_a..]);
            break;
        }
    }

    r
}

fn mergesort_main<T: std::cmp::PartialOrd + std::marker::Copy>(a: Vec<T>) -> Vec<T> {
    if a.len() <= 1 {
        return a;
    }
    let mut left: Vec<T> = Vec::new();
    let mut right: Vec<T> = Vec::new();
    left.extend_from_slice(&a[0..(a.len() / 2)]);
    right.extend_from_slice(&a[(a.len() / 2)..a.len()]);
    merge(mergesort_main(left), mergesort_main(right))
}

pub fn mergesort<T: std::cmp::PartialOrd + std::marker::Copy>(a: &mut [T]) {
    let mut b: Vec<T> = Vec::new();
    b.extend_from_slice(a);

    let b = mergesort_main(b);

    a.clone_from_slice(&b);
}
