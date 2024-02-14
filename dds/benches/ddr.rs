#![feature(test)]
#[cfg(test)]
mod test {
    extern crate test;

    use bridge_buddy_dds::DoubleDummyRunner;

    use bridge_buddy_core::primitives::Deal;

    use bridge_buddy_core::primitives::contract::Strain;
    use bridge_buddy_core::primitives::deal::Seat;

    use itertools::Itertools;
    use test::Bencher;

    #[bench]
    fn dd_runner(b: &mut Bencher) {
        const DEAL_SIZE: usize = 8;
        const N_SOLVES: usize = 10000;

        b.iter(|| {
            let mut runner = DoubleDummyRunner::default();
            let deals: [usize; N_SOLVES] = (0..N_SOLVES)
                .map(|_| Deal::<DEAL_SIZE>::new())
                .map(|deal| runner.solve_initial_position(deal, Strain::NoTrump, Seat::North))
                .collect_vec()
                .try_into()
                .unwrap();
            deals
        })
    }
}
