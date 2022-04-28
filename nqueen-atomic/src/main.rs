use std::sync::Arc;

use std::sync::atomic::{AtomicUsize, Ordering};

struct QueenState {
    board: Vec<Option<usize>>,
    col: Vec<bool>,
    diag: Vec<bool>,     // x - y
    antidiag: Vec<bool>, // x + y
    n: usize,
}

macro_rules! diag_index {
    ($row: expr, $col: expr, $n: expr) => {
        // $row - $col + $n - 1
        $row + $n - $col - 1
    };
}

impl QueenState {
    fn new(n: usize) -> Self {
        QueenState {
            board: vec![None; n],
            col: vec![false; n],
            diag: vec![false; 2 * n - 1],
            antidiag: vec![false; 2 * n - 1],
            n,
        }
    }

    // fn diag_index(&self, row: usize, col: usize) -> usize {
    //     row - col + self.n - 1
    // }

    fn put(&mut self, row: usize, col: usize) {
        debug_assert!(self.board[row] == None);
        debug_assert!(row < self.n && col < self.n);
        self.board[row] = Some(col);
        self.col[col] = true;
        self.diag[diag_index!(row, col, self.n)] = true;
        self.antidiag[row + col] = true;
    }

    fn unput(&mut self, row: usize, col: usize) {
        debug_assert!(self.board[row] == Some(col));
        debug_assert!(row < self.n && col < self.n);
        self.board[row] = None;
        self.col[col] = false;
        self.diag[diag_index!(row, col, self.n)] = false;
        self.antidiag[row + col] = false;
    }

    fn check(&self, row: usize, col: usize) -> bool {
        debug_assert!(row < self.n && col < self.n);
        !self.col[col] && !self.diag[diag_index!(row, col, self.n)] && !self.antidiag[row + col]
    }
}

fn queen(n: usize, starting_col: usize, cnt: &Arc<AtomicUsize>) {
    debug_assert!(starting_col < n);
    let mut state = QueenState::new(n);
    state.put(0, starting_col);
    fn dfs(state: &mut QueenState, n: usize, row: usize, cnt: &Arc<AtomicUsize>) {
        if row == n {
            cnt.fetch_add(1, Ordering::AcqRel);
            return;
        }
        for col in 0..n {
            if state.check(row, col) {
                state.put(row, col);
                dfs(state, n, row + 1, cnt);
                state.unput(row, col);
            }
        }
    }
    dfs(&mut state, n, 1, cnt);
}

fn main() {
    const SIZE: usize = 15;
    const THREAD: usize = 4;
    let mut handles = vec![];
    let result = Arc::new(AtomicUsize::new(0));
    for i in 0..THREAD {
        let range = (i * SIZE / THREAD)..((i + 1) * SIZE / THREAD);
        let cnt = result.clone();
        let handle = std::thread::spawn(move || {
            for j in range {
                queen(SIZE, j, &cnt);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("{}", result.load(Ordering::Acquire));
}
