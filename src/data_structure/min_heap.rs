use std::cmp::Reverse;
use std::collections::BinaryHeap;
#[derive(Debug)]
pub struct MinHeap<T>(BinaryHeap<Reverse<T>>);

impl<T> MinHeap<T>
where
    Reverse<T>: Ord,
{
    pub fn new() -> MinHeap<T> {
        Self(BinaryHeap::new())
    }

    pub fn with_capacity(capacity: usize) -> MinHeap<T> {
        Self(BinaryHeap::with_capacity(capacity))
    }

    pub fn push(&mut self, item: T) {
        self.0.push(Reverse(item));
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.0.pop() {
            Some(x) => Some(x.0),
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.0.peek() {
            Some(x) => Some(&x.0),
            None => None,
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn append(&mut self, other: &mut MinHeap<T>) {
        self.0.append(&mut other.0)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn into_sorted_vec(self) -> Vec<T> {
        self.0.into_sorted_vec().into_iter().map(|x| x.0).collect()
    }
}
