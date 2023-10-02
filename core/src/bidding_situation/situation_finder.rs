use super::situation_rule::SituationRule;
use crate::bidding_situation::BiddingSituation;
use crate::error::BBError;
use crate::primitives::bid_line::BidLine;
use std::collections::HashMap;
use std::str::FromStr;

pub struct SituationFinder {
    ruleset: HashMap<BidLine, BiddingSituation>,
}

impl SituationFinder {
    pub fn new() -> Self {
        let ruleset = HashMap::new();
        SituationFinder { ruleset }
    }

    pub fn add_rule(&mut self, rule: &SituationRule) -> Result<(), BBError> {
        match self.ruleset.insert(rule.bid_line.clone(), rule.situation) {
            None => Ok(()),
            Some(old_situation) => Err(BBError::DuplicateRule(rule.bid_line.clone(), old_situation)),
        }
    }

    pub fn contains(&self, rule: &SituationRule) -> bool {
        self.ruleset.get(&rule.bid_line) == Some(&rule.situation)
    }

    pub fn from_rules(rules: &[SituationRule]) -> Result<Self, BBError> {
        let ruleset = HashMap::with_capacity(rules.len());
        let mut situation_finder = SituationFinder { ruleset };
        for rule in rules {
            situation_finder.add_rule(rule)?;
        }
        Ok(situation_finder)
    }

    pub fn find_situation_after(&self, bid_line: BidLine) -> BiddingSituation {
        match self.ruleset.get(&bid_line) {
            None => BiddingSituation::Unknown,
            Some(sit) => *sit,
        }
    }
}

impl FromStr for SituationFinder {
    type Err = BBError;

    fn from_str(s: &str) -> Result<Self, BBError> {
        let rules = s
            .trim()
            .split('\n')
            .map(SituationRule::from_str)
            .collect::<Result<Vec<_>, BBError>>()?;
        SituationFinder::from_rules(&rules)
    }
}

impl Default for SituationFinder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use crate::bidding_situation::situation_finder::SituationFinder;
    use crate::bidding_situation::situation_rule::SituationRule;
    use crate::bidding_situation::BiddingSituation;
    use crate::primitives::bid_line::BidLine;
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case("1S;Answer1Major", &[SituationRule{ bid_line: BidLine::from_str("1S").unwrap(), situation: BiddingSituation::Answer1Major}]; "One Rule")]
    #[test_case("1S;Answer1Major\n1NT;Answer1NoTrump\n1D;Answer1Minor\nP-P;OpeningThirdFourth\n;OpeningFirstSecond\n2NT;Unknown",
        &[SituationRule{ bid_line: BidLine::from_str("1S").unwrap(), situation: BiddingSituation::Answer1Major},
            SituationRule{ bid_line: BidLine::from_str("1NT").unwrap(), situation: BiddingSituation::Answer1NoTrump},
            SituationRule{ bid_line: BidLine::from_str("1D").unwrap(), situation: BiddingSituation::Answer1Minor},
            SituationRule{ bid_line: BidLine::from_str("P-P").unwrap(), situation: BiddingSituation::OpeningThirdFourth},
            SituationRule{ bid_line: BidLine::from_str("").unwrap(), situation: BiddingSituation::OpeningFirstSecond},
            SituationRule{ bid_line: BidLine::from_str("2NT").unwrap(), situation: BiddingSituation::Unknown}];
        "Config Set")]
    fn from_str(input: &str, expected: &[SituationRule]) {
        let sit_finder = SituationFinder::from_str(input).unwrap();
        let contains_all = expected.iter().all(|x| sit_finder.contains(x));
        let contains_vec: Vec<_> = expected.iter().map(|x| sit_finder.contains(x)).collect();
        println!("{:?}", contains_vec);
        assert!(contains_all)
    }
}
