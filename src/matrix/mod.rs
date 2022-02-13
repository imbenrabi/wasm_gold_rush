pub mod matrix {
    use wasm_bindgen::prelude::*;
    
    pub type Board = Vec<Vec<usize>>;
    
    #[derive(Debug)]
    #[wasm_bindgen]
    pub struct Matrix {
        columns: usize,
        rows: usize,
        data: Board,
    }
    
    impl Matrix {
        fn new(columns: usize, rows: usize) ->  Matrix {
            let mut matrix = Matrix {
                columns: columns,
                rows: rows,
                data: vec![vec![0; columns]; rows]
            };
            matrix
        }

        fn get(&self) -> &Matrix {
            self
        }
    }

    pub fn create_new_matrix(cols: usize, rows: usize) -> Matrix {
        Matrix::new(cols, rows)
    }

    pub fn print_matrix(matrix: Matrix) {
        println!("{:?}", matrix.get());
    }
}
