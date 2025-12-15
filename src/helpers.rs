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