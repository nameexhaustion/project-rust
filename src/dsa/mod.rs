mod dlist;
mod slist;

pub fn test_slist() {
    {
        let mut l = slist::List::<u128>::new();

        for _ in 1..200 {
            for _ in 1..10000 {
                l.push(999999);
            }
        }
    }
}
