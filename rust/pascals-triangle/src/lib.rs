pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let row_count = row_count as usize;
        let mut rows = Vec::<Vec<u32>>::new();
        rows.reserve_exact(row_count);
        for i in 0..row_count {
            let mut row = Vec::new();
            row.reserve_exact(i + 1);
            for j in 0..i + 1 {
                if j == 0 || j == i {
                    row.push(1);
                } else {
                    let prev = &rows[i as usize - 1];
                    row.push(prev[j] + prev[j - 1])
                }
            }
            rows.push(row);
        }
        PascalsTriangle { rows: rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
