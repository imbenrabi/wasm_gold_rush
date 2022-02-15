pub mod matrix {
    use wasm_bindgen::prelude::*;
    
    pub type MatrixData = Vec<Vec<i32>>;

    #[derive(Debug)]
    #[wasm_bindgen]
    pub struct Matrix  {      
        data: MatrixData,
    }
    
    impl Matrix {
        fn new(columns: usize, rows: usize) ->  Matrix {
            let matrix = Matrix {              
                data: vec![vec![0; columns]; rows]
            };
            matrix
        }

        fn get(&self) -> &Matrix {
            &self
        }

        fn view(&self) -> &MatrixData {
            &self.data
        }
    }

    pub fn create_new_matrix(cols: usize, rows: usize) -> Matrix    {
        Matrix::new(cols, rows)
    }

    pub fn print_matrix(matrix: &Matrix) {
        println!("{:?}", matrix.get());
    }

    #[test]
    fn should_get_new_board() {
        let value: MatrixData = vec![vec![0; 3]; 3];
        let board = create_new_matrix(3, 3);
        print_matrix(&board);
        assert_eq!(&board.data, &value);
    }
}
