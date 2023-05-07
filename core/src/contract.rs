use crate::card::Suit;
use std::cmp::Ordering;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum ContractLevel {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}


#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum ContractState {
    Passed,
    Doubled,
    Redoubled,
}


#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum ContractDenomination {
    Trump(Suit),
    NoTrump,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Contract {
    level: ContractLevel,
    denomination: ContractDenomination,
    state: ContractState
}


#[cfg(test)]
mod test {
    use crate::contract::*;

    #[test]
    fn contract_Ordering_spades_notrump() {
        let level = ContractLevel::One;
        let denomination = ContractDenomination::Trump(Suit::Spades);
        let state = ContractState::Doubled;
        let c1 = Contract { level, denomination, state};
        let level = ContractLevel::One;
        let denomination = ContractDenomination::NoTrump;
        let state = ContractState::Passed;
        let c2 = Contract { level, denomination, state};
        assert_eq!(c1.cmp(&c2), Ordering::Less)
    }

    #[test]
    fn contract_Ordering_hearts_spades() {
        let level = ContractLevel::One;
        let denomination = ContractDenomination::Trump(Suit::Hearts);
        let state = ContractState::Doubled;
        let c1 = Contract { level, denomination, state};
        let level = ContractLevel::One;
        let denomination = ContractDenomination::Trump(Suit::Spades);
        let state = ContractState::Passed;
        let c2 = Contract { level, denomination, state};
        assert_eq!(c1.cmp(&c2), Ordering::Less)
    }
}