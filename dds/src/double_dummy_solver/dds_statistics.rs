#[derive(Copy, Clone, Debug, Default)]
pub struct DdsStatistics {
    pub node_count: [usize; 4],
    pub n_first_moves: [usize; 4],
    pub n_first_move_is_best: [usize; 4],
    pub n_one_of_first_two_moves_is_best: [usize; 4],
}

impl DdsStatistics {
    pub fn merge(&mut self, other: &Self) {
        for i in 0..4 {
            self.node_count[i] += other.node_count[i];
            self.n_first_moves[i] += other.n_first_moves[i];
            self.n_first_move_is_best[i] += other.n_first_move_is_best[i];
            self.n_one_of_first_two_moves_is_best[i] += other.n_one_of_first_two_moves_is_best[i];
        }
    }

    pub fn get_first_move_best_ratio(&self) -> Option<f32> {
        let n_first_moves = self.n_first_moves.iter().sum();
        let n_first_move_is_best: usize = self.n_first_move_is_best.iter().sum();
        match n_first_moves {
            0 => None,
            i => {
                // println!("best first moves: {} out of {}", self.n_first_move_is_best, i);
                Some(n_first_move_is_best as f32 / i as f32)
            }
        }
    }

    pub fn get_one_of_first_two_moves_is_best_ratio(&self) -> Option<f32> {
        let n_first_moves = self.n_first_moves.iter().sum();
        let n_first_move_is_best: usize = self.n_one_of_first_two_moves_is_best.iter().sum();
        match n_first_moves {
            0 => None,
            i => {
                // println!("best first moves: {} out of {}", self.n_first_move_is_best, i);
                Some(n_first_move_is_best as f32 / i as f32)
            }
        }
    }

    pub fn get_first_move_best_ratio_per_position(&self) -> [Option<f32>; 4] {
        [0usize, 1, 2, 3].map(|i| {
            match self.n_first_moves[i] {
                0 => None,
                n => {
                    // println!("best first moves: {} out of {}", self.n_first_move_is_best, i);
                    Some(self.n_first_move_is_best[i] as f32 / n as f32)
                }
            }
        })
    }

    pub fn get_one_of_first_two_moves_is_best_ratio_per_position(&self) -> [Option<f32>; 4] {
        [0usize, 1, 2, 3].map(|i| {
            match self.n_first_moves[i] {
                0 => None,
                n => {
                    // println!("best first moves: {} out of {}", self.n_first_move_is_best, i);
                    Some(self.n_one_of_first_two_moves_is_best[i] as f32 / n as f32)
                }
            }
        })
    }

    pub fn get_node_count(&self) -> usize {
        self.node_count.iter().sum()
    }

    pub fn get_node_count_per_position(&self) -> [usize; 4] {
        self.node_count
    }
}
