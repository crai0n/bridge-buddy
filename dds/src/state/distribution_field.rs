use crate::state::double_dummy_state::DoubleDummyState;
use crate::state::virtual_card::VirtualCard;
use bridge_buddy_core::primitives::card::rank::N_RANKS;
use bridge_buddy_core::primitives::card::suit::SUIT_ARRAY;
use bridge_buddy_core::primitives::card::virtual_rank::VirtualRank;
use bridge_buddy_core::primitives::deal::seat::SEAT_ARRAY;
use bridge_buddy_core::primitives::deal::Seat;
use bridge_buddy_core::primitives::Suit;
use itertools::Itertools;

pub struct DistFieldManager {
    fields: Vec<DistributionField>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DistributionField([u32; 4]);

#[allow(dead_code)]
impl DistributionField {
    pub fn new(fields: [u32; 4]) -> Self {
        Self(fields)
    }

    pub fn cards_in_suit(&self, suit: Suit) -> u32 {
        self.0[suit as usize]
    }

    pub fn owner_of(&self, card: &VirtualCard) -> Option<Seat> {
        let field = self.0[card.suit as usize];
        let index = card.rank as usize * 2;
        if (card.rank as usize) < N_RANKS - (field as usize >> COUNT_OFFSET) {
            return None;
        }
        match (field >> index) % 4 {
            0 => Some(Seat::North),
            1 => Some(Seat::East),
            2 => Some(Seat::South),
            3 => Some(Seat::West),
            _ => unreachable!(),
        }
    }
}

const COUNT_OFFSET: usize = 26;

#[allow(dead_code)]
impl DistFieldManager {
    pub fn owner_of(&self, card: &VirtualCard) -> Option<Seat> {
        self.fields.last().unwrap().owner_of(card)
    }

    pub fn owner_of_winning_rank_in(&self, suit: Suit) -> Option<Seat> {
        self.fields.last().unwrap().owner_of(&VirtualCard {
            suit,
            rank: VirtualRank::Ace,
        })
    }

    pub fn owner_of_runner_up_in(&self, suit: Suit) -> Option<Seat> {
        self.fields.last().unwrap().owner_of(&VirtualCard {
            suit,
            rank: VirtualRank::King,
        })
    }

    pub fn get_field(&self) -> DistributionField {
        self.fields.last().copied().unwrap()
    }

    pub fn step_back(&mut self) {
        self.fields.pop();
    }

    pub fn new_for_game<const N: usize>(game: &DoubleDummyState<N>) -> Self {
        // println!("starting fields are: ");
        let mut fields = Vec::with_capacity(N + 1);
        let initial = SUIT_ARRAY.map(|suit| {
            let mut field = 0u32;
            for player in SEAT_ARRAY {
                if player != Seat::North {
                    // North's ID is 00 anyway
                    for rank in game.cards_of(player).ranks_in(suit) {
                        let offset = 2 * rank as usize;
                        field |= (player as u32) << offset;
                    }
                }
                let count = game.cards_of(player).count_cards_in(suit) as u32;
                // println!("found {} cards for player {} in suit", count, player);
                field += count << COUNT_OFFSET; // count the cards still in play on bits 30-27
            }

            field
        });
        fields.push(DistributionField(initial));
        Self { fields }
    }

    pub fn remove_cards(&mut self, cards: impl Iterator<Item = VirtualCard>) {
        self.fields.push(self.fields.last().copied().unwrap());
        for card in cards.sorted_unstable_by_key(|card| card.rank) {
            self.remove_card(card)
        }
    }

    pub fn add_cards(&mut self, cards: impl Iterator<Item = VirtualCard>, last_leader: Seat) {
        self.fields.push(self.fields.last().copied().unwrap());
        for (index, card) in cards.enumerate().sorted_unstable_by_key(|(_, card)| card.rank).rev() {
            self.add_card(card, last_leader + index)
        }
    }

    pub fn remove_rank(&mut self, suit: Suit, virtual_rank: VirtualRank) {
        let before = self.fields.last().unwrap().0[suit as usize];
        let after = Self::without_rank(before, virtual_rank);
        self.fields.last_mut().unwrap().0[suit as usize] = after;
    }

    pub fn remove_card(&mut self, virtual_card: VirtualCard) {
        self.remove_rank(virtual_card.suit, virtual_card.rank)
    }

    pub fn add_rank(&mut self, suit: Suit, virtual_rank: VirtualRank, owner: Seat) {
        let before = self.fields.last().unwrap().0[suit as usize];
        let after = Self::with_added_rank(before, virtual_rank, owner);
        self.fields.last_mut().unwrap().0[suit as usize] = after;
    }

    pub fn add_card(&mut self, virtual_card: VirtualCard, owner: Seat) {
        self.add_rank(virtual_card.suit, virtual_card.rank, owner)
    }

    fn without_rank(field: u32, virt_rank: VirtualRank) -> u32 {
        let index = 2 * virt_rank as usize;
        let lower_mask = (1 << index) - 1;
        // println!("lower mask:");
        // println!("{:32b}", lower_mask);
        let index_mask = 3 << index;
        // println!("index mask:");
        // println!("{:32b}", index_mask);
        let upper_mask = !(lower_mask | index_mask);
        // println!("{:32b}", upper_mask);

        let upper_field = field & upper_mask;
        let lower_field = (field & lower_mask) << 2;

        let suit_distribution = upper_field | lower_field;

        suit_distribution - (1 << COUNT_OFFSET) // lower count
    }

    fn with_added_rank(field: u32, virt_rank: VirtualRank, owner: Seat) -> u32 {
        let index = 2 * (virt_rank as usize);
        let lower_mask = (1 << (index + 2)) - 1;
        let upper_mask = !lower_mask;

        let upper_field = field & upper_mask;
        let lower_field = (field & lower_mask) >> 2;

        let suit_distribution = upper_field | lower_field | ((owner as u32) << index);

        suit_distribution + (1 << COUNT_OFFSET) // increase count
    }
}
