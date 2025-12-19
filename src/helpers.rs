pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let n_cols = v[0].len();
    let mut lines: Vec<_> = v.into_iter().map(|line| line.into_iter()).collect();

    (0..n_cols)
        .map(|_| {
            lines
                .iter_mut()
                .map(|line| line.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[cfg(test)]
pub fn rot90<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = v.len();
    let cols = v[0].len();
    assert_eq!(rows, cols);

    (0..rows)
        .map(|i| (0..cols).map(|j| v[j][rows-i-1].clone()).collect())
        .collect()
}

#[test]
fn test_rot90() {
    let v = vec![vec![1, 2], vec![3, 4]];
    let rotated = vec![vec![2, 4], vec![1, 3]];
    assert_eq!(rot90(v), rotated);
}
