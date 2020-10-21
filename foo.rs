#![allow(unused)]
macro_rules! cons {
    ($head:tt) => (
        ($head, ())
    );
    ($head:tt, $($tail:tt),*) => (
        ($head, cons!($($tail),*))
    );
}

pub fn blah() {
    let cons!(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, z): (u32, (u32, (u32, (u32, (u32, (u32, (u32, (u32, (u32, (u32, (u32, (u32, (u32, (u32, (u32, (u32, (u32, (u32, (u32, (u32, (u32, (u32, (u32, (u32, (u32, ()))))))))))))))))))))))))) = todo!();
}
