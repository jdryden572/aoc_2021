#[derive(Debug)]
pub struct Board {
    pub index: usize,
    rows: [[Number; 5]; 5],
}

impl Board {
    pub fn new<I: Iterator<Item = i32>>(index: usize, mut numbers: I) -> Self {
        let mut rows: [[Number; 5]; 5] = [[Number::default(); 5]; 5];
        for row in 0..5 {
            for col in 0..5 {
                rows[row][col] = numbers.next().unwrap().into();
            }
        }
        Self { index, rows }
    }

    pub fn mark(&mut self, number: i32) {
        for row in 0..5 {
            for col in 0..5 {
                if self.rows[row][col] == Number::Unmarked(number) {
                    self.rows[row][col] = Number::Marked(number);
                    return;
                }
            }
        }
    }

    fn columns(&self) -> [[Number; 5]; 5] {
        let mut columns = [[Number::default(); 5]; 5];
        for row in 0..5 {
            for col in 0..5 {
                columns[col][row] = self.rows[row][col];
            }
        }

        columns
    }

    pub fn bingo(&self) -> bool {
        self.has_full_row() || self.has_full_column()
    }

    fn has_full_row(&self) -> bool {
        self.rows
            .iter()
            .any(|row| row.iter().all(Number::is_marked))
    }

    fn has_full_column(&self) -> bool {
        self.columns()
            .iter()
            .any(|col| col.iter().all(Number::is_marked))
    }

    pub fn unmarked_total(&self) -> i32 {
        let mut score = 0;
        for row in self.rows {
            for number in row {
                if let Number::Unmarked(num) = number {
                    score += num;
                }
            }
        }

        score
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Number {
    Unmarked(i32),
    Marked(i32),
}

impl Number {
    fn is_marked(&self) -> bool {
        matches!(self, Number::Marked(_))
    }
}

impl From<i32> for Number {
    fn from(n: i32) -> Self {
        Number::Unmarked(n)
    }
}

impl Default for Number {
    fn default() -> Self {
        Number::Unmarked(0)
    }
}
