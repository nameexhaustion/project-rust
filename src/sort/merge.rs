fn merge<T: PartialOrd + Copy>(l: &mut [T], r: &mut [T], a: &mut [T]) {
    let mut index_l = 0;
    let mut index_r = 0;

    for i in 0..a.len() {
        let value_l = l.get(index_l).unwrap();
        let value_r = r.get(index_r).unwrap();
        match value_l <= value_r {
            true => {
                a[i] = *value_l;
                index_l += 1;
            }
            false => {
                a[i] = *value_r;
                index_r += 1;
            }
        }

        if index_l == l.len() {
            a[i + 1..].clone_from_slice(&r[index_r..]);
            break;
        }

        if index_r == r.len() {
            a[i + 1..].clone_from_slice(&l[index_l..]);
            break;
        }
    }
}

pub fn mergesort<T: std::cmp::PartialOrd + std::marker::Copy>(a: &mut [T]) -> &mut [T] {
    if a.len() <= 1 {
        return a;
    }

    let mut left: Vec<T> = Vec::new();
    let mut right: Vec<T> = Vec::new();
    left.extend_from_slice(&a[0..(a.len() / 2)]);
    right.extend_from_slice(&a[(a.len() / 2)..a.len()]);
    merge(mergesort(&mut left), mergesort(&mut right), a);
    a
}
