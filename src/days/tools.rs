// use std::{
//     cell::RefCell,
//     rc::{Rc, Weak},
// };

pub fn matrix_transpose<T: Copy>(m: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut t = vec![Vec::with_capacity(m.len()); m[0].len()];
    for r in m {
        for i in 0..r.len() {
            t[i].push(r[i]);
        }
    }
    t
}

// #[derive(Debug)]
// struct Node<T> {
//     value: T,
//     parent: RefCell<Weak<Self>>,
//     children: RefCell<Vec<Rc<Self>>>,
// }
