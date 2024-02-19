#![feature(test)]

#[cfg(test)]
mod test {
    use itertools::Itertools;
    extern crate test;
    use bridge_buddy_dds::card_manager::suit_field::SuitField;
    use test::Bencher;

    #[bench]
    fn is_void(b: &mut Bencher) {
        let val = test::black_box(8192u16);
        b.iter(|| {
            (0..val)
                .map(|v| {
                    let suit_field = SuitField::from_u16(v);
                    suit_field.is_void()
                })
                .collect_vec()
        })
    }

    #[bench]
    fn count_cards(b: &mut Bencher) {
        let val = test::black_box(8192u16);

        b.iter(|| {
            (0..val)
                .map(|v| {
                    let suit_field = SuitField::from_u16(v);
                    suit_field.count_cards()
                })
                .collect_vec()
        })
    }

    #[bench]
    fn all_contained_ranks(b: &mut Bencher) {
        let n = test::black_box(8192u16);

        b.iter(|| {
            (0..n)
                .map(|v| {
                    let suit_field = SuitField::from_u16(v);
                    suit_field.all_contained_ranks()
                })
                .collect_vec()
        })
    }

    #[bench]
    fn bench_highest_rank(b: &mut Bencher) {
        let n = test::black_box(8192u16);

        b.iter(|| {
            (0..n)
                .map(|v| {
                    let suit_field = SuitField::from_u16(v);
                    suit_field.highest_rank()
                })
                .collect_vec()
        })
    }

    #[bench]
    fn bench_lowest_rank(b: &mut Bencher) {
        let n = test::black_box(8192u16);

        b.iter(|| {
            (0..n)
                .map(|v| {
                    let suit_field = SuitField::from_u16(v);
                    suit_field.lowest_rank()
                })
                .collect_vec()
        })
    }
}
