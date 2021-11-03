/*use std::mem::transmute;

// |f| (|x| f(|v| x(x)(v)))

pub unsafe fn test<T>(f: T) {
    |f: *const ()| {
        let f = transmute::<*const (), fn(*const ())>(f);
        move |x: *const ()| {
            let x = transmute::<*const (), fn(*const ()) -> *const ()>(x);
            let v = |v| {
                let y = transmute::<fn(*const ()) -> *const (), *const ()>(x);
                let a = x(y);
                let a = transmute::<*const (), fn(*const ())>(a);
                a(v)
            };

            f(transmute(v));
        }
    };
}
*/
