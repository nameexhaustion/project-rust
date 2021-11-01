fn insertion_sort<T: std::cmp::PartialOrd + std::marker::Copy>(a: &mut [T]) {
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn insertion_sort_test() {
        let mut d = vec![5, 4, 3, 2, 1];
        insertion_sort(&mut d);
        assert_eq!(d, [1, 2, 3, 4, 5]);
        let mut d = vec!["E", "D", "C", "B", "A"];
        insertion_sort(&mut d);
        assert_eq!(d, ["A", "B", "C", "D", "E"]);
    }
}
