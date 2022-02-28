const GOLD_RATIO: f8  = 0.25;
const WALLS_RATIO: f8 = 0.20;

pub mod gold_rush {
    use std::cell::Cell;
    use rand::Rng;
    use matrix::matrix;

    pub type Position = (Cell<u32>,Cell<u32>);

    pub struct BoardProperties {
        gold_amount: u8,
        gold_positions: Vec<Position>,
        walls_amount: u8,
        walls_positions: Vec<Position>,
    }

    impl BoardProperties {
        fn new(board_size: u32) -> BoardProperties {
            let mut rng = rand::thread_rng();
            let gold_positions:Vec<Position> = vec![];
            let walls_positions:Vec<Position> = vec![];
            
            // TODO add unique positions allocation
            BoardProperties {
                gold_amount: rng.gen_range(0..board_size) * GOLD_RATIO,
                walls_amount: rng.gen_range(0..board_size)  * WALLS_RATIO,
                gold_positions: gold_positions,
                walls_amount: walls_positions,
            }
        }
    }

    pub struct Player {
        current_position: Position,
        last_Position: Position,
        score: Cell<u32>
    }

    impl Player {
        fn new(is_player_2: boolean, num_columns: u32, num_rows: u32) -> &Player {
            let mut position = (Cell::new(0),Cell::new(0));
            if  is_player_2 {
                position = (Cell::new(num_rows - 1),Cell::new(num_columns - 1));  
            } 
            Player {
                current_position: position;
                last_Position: position;
                score: Cell::new(0);
            }
        }
    }

    pub struct GoldRush {
        columns: u32,
        rows: u32,
        player_1: Player,
        player_2: Player,
        properties: BoardProperties
    }

    impl GoldRush {
        fn new(cols: usize, rows: usize) -> GoldRush {
            let _matrix = matrix::create_new_matrix(cols, rows);            
            let _player_1 = Player::new(false, cols, rows);
            let _player_2 = Player::new(true, cols, rows);

            GoldRush {
                columns: cols,
                rows: rows,
                player_1: _player_1,
                player_2: _player_2,
                properties: BoardProperties::new(cols*rows)
            }
        }
    }
}