pub fn z_function<T: std::cmp::PartialEq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    let (mut l, mut r) = (0, 0);
    for i in 1..n {
        if i <= r {
            z[i] = (r - i + 1).min(z[i - l]);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] - 1 > r {
            l = i;
            r = i + z[i] - 1;
        }
    }
    z
}
