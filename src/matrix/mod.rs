pub mod matrix {
    pub type Board = Vec<Vec<usize>>;
    
    
    #[derive(Debug)]
    pub struct Matrix {
        columns: usize,
        rows: usize,
        data: Board,
    }
    
    impl Matrix {
        fn new(columns: usize, rows: usize) -> Matrix {
            Matrix {
                columns: columns,
                rows: rows,
                data: vec![vec![0; columns]; rows]
            }
        }

        fn get(&self) -> &Matrix {
            self
        }
    }

    pub fn create_new_board(cols: usize, rows: usize) -> Matrix {
        Matrix::new(cols, rows)
    }

    pub fn print_matrix(matrix: Matrix) {
        println!("{:?}", matrix.get());
    }
}
