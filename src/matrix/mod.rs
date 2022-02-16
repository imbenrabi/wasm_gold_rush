pub mod matrix {
    use std::cell::Cell;
    use wasm_bindgen::prelude::*;
    
    
    pub type Point = Cell<u32>;
    pub type MatrixData = Vec<Vec<Point>>;

    #[derive(Debug)]
    #[wasm_bindgen]
    pub struct Matrix  {      
        data: MatrixData,
    }
    
    impl Matrix {
        fn new(columns: usize, rows: usize) ->  Matrix {
            let matrix = Matrix {              
                data: vec![vec![Cell::new(0); columns]; rows]
            };
            matrix
        }

        fn get_point(&self, x:usize, y:usize) -> &Point {
            let point = &self.data[x][y];
            point
        }

        fn view(&self) -> &MatrixData {
            &self.data
        }
    }

    pub fn create_new_matrix(cols: usize, rows: usize) -> Matrix    {
        Matrix::new(cols, rows)
    }

    pub fn print_point(matrix: &Matrix, x:usize, y: usize) {
        println!("{:?}", matrix.get_point(x,y));
    }

    #[test]
    fn should_get_new_board() {
        let value: MatrixData = vec![vec![Cell::new(0); 3]; 3];
        let board = create_new_matrix(3, 3);
        assert_eq!(&board.data, &value);
    }
}
