pub mod matrix {
    pub struct Matrix {
        columns: i32,
        rows: i32,
        data: Vec<i32>,
    }
    
    impl Matrix {
        fn new(columns: i32, rows: i32) -> Matrix {
            Matrix {
                columns: columns, rows: rows, data: vec![0, columns * rows] 
            }
        }
    }

    pub fn generate_new_board(cols: i32, rows: i32) -> Matrix {
        Matrix::new(cols, rows)
    }
}
