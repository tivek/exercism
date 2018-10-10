pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let cols_n = input[0].len();
    let min_in_cols: Vec<u64> = (0..cols_n)
        .map(|ic| input.iter().map(|r| r[ic]).min().unwrap_or(std::u64::MAX))
        .collect();

    let max_in_rows_it = input.iter().map(|r| r.iter().max().unwrap_or(&0));

    let mut out = Vec::new();
    for (ir, max_in_row) in max_in_rows_it.enumerate() {
        for (ic, min_in_col) in min_in_cols.iter().enumerate() {
            if *max_in_row == *min_in_col {
                out.push((ir, ic));
            }
        }
    }
    out
}
