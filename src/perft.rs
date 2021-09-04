use crate::state::State;

impl State {
    /// Perft (Performance Test) function.
    /// Used to make sure that the move generation generates the right amount of moves.
    pub fn perft(&mut self, depth: u8) -> u64 {
        let mut nodes = 0;
        if depth == 0 {
            return 1;
        }
        for x in self.get_moves() {
            self.make_move(x);
            nodes += self.perft(depth - 1);
            self.undo_move(x);
        }
        nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perft_total_depth_nine() {
        assert_eq!(State::new().perft(9), 5478);
    }
}
