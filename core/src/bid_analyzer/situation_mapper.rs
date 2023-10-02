use super::bidding_situation::BiddingSituation;
use super::situation_rule::SituationRule;
use crate::error::BBError;
use crate::primitives::bid_line::BidLine;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

pub struct SituationMapper {
    ruleset: HashMap<BidLine, BiddingSituation>,
}

impl SituationMapper {
    pub fn new() -> Self {
        let ruleset = HashMap::new();
        SituationMapper { ruleset }
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
        let mut situation_finder = SituationMapper { ruleset };
        for rule in rules {
            situation_finder.add_rule(rule)?;
        }
        Ok(situation_finder)
    }

    pub fn situation_from_bid_line(&self, bid_line: BidLine) -> BiddingSituation {
        match self.ruleset.get(&bid_line) {
            None => BiddingSituation::Unknown,
            Some(sit) => *sit,
        }
    }

    pub fn from_file(path: &str) -> Result<Self, BBError> {
        let mut f = File::open(path)?;
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Self::from_str(&s),
            Err(e) => Err(BBError::IoError(e)),
        }
    }
}

impl FromStr for SituationMapper {
    type Err = BBError;

    fn from_str(s: &str) -> Result<Self, BBError> {
        let rules = s
            .trim()
            .split('\n')
            .map(SituationRule::from_str)
            .collect::<Result<Vec<_>, BBError>>()?;
        SituationMapper::from_rules(&rules)
    }
}

impl Default for SituationMapper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::BiddingSituation;
    use crate::bid_analyzer::situation_mapper::SituationMapper;
    use crate::bid_analyzer::situation_rule::SituationRule;
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
        let sit_finder = SituationMapper::from_str(input).unwrap();
        let contains_all = expected.iter().all(|x| sit_finder.contains(x));
        assert!(contains_all)
    }
}
