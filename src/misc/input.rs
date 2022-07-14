use std::str::FromStr;
use std::str::SplitWhitespace;
pub struct In<'a> {
    iter: SplitWhitespace<'a>,
}
impl<'a> In<'a> {
    pub fn new(s: &'a String) -> Self {
        Self {
            iter: s.split_whitespace(),
        }
    }
    pub fn read<T: FromStr>(&mut self) -> T {
        self.iter.next().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn checked_read<T: FromStr>(&mut self) -> Option<T> {
        match self.iter.next() {
            Some(x) => Some(x.parse::<T>().ok().unwrap()),
            None => None
        }
    }
    pub fn i32(&mut self) -> i32 {
        self.read::<i32>()
    }
    pub fn i64(&mut self) -> i64 {
        self.read::<i64>()
    }
    pub fn usize(&mut self) -> usize {
        self.read::<usize>()
    }

    pub fn bytes(&mut self) -> Vec<u8> {
        self.iter.next().unwrap().bytes().collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.iter.next().unwrap().chars().collect()
    }
    pub fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
}
