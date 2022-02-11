use matrix;

#[cfg(test)]
mod tests {
    #[test]
    fn should_return_new_board() {
        let board = matrix::generate_new_board(3, 3);
        matrix::print_board(board);
    }
}