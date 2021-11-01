pub fn insertion_sort<T: std::cmp::PartialOrd + std::marker::Copy>(a: &mut [T]) {
    for i in 1..a.len() {
        let k = a[i];
        let mut j = i;
        while j > 0 && a[j - 1] > k {
            a[j] = a[j - 1];
            j -= 1;
        }
        a[j] = k;
    }
}
