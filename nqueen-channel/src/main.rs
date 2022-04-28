use std::sync::mpsc::{self, Sender};

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

fn queen(n: usize, starting_col: usize, tx: &Sender<Vec<usize>>) {
    debug_assert!(starting_col < n);
    let mut state = QueenState::new(n);
    state.put(0, starting_col);
    fn dfs(state: &mut QueenState, n: usize, row: usize, tx: &Sender<Vec<usize>>) {
        if row == n {
            let vec = state.board.iter().map(|x| x.unwrap()).collect::<Vec<_>>();
            tx.send(vec).unwrap();
            return;
        }
        for col in 0..n {
            if state.check(row, col) {
                state.put(row, col);
                dfs(state, n, row + 1, tx);
                state.unput(row, col);
            }
        }
    }
    dfs(&mut state, n, 1, tx);
}

fn main() {
    const SIZE: usize = 15;
    const THREAD: usize = 4;
    let mut handles = vec![];
    let (tx, rx) = mpsc::channel();
    for i in 0..THREAD {
        let range = (i * SIZE / THREAD)..((i + 1) * SIZE / THREAD);
        let tx = tx.clone();
        let handle = std::thread::spawn(move || {
            for j in range {
                queen(SIZE, j, &tx);
            }
        });
        handles.push(handle);
    }
    std::mem::drop(tx);
    for received in rx {
        println!("{:?}", received);
    }
}
