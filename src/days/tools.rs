// use std::{
//     cell::RefCell,
//     rc::{Rc, Weak},
// };
// use std::collections::{HashSet, VecDeque};
// use std::hash::Hash;

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

// fn bfs<T, N, E>(start: T, neighbourhood: N, end: E) -> Option<T>
// where
//     T: Clone + Eq + Hash + Copy,
//     N: Fn(&T) -> Vec<T>,
//     E: Fn(T) -> bool,
// {
//     let mut queue = VecDeque::new();
//     let mut visited = HashSet::new();
//     queue.push_front(start);
//     while let Some(state) = queue.pop_front() {
//         if end(state) {
//             return Some(state);
//         }
//         for state in neighbourhood(&state) {
//             if visited.contains(&state) {
//                 continue;
//             }
//             queue.push_back(state.clone());
//             visited.insert(state);
//         }
//     }
//     None
// }
