
const GOLD_RATIO: f8  = 0.2;
const WALLS_RATIO: f8 = 0.2;

pub mod gold_rush {
    use matrix::matrix;

    pub type Position = (Cell<u32>,Cell<u32>);
    pub struct BoardProperty {
        amount: u8,
        positions:  Vec<Position>
    }

    pub struct Player {
        start_position: Position,
        last_Position: Position,
        current_position: Position,
        score: usize
    }

    pub struct GoldRush {
        rows: u32,
        columns: u32,
        player_1: Player,
        player_2: Player,
        gold: BoardProperty, 
        walls: BoardProperty,
    }
}