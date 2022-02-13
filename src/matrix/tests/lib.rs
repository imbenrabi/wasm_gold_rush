#[cfg(test)]
mod matrix_tests {
    use super::super::matrix;
    #[test]
    fn should_get_new_board() {
        let value = vec![vec![0; 3]; 3];
        let board = matrix::create_new_matrix(3, 3);
        matrix::print_matrix(board);
        assert_eq!(board, value);
    }
}
