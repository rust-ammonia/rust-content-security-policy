/*!
A cache-friendly binary search implementation.
This should probably be moved to its own crate, eventually.

As described on <http://cglab.ca/%7Emorin/misc/arraylayout/>,
this is a eytzinger (breadth-first) layout.

Also described, and benched, at
<http://bannalia.blogspot.com/2015/06/cache-friendly-binary-search.html>.
*/

use std::cmp::min;
use std::cmp::Ordering::{self, Less, Equal, Greater};
use std::isize;
use std::mem;
use std::ptr;

/// Given a sorted vector, rearrange it for cache-friendly binary search.
/// This allocates an intermediate buffer the same length as the given vector.
pub fn arrange<T>(v: &mut Vec<T>) {
    unsafe {
        assert!(v.len() < (isize::MAX >> 1) as usize);
        let mut buf = Vec::with_capacity(v.len());
        insert(0, v.len() as isize, v.get_unchecked(0), buf.get_unchecked_mut(0));
        buf.set_len(v.len());
        mem::forget(mem::replace(v, buf))
    }
}

unsafe fn insert<T>(i: isize, n: isize, src: *const T, dest: *mut T) {
    if n == 0 { return };
    let h = root(n);
    ptr::copy_nonoverlapping(src.offset(h), dest.offset(i), 1);
    insert(2 * i + 1, h, src, dest);
    insert(2 * i + 2, n - h - 1, src.offset(h + 1), dest);
}

fn root(n: isize) -> isize {
    if n <= 1 { return 0 };
    let mut i = 2;
    while i <= n { i *= 2 };
    min(i / 2 - 1, n - i / 4)
}

pub fn find<T, F: FnMut(&T) -> Ordering>(v: &[T], mut f: F) -> Option<&T> {
    let mut i = 0;
    loop {
        if let Some(x) = v.get(i) {
            match f(x) {
                Greater => i = 2 * i + 1,
                Less => i = 2 * i + 2,
                Equal => return Some(x),
            }
        } else {
            return None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    macro_rules! do_search_test {
        ($i:ident; $( $a:expr ),*; $( $b:expr ),*) => {
            #[test]
            fn $i() {
                let mut v: Vec<usize> = vec![$($a,)*];
                arrange(&mut v);
                assert_eq!(v, vec![$($b,)*]);
                assert_eq!(find(&v[..], |_| Less), None);
                for i in 0 .. v.len() {
                    assert_eq!(find(&v[..], |x| x.cmp(&(i + 1))), Some(&(i + 1)));
                }
                assert_eq!(find(&v[..], |_| Greater), None);
            }
        }
    }
    do_search_test!{zero; ; }
    do_search_test!{one; 1; 1}
    do_search_test!{two; 1, 2; 2, 1}
    do_search_test!{three; 1, 2, 3; 2, 1, 3}
    do_search_test!{four; 1, 2, 3, 4; 3, 2, 4, 1}
    do_search_test!{five; 1, 2, 3, 4, 5; 4, 2, 5, 1, 3}
    do_search_test!{six; 1, 2, 3, 4, 5, 6; 4, 2, 6, 1, 3, 5}
}