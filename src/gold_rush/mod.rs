const GOLD_RATIO: f8  = 0.25;
const WALLS_RATIO: f8 = 0.20;

pub mod gold_rush {
    use std::cell::Cell;
    use rand::Rng;
    use matrix::matrix;
    use crate::gold_rush::WALLS_RATIO;
    use crate::gold_rush::GOLD_RATIO;
    use crate::matrix;
    use crate::matrix::matrix::Matrix;

    pub type Position = (u32,u32);

    fn is_tile_occupied(coordinate:Position, matrix: matrix::Matrix) -> bool  {
        let point = matrix.get_point(matrix, coordinate[0], coordinate[1]); 
        return match point {
            0 => true,
            _ => false
        }
    }

    // fn get_coordinate() -> Position {
    //
    // }
    //
    // fn set_gold(board: Matrix) -> Matrix {
    //
    // }

    pub struct Player {
        current_position: Position,
        last_position: Position,
        score: Cell<u32>
    }

    impl Player {
        fn new(is_player_2: boolean, num_columns: u32, num_rows: u32) -> Player {
            let mut position: Position = (0,0);
            if  is_player_2 {
                position = ((num_rows - 1),(num_columns - 1));
            } 
            Player {
                current_position: position,
                last_position: position,
                score: Cell::new(0),
            }
        }
    }

    pub struct GoldRush {
        columns: u32,
        rows: u32,
        player_1: Player,
        player_2: Player,
        gold_amount: u8,
        gold_positions: Vec<Position>,
        walls_amount: u8,
        walls_positions: Vec<Position>,
        board: Matrix,
    }

    impl GoldRush {
        fn new(columns: u32, rows: u32) -> GoldRush {
            let mut rng = rand::thread_rng();
            let _matrix = matrix::create_new_matrix(cols, rows);            
            let _player_1 = Player::new(false, columns, rows);
            let _player_2 = Player::new(true, columns, rows);
            let gold_positions= vec![];
            let walls_positions= vec![];

            // TODO add the matrix as with updated properties positions
            GoldRush {
                columns,
                rows,
                player_1: _player_1,
                player_2: _player_2,
                gold_amount: rng.gen_range(0..board_size) * GOLD_RATIO,
                walls_amount: rng.gen_range(0..board_size) * WALLS_RATIO,
                gold_positions,
                walls_positions,
                board: _matrix,
            }
        }

        fn load_board(&self) {

        }
    }
}