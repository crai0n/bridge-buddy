mod dds_statistics;
mod double_dummy_runner;

use crate::dds_config::DdsConfig;
use crate::primitives::DoubleDummyResult;
use crate::transposition_table::TranspositionTable;

use crate::double_dummy_solver::dds_statistics::DdsStatistics;
use crate::double_dummy_solver::double_dummy_runner::DoubleDummyRunner;

use bridge_buddy_core::primitives::contract::Strain;
use bridge_buddy_core::primitives::deal::Seat;
use bridge_buddy_core::primitives::Deal;
use enum_iterator::all;
use strum::IntoEnumIterator;

pub struct DoubleDummySolver {
    config: DdsConfig,
    transposition_table: TranspositionTable,
    statistics: DdsStatistics,
}

impl Default for DoubleDummySolver {
    fn default() -> Self {
        Self::new(DdsConfig::default())
    }
}

impl DoubleDummySolver {
    pub fn new(config: DdsConfig) -> Self {
        Self {
            config,
            transposition_table: TranspositionTable::new(),
            statistics: DdsStatistics::default(),
        }
    }

    pub fn new_runner(&self) -> DoubleDummyRunner {
        DoubleDummyRunner::with_config(self.config.clone())
    }

    pub fn solve<const N: usize>(&mut self, deal: Deal<N>) -> DoubleDummyResult {
        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        self.reset_statistics();

        let mut result = DoubleDummyResult::new();

        let mut strain_results = [[0usize; 4]; 5];
        let mut strain_statistics = [DdsStatistics::default(); 5];

        for (index, strain) in all::<Strain>().enumerate() {
            self.transposition_table.clear();
            let mut strain_runner = self.new_runner();
            strain_results[index] = strain_runner.solve_for_all_declarers(deal, strain);
            strain_statistics[index] = strain_runner.get_statistics();
        }

        for (index, strain) in all::<Strain>().enumerate() {
            for declarer in Seat::iter() {
                result.set_tricks_for_declarer_in_strain(strain_results[index][declarer as usize], declarer, strain)
            }
            self.update_statistics(&strain_statistics[index]);
        }

        result
    }

    fn reset_statistics(&mut self) {
        self.statistics = DdsStatistics::default()
    }

    fn update_statistics(&mut self, new: &DdsStatistics) {
        self.statistics.merge(new)
    }

    pub fn get_statistics(&self) -> DdsStatistics {
        self.statistics
    }
}

