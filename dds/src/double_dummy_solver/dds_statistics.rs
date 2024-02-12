#[derive(Copy, Clone, Debug, Default)]
pub struct DdsStatistics {
    pub node_count: usize,
    pub n_first_moves: usize,
    pub n_first_move_is_best: usize,
}

impl DdsStatistics {
    pub fn merge(&mut self, other: &Self) {
        self.node_count += other.node_count;
        self.n_first_moves += other.n_first_moves;
        self.n_first_move_is_best += other.n_first_move_is_best;
    }

    pub fn get_first_move_best_ratio(&self) -> Option<f32> {
        match self.n_first_moves {
            0 => None,
            i => {
                // println!("best first moves: {} out of {}", self.n_first_move_is_best, i);
                Some(self.n_first_move_is_best as f32 / i as f32)
            }
        }
    }

    pub fn get_node_count(&self) -> usize {
        self.node_count
    }
}
