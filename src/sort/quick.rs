fn partition<T: PartialOrd + Copy + std::fmt::Debug>(a: &mut [T]) -> usize {
    let p = a[0];
    let mut l = 0;
    let mut r = a.len() - 1;
    loop {
        loop {
            if a[l] > p {
                break;
            }

            if l == a.len() - 1 {
                break;
            }

            l += 1;
        }

        loop {
            if a[r] < p {
                break;
            }

            if r == 0 {
                break;
            }

            r -= 1;
        }

        if l >= r {
            break;
        }

        println!("swapping {} {} {:?}", l, r, a);

        let n = a[l];
        a[l] = a[r];
        a[r] = n;
    }

    a[0] = a[r];
    a[r] = p;

    r
}

pub fn quicksort<T: PartialOrd + Copy + std::fmt::Debug>(a: &mut [T]) {
    if a.len() > 1 {
        let i = partition(a);
        quicksort(&mut a[..i]);
        quicksort(&mut a[i + 1..]);
    }
}