#[cfg(test)]
mod test {
    use super::DoubleDummySolver;
    use crate::double_dummy_solver::double_dummy_runner::DoubleDummyRunner;
    use bridge_buddy_core::primitives::contract::Strain;
    use bridge_buddy_core::primitives::deal::{Board, Seat};
    use bridge_buddy_core::primitives::{Deal, Hand, Suit};
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case( 30u64, [1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0]; "Test A")]
    #[test_case( 31u64, [1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0]; "Test B")]
    #[test_case( 32u64, [0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0]; "Test C")]
    #[test_case( 33u64, [1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0]; "Test D")]
    #[test_case( 34u64, [0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0]; "Test E")]
    #[test_case( 35u64, [0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0]; "Test F")]
    #[test_case( 36u64, [1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0]; "Test G")]
    #[test_case( 37u64, [1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0]; "Test H")]
    #[test_case( 38u64, [0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0]; "Test I")]
    #[test_case( 39u64, [1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0]; "Test J")]
    #[test_case( 40u64, [0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0]; "Test K")]
    #[test_case( 41u64, [0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0]; "Test L")]
    #[test_case( 42u64, [1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0]; "Test M")]
    #[test_case( 43u64, [0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0]; "Test N")]
    #[test_case( 44u64, [1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0]; "Test O")]
    #[test_case( 45u64, [0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0]; "Test P")]
    #[test_case( 46u64, [0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0]; "Test Q")]
    #[test_case( 47u64, [0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0]; "Test R")]
    #[test_case( 48u64, [1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0]; "Test S")]
    #[test_case( 49u64, [1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0]; "Test T")]
    fn solve1(seed: u64, expected: [usize; 20]) {
        let deal: Deal<1> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let mut dds = DoubleDummySolver::default();
        let dds_result = dds.solve(deal);
        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[ignore]
    #[test]
    fn node_count() {
        const N_AVERAGE: usize = 2000;
        const N_TRICKS: usize = 6;
        // let expected_plys = (N_TRICKS - 1) * 4 + 1;
        let mut dds = DoubleDummySolver::default();

        let mut node_counts = vec![];
        let mut first_move_best_percentages = vec![];

        for _ in 0..N_AVERAGE {
            let deal: Deal<N_TRICKS> = Deal::new();
            let _dds_result = dds.solve(deal);
            let statistics = dds.get_statistics();
            node_counts.push(statistics.get_node_count() as i32);
            if let Some(ratio) = statistics.get_first_move_best_ratio() {
                // println!("First move was best in {}% of nodes.", ratio * 100.0);
                first_move_best_percentages.push(ratio);
            }
        }

        let mean_val = mean(&node_counts).unwrap() / 20f32;

        let std_err = std_error(&node_counts).unwrap() / 20f32;

        let best_mean = mean_f32(&first_move_best_percentages);

        println!("Expanded {} +- {} nodes on average", mean_val, std_err);

        match best_mean {
            Some(ratio) => println!("First move was not best in {}% of tries.", (1.0 - ratio) * 100.0),
            _ => println!("No statistics on move ordering"),
        };
    }

    fn mean(data: &[i32]) -> Option<f32> {
        let sum = data.iter().sum::<i32>() as f32;
        let count = data.len();

        match count {
            positive if positive > 0 => Some(sum / count as f32),
            _ => None,
        }
    }

    fn mean_f32(data: &[f32]) -> Option<f32> {
        let sum = data.iter().sum::<f32>();
        let count = data.len();

        match count {
            positive if positive > 0 => Some(sum / count as f32),
            _ => None,
        }
    }

    fn std_error(data: &[i32]) -> Option<f32> {
        match (mean(data), data.len()) {
            (Some(data_mean), count) if count > 0 => {
                let variance = data
                    .iter()
                    .map(|value| {
                        let diff = data_mean - (*value as f32);

                        diff * diff
                    })
                    .sum::<f32>()
                    / count as f32
                    / count as f32;

                Some(variance.sqrt())
            }
            _ => None,
        }
    }

    #[test_case( 30u64, [1, 2, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0]; "Test A")]
    #[test_case( 31u64, [1, 2, 1, 0, 1, 0, 0, 0, 2, 0, 1, 2, 1, 0, 1, 0, 0, 0, 2, 0]; "Test B")]
    #[test_case( 32u64, [0, 2, 2, 0, 0, 2, 0, 0, 2, 0, 0, 2, 2, 0, 0, 2, 0, 0, 2, 0]; "Test C")]
    #[test_case( 33u64, [0, 0, 2, 1, 0, 1, 2, 0, 1, 0, 1, 0, 2, 1, 1, 1, 2, 0, 0, 0]; "Test D")]
    #[test_case( 34u64, [1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0]; "Test E")]
    #[test_case( 35u64, [0, 1, 1, 2, 0, 2, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0]; "Test F")]
    #[test_case( 36u64, [2, 1, 0, 1, 1, 0, 1, 2, 1, 0, 2, 1, 0, 1, 0, 0, 0, 2, 1, 0]; "Test G")]
    #[test_case( 37u64, [0, 0, 0, 2, 0, 1, 1, 2, 0, 1, 0, 0, 0, 2, 0, 1, 1, 2, 0, 1]; "Test H")]
    #[test_case( 38u64, [0, 1, 1, 2, 1, 2, 0, 1, 0, 0, 0, 1, 1, 2, 1, 2, 0, 0, 0, 0]; "Test I")]
    #[test_case( 39u64, [2, 2, 0, 2, 2, 0, 0, 2, 0, 0, 2, 1, 0, 1, 0, 0, 0, 2, 0, 0]; "Test J")]
    #[test_case( 40u64, [0, 2, 0, 0, 0, 1, 0, 1, 2, 1, 0, 2, 0, 0, 0, 1, 0, 1, 2, 1]; "Test K")]
    #[test_case( 41u64, [1, 0, 0, 2, 0, 0, 2, 1, 0, 0, 1, 0, 1, 2, 1, 1, 2, 1, 0, 0]; "Test L")]
    #[test_case( 42u64, [1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 2, 1, 2, 2, 1, 0, 1, 0, 0]; "Test M")]
    #[test_case( 43u64, [1, 1, 0, 2, 1, 0, 1, 2, 0, 0, 1, 0, 0, 2, 0, 1, 1, 2, 0, 0]; "Test N")]
    #[test_case( 44u64, [2, 1, 0, 1, 1, 0, 1, 2, 1, 1, 2, 1, 0, 1, 0, 0, 0, 2, 1, 0]; "Test O")]
    #[test_case( 45u64, [0, 2, 0, 1, 0, 1, 0, 2, 1, 1, 1, 2, 0, 0, 0, 1, 0, 2, 1, 1]; "Test P")]
    #[test_case( 46u64, [0, 2, 0, 0, 0, 2, 0, 1, 1, 1, 0, 2, 0, 0, 0, 2, 0, 1, 1, 1]; "Test Q")]
    #[test_case( 47u64, [0, 2, 0, 0, 0, 2, 0, 1, 1, 1, 0, 2, 1, 0, 0, 2, 0, 1, 1, 1]; "Test R")]
    #[test_case( 48u64, [1, 2, 1, 0, 1, 0, 0, 1, 2, 0, 1, 2, 1, 0, 0, 1, 0, 1, 2, 1]; "Test S")]
    #[test_case( 49u64, [2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0]; "Test T")]
    fn solve2(seed: u64, expected: [usize; 20]) {
        let deal: Deal<2> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let mut dds = DoubleDummySolver::default();
        let dds_result = dds.solve(deal);
        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    // #[ignore]
    #[test_case( 30u64, [1, 3, 3, 1, 1, 2, 0, 0, 0, 0, 1, 3, 3, 1, 1, 2, 0, 0, 2, 0]; "Test A")]
    #[test_case( 31u64, [2, 3, 0, 0, 0, 0, 0, 3, 2, 0, 2, 3, 0, 0, 0, 0, 0, 3, 2, 0]; "Test B")]
    #[test_case( 32u64, [0, 3, 0, 0, 0, 3, 0, 2, 3, 1, 0, 3, 0, 0, 0, 2, 0, 2, 2, 1]; "Test C")]
    #[test_case( 33u64, [1, 2, 3, 2, 2, 0, 1, 0, 0, 0, 1, 1, 2, 2, 2, 2, 1, 0, 0, 0]; "Test D")]
    #[test_case( 34u64, [0, 1, 1, 2, 0, 3, 2, 2, 1, 3, 0, 1, 1, 0, 0, 3, 2, 2, 1, 3]; "Test E")]
    #[test_case( 35u64, [2, 2, 1, 1, 1, 0, 0, 1, 1, 0, 2, 3, 2, 1, 3, 0, 0, 1, 1, 0]; "Test F")]
    #[test_case( 36u64, [1, 0, 0, 2, 0, 2, 2, 2, 1, 3, 1, 0, 0, 0, 0, 2, 3, 3, 1, 3]; "Test G")]
    #[test_case( 37u64, [0, 2, 0, 2, 0, 3, 1, 3, 1, 2, 0, 2, 0, 0, 0, 3, 1, 3, 1, 2]; "Test H")]
    #[test_case( 38u64, [0, 2, 0, 1, 0, 3, 1, 2, 2, 3, 0, 2, 0, 1, 0, 3, 1, 3, 1, 1]; "Test I")]
    #[test_case( 39u64, [2, 1, 2, 1, 1, 1, 1, 0, 1, 1, 2, 1, 2, 1, 1, 1, 2, 0, 2, 1]; "Test J")]
    #[test_case( 40u64, [0, 0, 2, 3, 0, 2, 3, 1, 0, 1, 0, 0, 2, 3, 0, 2, 3, 1, 0, 1]; "Test K")]
    #[test_case( 41u64, [3, 0, 1, 3, 0, 0, 3, 2, 0, 0, 3, 0, 1, 3, 0, 0, 3, 2, 0, 0]; "Test L")]
    #[test_case( 42u64, [3, 0, 0, 2, 0, 0, 3, 3, 0, 0, 3, 0, 0, 2, 0, 0, 3, 3, 0, 0]; "Test M")]
    #[test_case( 43u64, [2, 3, 1, 3, 2, 1, 0, 2, 0, 0, 2, 2, 1, 3, 2, 1, 0, 0, 0, 0]; "Test N")]
    #[test_case( 44u64, [2, 1, 0, 3, 1, 1, 0, 2, 0, 0, 2, 1, 0, 3, 1, 1, 2, 3, 0, 0]; "Test O")]
    #[test_case( 45u64, [2, 3, 1, 1, 1, 0, 0, 1, 2, 0, 3, 2, 2, 1, 3, 0, 0, 1, 2, 0]; "Test P")]
    #[test_case( 46u64, [0, 0, 2, 2, 1, 2, 2, 0, 0, 0, 0, 0, 3, 2, 1, 2, 2, 0, 0, 0]; "Test Q")]
    #[test_case( 47u64, [1, 3, 1, 1, 1, 2, 0, 2, 2, 1, 1, 3, 1, 1, 1, 0, 0, 2, 1, 0]; "Test R")]
    #[test_case( 48u64, [2, 1, 3, 1, 2, 0, 1, 0, 2, 1, 2, 1, 2, 1, 2, 0, 2, 0, 2, 0]; "Test S")]
    #[test_case( 49u64, [3, 1, 2, 2, 2, 0, 2, 1, 1, 0, 2, 1, 2, 1, 2, 0, 1, 1, 1, 1]; "Test T")]
    fn solve3(seed: u64, expected: [usize; 20]) {
        let deal: Deal<3> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let mut dds = DoubleDummySolver::default();
        let dds_result = dds.solve(deal);

        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[test_case( 30u64, [2, 5, 5, 1, 4, 2, 0, 0, 4, 0, 3, 5, 5, 1, 4, 3, 0, 0, 4, 0]; "Test A")]
    #[test_case( 31u64, [1, 1, 1, 0, 1, 3, 4, 4, 4, 4, 2, 1, 1, 1, 1, 2, 4, 4, 4, 4]; "Test B")]
    #[test_case( 32u64, [1, 4, 1, 3, 1, 3, 0, 3, 2, 0, 1, 5, 1, 3, 2, 3, 0, 3, 1, 3]; "Test C")]
    #[test_case( 33u64, [4, 2, 2, 2, 3, 0, 3, 3, 2, 0, 4, 2, 2, 2, 3, 0, 3, 3, 3, 0]; "Test D")]
    #[test_case( 34u64, [3, 3, 2, 2, 3, 1, 1, 2, 3, 1, 2, 3, 2, 1, 3, 1, 1, 2, 3, 1]; "Test E")]
    #[test_case( 35u64, [4, 1, 1, 3, 2, 1, 3, 2, 1, 2, 4, 1, 2, 3, 2, 1, 3, 2, 1, 2]; "Test F")]
    #[test_case( 36u64, [2, 3, 1, 3, 2, 1, 1, 2, 1, 2, 1, 3, 1, 2, 2, 1, 1, 3, 1, 2]; "Test G")]
    #[test_case( 37u64, [0, 0, 1, 1, 0, 5, 5, 3, 3, 5, 0, 0, 1, 1, 0, 5, 4, 3, 3, 5]; "Test H")]
    #[test_case( 38u64, [0, 1, 4, 3, 1, 4, 3, 1, 2, 1, 0, 1, 4, 3, 0, 4, 3, 1, 2, 1]; "Test I")]
    #[test_case( 39u64, [0, 0, 2, 2, 0, 3, 4, 2, 2, 4, 0, 0, 3, 1, 0, 4, 4, 2, 2, 4]; "Test J")]
    #[test_case( 40u64, [3, 1, 2, 3, 2, 1, 4, 3, 2, 2, 3, 1, 2, 2, 2, 1, 4, 2, 2, 2]; "Test K")]
    #[test_case( 41u64, [2, 1, 1, 3, 1, 3, 2, 2, 2, 3, 2, 1, 1, 2, 1, 2, 2, 4, 1, 3]; "Test L")]
    #[test_case( 42u64, [3, 0, 2, 1, 1, 1, 3, 2, 3, 3, 4, 0, 2, 1, 1, 1, 3, 2, 3, 3]; "Test M")]
    #[test_case( 43u64, [1, 0, 5, 2, 1, 3, 4, 0, 2, 0, 1, 0, 5, 2, 1, 3, 4, 0, 2, 3]; "Test N")]
    #[test_case( 44u64, [3, 3, 3, 0, 0, 1, 1, 2, 5, 3, 4, 3, 3, 0, 0, 1, 0, 2, 5, 3]; "Test O")]
    #[test_case( 45u64, [2, 0, 3, 1, 1, 2, 4, 2, 2, 3, 3, 0, 2, 2, 1, 2, 4, 2, 2, 3]; "Test P")]
    #[test_case( 46u64, [3, 3, 1, 3, 1, 2, 1, 4, 2, 2, 3, 3, 1, 3, 1, 2, 2, 4, 2, 2]; "Test Q")]
    #[test_case( 47u64, [2, 0, 0, 1, 0, 2, 3, 3, 4, 3, 2, 2, 0, 1, 2, 3, 3, 5, 4, 3]; "Test R")]
    #[test_case( 48u64, [3, 2, 3, 2, 2, 2, 3, 2, 3, 2, 3, 2, 3, 2, 2, 2, 3, 2, 3, 2]; "Test S")]
    #[test_case( 49u64, [0, 1, 3, 0, 1, 4, 4, 2, 4, 2, 0, 1, 2, 0, 1, 4, 4, 2, 4, 2]; "Test T")]
    fn solve5(seed: u64, expected: [usize; 20]) {
        let deal: Deal<5> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let mut dds = DoubleDummySolver::default();
        let dds_result = dds.solve(deal);

        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[test_case( 30u64, [1, 3, 3, 1, 1, 5, 3, 3, 5, 4, 1, 3, 3, 1, 1, 5, 3, 3, 5, 4]; "Test A")]
    #[test_case( 31u64, [1, 4, 1, 3, 2, 3, 2, 2, 3, 4, 1, 4, 1, 3, 2, 4, 2, 3, 2, 3]; "Test B")]
    #[test_case( 32u64, [2, 4, 6, 3, 3, 4, 0, 0, 3, 0, 2, 5, 6, 3, 3, 4, 0, 0, 3, 0]; "Test C")]
    #[test_case( 33u64, [5, 3, 4, 4, 5, 1, 2, 1, 1, 1, 5, 3, 4, 4, 5, 0, 2, 1, 0, 0]; "Test D")]
    #[test_case( 34u64, [6, 5, 4, 4, 5, 0, 1, 2, 1, 0, 6, 5, 4, 4, 5, 0, 1, 2, 1, 0]; "Test E")]
    #[test_case( 35u64, [6, 5, 4, 4, 4, 0, 1, 1, 2, 2, 6, 5, 4, 4, 4, 0, 1, 1, 2, 2]; "Test F")]
    #[test_case( 36u64, [5, 6, 5, 6, 6, 1, 0, 1, 0, 0, 5, 6, 5, 6, 6, 1, 0, 1, 0, 0]; "Test G")]
    #[test_case( 37u64, [3, 1, 1, 1, 1, 3, 4, 4, 4, 5, 3, 1, 1, 1, 1, 2, 4, 4, 4, 5]; "Test H")]
    #[test_case( 38u64, [2, 4, 5, 2, 4, 3, 0, 0, 3, 0, 2, 5, 5, 2, 4, 3, 0, 0, 3, 0]; "Test I")]
    #[test_case( 39u64, [5, 3, 3, 4, 3, 1, 3, 3, 2, 1, 5, 3, 3, 4, 3, 0, 3, 3, 2, 3]; "Test J")]
    #[test_case( 40u64, [1, 1, 3, 2, 2, 5, 4, 2, 4, 2, 1, 1, 3, 2, 2, 5, 4, 2, 4, 2]; "Test K")]
    #[test_case( 41u64, [4, 3, 0, 1, 1, 2, 2, 4, 4, 4, 4, 3, 0, 1, 1, 2, 3, 4, 4, 4]; "Test L")]
    #[test_case( 42u64, [4, 3, 2, 6, 3, 2, 2, 3, 0, 0, 4, 3, 2, 6, 3, 2, 2, 3, 0, 3]; "Test M")]
    #[test_case( 43u64, [2, 4, 3, 3, 4, 4, 1, 1, 2, 1, 2, 3, 3, 3, 4, 4, 1, 1, 2, 1]; "Test N")]
    #[test_case( 44u64, [1, 3, 2, 3, 3, 4, 2, 3, 3, 3, 1, 3, 2, 3, 3, 4, 2, 3, 3, 3]; "Test O")]
    #[test_case( 45u64, [1, 4, 2, 5, 3, 3, 1, 4, 1, 3, 2, 4, 2, 5, 3, 3, 1, 4, 1, 3]; "Test P")]
    #[test_case( 46u64, [0, 4, 2, 1, 1, 6, 2, 4, 4, 3, 0, 4, 2, 1, 0, 5, 2, 4, 4, 3]; "Test Q")]
    #[test_case( 47u64, [2, 3, 2, 3, 2, 3, 3, 3, 3, 3, 2, 3, 2, 3, 2, 3, 3, 3, 3, 3]; "Test R")]
    #[test_case( 48u64, [5, 2, 2, 0, 1, 1, 3, 3, 4, 4, 5, 2, 2, 0, 1, 1, 3, 3, 4, 4]; "Test S")]
    #[test_case( 49u64, [3, 0, 4, 2, 1, 1, 6, 1, 4, 4, 3, 0, 4, 2, 1, 1, 6, 1, 4, 4]; "Test T")]
    fn solve6(seed: u64, expected: [usize; 20]) {
        let deal: Deal<6> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let mut dds = DoubleDummySolver::default();
        let dds_result = dds.solve(deal);

        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[test_case( 30u64, [3, 2, 3, 5, 2, 4, 6, 5, 3, 6, 4, 2, 3, 5, 2, 4, 6, 5, 3, 6]; "Test A")]
    #[test_case( 31u64, [6, 6, 1, 4, 4, 2, 2, 7, 3, 3, 6, 6, 1, 4, 4, 2, 2, 7, 3, 3]; "Test B")]
    #[test_case( 32u64, [1, 1, 5, 3, 1, 6, 6, 3, 5, 6, 1, 1, 5, 3, 1, 6, 7, 3, 5, 6]; "Test C")]
    #[test_case( 33u64, [4, 2, 1, 3, 2, 4, 6, 6, 5, 6, 4, 2, 1, 3, 2, 4, 6, 6, 5, 6]; "Test D")]
    #[test_case( 34u64, [4, 2, 1, 0, 1, 3, 6, 7, 8, 6, 4, 2, 1, 0, 1, 3, 6, 6, 8, 6]; "Test E")]
    #[test_case( 35u64, [1, 0, 5, 5, 1, 7, 6, 2, 3, 4, 1, 0, 5, 5, 0, 6, 6, 2, 3, 4]; "Test F")]
    #[test_case( 36u64, [3, 6, 3, 6, 4, 5, 2, 4, 2, 2, 3, 6, 3, 6, 4, 4, 2, 4, 2, 2]; "Test G")]
    #[test_case( 37u64, [4, 2, 1, 1, 1, 4, 5, 7, 7, 6, 4, 2, 1, 1, 1, 4, 5, 7, 7, 6]; "Test H")]
    #[test_case( 38u64, [1, 3, 3, 4, 2, 7, 5, 4, 4, 5, 1, 3, 3, 4, 2, 7, 5, 4, 4, 5]; "Test I")]
    #[test_case( 39u64, [5, 5, 1, 3, 3, 2, 2, 6, 4, 4, 6, 6, 1, 3, 3, 2, 2, 6, 4, 4]; "Test J")]
    #[test_case( 40u64, [5, 5, 5, 4, 5, 1, 1, 2, 3, 3, 5, 5, 5, 4, 5, 1, 1, 2, 3, 3]; "Test K")]
    #[test_case( 41u64, [6, 5, 5, 8, 7, 1, 2, 3, 0, 1, 6, 5, 5, 8, 7, 1, 3, 3, 0, 1]; "Test L")]
    #[test_case( 42u64, [7, 3, 8, 7, 7, 1, 5, 0, 0, 0, 7, 3, 8, 6, 7, 1, 5, 0, 0, 0]; "Test M")]
    #[test_case( 43u64, [5, 6, 6, 6, 6, 2, 1, 1, 2, 1, 5, 6, 6, 6, 6, 2, 2, 1, 2, 1]; "Test N")]
    #[test_case( 44u64, [1, 1, 0, 0, 0, 7, 7, 8, 8, 8, 1, 1, 0, 0, 0, 7, 7, 8, 8, 8]; "Test O")]
    #[test_case( 45u64, [3, 5, 4, 1, 2, 3, 2, 2, 5, 2, 3, 5, 4, 1, 2, 3, 2, 2, 5, 2]; "Test P")]
    #[test_case( 46u64, [3, 4, 1, 6, 3, 5, 3, 6, 2, 3, 3, 4, 1, 6, 3, 4, 3, 6, 2, 3]; "Test Q")]
    #[test_case( 47u64, [1, 0, 0, 4, 0, 7, 8, 7, 4, 7, 1, 0, 0, 4, 0, 7, 8, 7, 3, 3]; "Test R")]
    #[test_case( 48u64, [2, 3, 7, 4, 4, 6, 5, 1, 4, 3, 2, 3, 7, 4, 4, 6, 5, 1, 4, 3]; "Test S")]
    #[test_case( 49u64, [6, 5, 3, 2, 6, 2, 3, 5, 5, 2, 5, 5, 3, 1, 3, 2, 3, 5, 5, 2]; "Test T")]
    fn solve8(seed: u64, expected: [usize; 20]) {
        let deal: Deal<8> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let mut dds = DoubleDummySolver::default();
        let dds_result = dds.solve(deal);

        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[test_case( 30u64, [2, 6, 4, 3, 3, 6, 3, 4, 5, 4, 2, 6, 4, 3, 3, 6, 3, 4, 5, 4]; "Test A")]
    #[test_case( 31u64, [2, 3, 2, 3, 3, 6, 5, 5, 5, 5, 2, 3, 2, 3, 3, 7, 6, 6, 6, 6]; "Test B")]
    #[test_case( 32u64, [0, 0, 5, 4, 0, 8, 8, 4, 5, 8, 0, 0, 5, 4, 0, 8, 8, 4, 5, 6]; "Test C")]
    #[test_case( 33u64, [8, 6, 4, 8, 5, 1, 3, 4, 0, 1, 7, 6, 4, 8, 5, 1, 3, 4, 1, 1]; "Test D")]
    #[test_case( 34u64, [4, 4, 4, 4, 4, 3, 4, 5, 5, 5, 4, 4, 4, 4, 4, 3, 4, 5, 5, 5]; "Test E")]
    #[test_case( 35u64, [5, 6, 6, 8, 7, 3, 3, 3, 0, 2, 5, 6, 6, 8, 7, 4, 3, 3, 0, 2]; "Test F")]
    #[test_case( 36u64, [0, 3, 2, 3, 0, 6, 6, 6, 6, 8, 0, 3, 2, 3, 0, 7, 6, 6, 6, 8]; "Test G")]
    #[test_case( 37u64, [1, 0, 2, 3, 0, 8, 9, 7, 6, 9, 1, 0, 2, 3, 0, 7, 9, 5, 6, 8]; "Test H")]
    #[test_case( 38u64, [6, 7, 7, 8, 8, 3, 1, 2, 1, 1, 6, 7, 7, 8, 8, 3, 1, 2, 1, 1]; "Test I")]
    #[test_case( 39u64, [3, 2, 0, 0, 0, 6, 7, 9, 9, 9, 3, 2, 0, 0, 0, 6, 7, 9, 9, 9]; "Test J")]
    #[test_case( 40u64, [2, 1, 3, 3, 1, 6, 7, 6, 6, 7, 2, 1, 3, 3, 1, 7, 8, 6, 6, 8]; "Test K")]
    #[test_case( 41u64, [2, 1, 1, 1, 1, 7, 8, 8, 7, 8, 2, 1, 1, 1, 1, 7, 8, 8, 7, 8]; "Test L")]
    #[test_case( 42u64, [4, 8, 4, 8, 8, 4, 0, 5, 0, 0, 4, 8, 4, 8, 8, 4, 0, 5, 0, 0]; "Test M")]
    #[test_case( 43u64, [2, 4, 4, 3, 2, 7, 5, 5, 5, 5, 2, 4, 4, 3, 2, 7, 5, 5, 5, 5]; "Test N")]
    #[test_case( 44u64, [5, 2, 2, 4, 3, 4, 7, 7, 5, 6, 5, 2, 2, 4, 3, 4, 7, 7, 5, 6]; "Test O")]
    #[test_case( 45u64, [0, 5, 1, 4, 1, 9, 3, 8, 4, 5, 0, 6, 1, 4, 1, 9, 3, 8, 4, 5]; "Test P")]
    #[test_case( 46u64, [5, 7, 5, 5, 4, 4, 2, 4, 4, 5, 4, 7, 5, 4, 4, 4, 2, 4, 4, 5]; "Test Q")]
    #[test_case( 47u64, [1, 1, 2, 3, 1, 8, 8, 7, 5, 5, 1, 1, 2, 3, 1, 8, 8, 7, 5, 5]; "Test R")]
    #[test_case( 48u64, [6, 3, 4, 7, 5, 0, 6, 5, 0, 0, 6, 3, 4, 7, 5, 0, 6, 5, 0, 0]; "Test S")]
    #[test_case( 49u64, [5, 4, 8, 7, 4, 3, 5, 1, 1, 3, 5, 3, 8, 7, 4, 3, 5, 1, 1, 3]; "Test T")]
    fn solve9(seed: u64, expected: [usize; 20]) {
        let deal: Deal<9> = Deal::from_u64_seed(seed);

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let mut dds = DoubleDummySolver::default();
        let dds_result = dds.solve(deal);

        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[test_case( "S:A", "H:A", "C:A", "D:A", [1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0]; "Test A")]
    fn solve_explicit1(north: &str, east: &str, south: &str, west: &str, expected: [usize; 20]) {
        let north_hand = Hand::<1>::from_str(north).unwrap();
        let east_hand = Hand::<1>::from_str(east).unwrap();
        let south_hand = Hand::<1>::from_str(south).unwrap();
        let west_hand = Hand::<1>::from_str(west).unwrap();

        let deal = Deal {
            board: Board::from_number(1),
            hands: [north_hand, east_hand, south_hand, west_hand],
        };

        let mut dds = DoubleDummySolver::default();
        let dds_result = dds.solve(deal);

        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[test_case( 2u64, Strain::Trump(Suit::Spades), Seat::North, 2; "Test B")]
    fn solve_single5(seed: u64, strain: Strain, declarer: Seat, expected: usize) {
        let deal: Deal<5> = Deal::from_u64_seed(seed);

        let mut ddr = DoubleDummyRunner::default();

        let dds_result = ddr.solve_initial_position(deal, strain, declarer);

        // println!("{}", dds_result);
        assert_eq!(dds_result, expected);
    }

    #[ignore]
    #[test_case( "S:8654,H:J964,D:75,C:K98", "S:J92,H:KT83,D:AK64,C:AQ", "S:AQ7,H:A7,D:QJ83,C:J764", "S:KT3, H:Q52,D:T92,C:T532", [0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0]; "Test A")]
    fn solve_explicit13(north: &str, east: &str, south: &str, west: &str, expected: [usize; 20]) {
        let north_hand = Hand::<13>::from_str(north).unwrap();
        let east_hand = Hand::<13>::from_str(east).unwrap();
        let south_hand = Hand::<13>::from_str(south).unwrap();
        let west_hand = Hand::<13>::from_str(west).unwrap();

        let deal = Deal {
            board: Board::from_number(1),
            hands: [north_hand, east_hand, south_hand, west_hand],
        };

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let mut dds = DoubleDummySolver::default();
        let dds_result = dds.solve(deal);

        // println!("{}", dds_result);
        assert_eq!(dds_result.max_tricks, expected);
    }

    #[ignore]
    #[test_case( "S:8654,H:J964,D:75,C:K98", "S:J92,H:KT83,D:AK64,C:AQ", "S:AQ7,H:A7,D:QJ83,C:J764", "S:KT3, H:Q52,D:T92,C:T532", Strain::NoTrump, Seat::West, 9; "Test A")]
    fn solve_single13(
        north: &str,
        east: &str,
        south: &str,
        west: &str,
        strain: Strain,
        declarer: Seat,
        expected: usize,
    ) {
        let north_hand = Hand::<13>::from_str(north).unwrap();
        let east_hand = Hand::<13>::from_str(east).unwrap();
        let south_hand = Hand::<13>::from_str(south).unwrap();
        let west_hand = Hand::<13>::from_str(west).unwrap();

        let deal = Deal {
            board: Board::from_number(1),
            hands: [north_hand, east_hand, south_hand, west_hand],
        };

        // for (seat, hand) in Seat::iter().zip(deal.hands) {
        //     println!("{}:\n{}", seat, hand)
        // }

        let mut ddr = DoubleDummyRunner::default();

        let ddr_result = ddr.solve_initial_position(deal, strain, declarer);

        // println!("{}", dds_result);
        assert_eq!(ddr_result, expected);
    }
}
