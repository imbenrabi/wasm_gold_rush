#[cfg(test)]
mod tests {
    use matrix;
    #[test]
    fn should_return_new_board() {
        let board = matrix::generate_new_board(3, 3);
        println!("board: {:?}", board);
    }
}