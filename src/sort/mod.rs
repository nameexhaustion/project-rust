mod insertion;
use insertion::insertionsort;
mod merge;
use merge::mergesort;
mod quick;
use quick::quicksort;

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

macro_rules! test_sort_stable {
    ($a:ident) => {{
        #[derive(Debug)]
        struct V {
            n: u32,
            d: u32,
        }

        impl V {
            fn new(n: u32) -> V {
                let mut d = n;
                while d > 10 {
                    d /= 10;
                }
                V { n, d }
            }
        }

        impl PartialEq for V {
            fn eq(&self, other: &Self) -> bool {
                self.n == other.n
            }
        }

        impl PartialOrd for V {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.d.cmp(&other.d))
            }

            fn lt(&self, other: &Self) -> bool {
                self.d < other.d
            }
            fn le(&self, other: &Self) -> bool {
                self.d <= other.d
            }
            fn gt(&self, other: &Self) -> bool {
                self.d > other.d
            }
            fn ge(&self, other: &Self) -> bool {
                self.d >= other.d
            }
        }

        impl Clone for V {
            fn clone(&self) -> Self {
                V {
                    n: self.n,
                    d: self.d,
                }
            }
        }

        impl Copy for V {}

        let mut d = vec![V::new(11), V::new(12)];
        $a(&mut d);
        assert_eq!(d, vec![V::new(11), V::new(12)]);
    }};
}

pub fn test_function() {
    test_sort!(insertionsort);
    test_sort_stable!(insertionsort);
    test_sort!(mergesort);
    test_sort_stable!(mergesort);
    test_sort!(quicksort);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insertion() {
        test_sort!(insertionsort);
        test_sort_stable!(insertionsort);
    }

    #[test]
    fn test_merge() {
        test_sort!(mergesort);
        test_sort_stable!(mergesort);
    }

    #[test]
    fn test_quick() {
        test_sort!(quicksort);
    }
}
