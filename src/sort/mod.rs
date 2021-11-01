mod insertion;
use insertion::insertion_sort;
mod merge;
use merge::merge_sort;

macro_rules! test_sort {
    ($a:ident) => {{
        let mut d = vec![5, 4, 3, 2, 1];
        $a(&mut d);
        assert_eq!(d, [1, 2, 3, 4, 5]);
        let mut d = vec![0, 0];
        $a(&mut d);
        assert_eq!(d, [0, 0]);
        let mut d = vec![0];
        $a(&mut d);
        assert_eq!(d, [0]);
        let mut d: Vec<u32> = Vec::new();
        $a(&mut d);
        assert_eq!(d, []);
    }};
}

pub fn test_function() {
    test_sort!(insertion_sort);
    test_sort!(merge_sort);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insertion() {
        test_sort!(insertion_sort);
    }

    #[test]
    fn test_merge() {
        test_sort!(merge_sort);
    }
}
