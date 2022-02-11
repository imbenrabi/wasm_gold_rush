pub mod matrix {
    #[derive(Debug)]
    pub struct Matrix {
        columns: usize,
        rows: usize,
        data: Vec<Vec<usize>>,
    }
    
    impl Matrix {
        fn new(columns: usize, rows: usize) -> Matrix {
            Matrix {
                columns: columns,
                rows: rows,
                data: vec![vec![0; columns]; rows]
            }
        }
    }

    pub fn create_new_board(cols: usize, rows: usize) -> Matrix {
        Matrix::new(cols, rows)
    }

    pub fn print_matrix(matrix: Matrix) {
        println!("{:?}", matrix);
    }
}
