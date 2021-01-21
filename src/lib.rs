use core::fmt::Debug;
use std::cmp;
use std::str;

#[derive(Debug)]
pub enum LexoRankKind {
    FIGMA,
    //ATLASIAN,
}

impl Default for LexoRankKind {
    fn default() -> Self {
        LexoRankKind::FIGMA
    }
}

#[derive(Debug)]
pub struct LexoRank {
    lexorank_strategy: Box<dyn LexoRankStrategy>,
    kind: LexoRankKind,
}

impl Debug for dyn LexoRankStrategy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "LexoRankStrategy")
    }
}

trait LexoRankStrategy {
    fn compare_positions(&self, first_pos: &str, second_pos: &str) -> cmp::Ordering;
    fn is_valid_position(&self, pos: &str) -> bool;
    fn position_before(&self, pos: &str) -> String;
    fn position_after(&self, pos: &str) -> String;
    fn position_between(&self, first_pos: &str, second_pos: &str) -> String;
}

impl LexoRank {
    fn new(kind: LexoRankKind) -> LexoRank {
        match kind {
            LexoRankKind::FIGMA => LexoRank {
                kind: kind,
                lexorank_strategy: Box::new(FigmaStrategy {}),
            },
        }
    }
    fn compare_positions(&self, first_pos: &str, second_pos: &str) -> cmp::Ordering {
        self.lexorank_strategy
            .compare_positions(first_pos, second_pos)
    }

    fn is_valid_position(&self, pos: &str) -> bool {
        self.lexorank_strategy.is_valid_position(pos)
    }

    fn position_before(&self, pos: &str) -> String {
        self.lexorank_strategy.position_before(pos)
    }
    fn position_after(&self, pos: &str) -> String {
        self.lexorank_strategy.position_after(pos)
    }
    fn position_between(&self, first_pos: &str, second_pos: &str) -> String {
        self.lexorank_strategy
            .position_between(first_pos, second_pos)
    }
}

#[derive(Debug)]
struct FigmaStrategy {}

impl FigmaStrategy {
    const START_CHAR_CODE: u8 = 32;
    const END_CHAR_CODE: u8 = 126;

    fn avg(a: u8, b: u8) -> u8 {
        return (a + b) / 2;
    }
}

impl LexoRankStrategy for FigmaStrategy {
    fn compare_positions(&self, first_pos: &str, second_pos: &str) -> cmp::Ordering {
        first_pos.cmp(second_pos)
    }
    fn is_valid_position(&self, pos: &str) -> bool {
        // We convert to bytes since the allowed alphabet
        // is in the visible ASCII
        let pos_bytes = pos.as_bytes();

        if (pos.is_empty()) || (pos_bytes[pos.len() - 1] == Self::START_CHAR_CODE) {
            return false;
        }

        for c in pos_bytes {
            // println!("{:?}", c as &u8);
            if *c < Self::START_CHAR_CODE || *c > Self::END_CHAR_CODE {
                return false;
            }
        }
        true
    }

    fn position_before(&self, pos: &str) -> String {
        let pos_bytes = pos.as_bytes();

        for (i, c) in pos_bytes.iter().enumerate().rev() {
            if *c > Self::START_CHAR_CODE + 1 {
                let position = pos[0..i].to_string() + str::from_utf8(&[(*c - 1)]).unwrap();
                return position;
            }
        }

        let position = pos[0..pos.len() - 1].to_string()
            + str::from_utf8(&[Self::START_CHAR_CODE]).unwrap()
            + str::from_utf8(&[Self::END_CHAR_CODE]).unwrap();

        return position;
    }

    fn position_after(&self, pos: &str) -> String {
        let pos_bytes = pos.as_bytes();
        for (i, c) in pos_bytes.iter().enumerate().rev() {
            if *c < Self::END_CHAR_CODE {
                let position = pos[0..i].to_string() + str::from_utf8(&[(*c + 1)]).unwrap();
                return position;
            }
        }
        let position = pos.to_string() + str::from_utf8(&[(Self::START_CHAR_CODE + 1)]).unwrap();
        return position;
    }

    fn position_between(&self, first_pos: &str, second_pos: &str) -> String {
        let mut flag = false;
        let mut position = String::new();
        let first_pos_len = first_pos.len();
        let second_pos_len = second_pos.len();
        let first_pos_bytes = first_pos.as_bytes();
        let second_pos_bytes = second_pos.as_bytes();
        let max_len = cmp::max(first_pos_len, second_pos_len);

        for i in 0..max_len {
            let lower = if i < first_pos_len {
                first_pos_bytes[i]
            } else {
                Self::START_CHAR_CODE
            };
            let upper = if i < second_pos_len && !flag {
                second_pos_bytes[i]
            } else {
                Self::END_CHAR_CODE
            };
            if lower == upper {
                position += str::from_utf8(&[lower]).unwrap();
            } else if upper - lower > 1 {
                position += str::from_utf8(&[Self::avg(lower, upper)]).unwrap();
                flag = false;
                break;
            } else {
                position += str::from_utf8(&[lower]).unwrap();
                flag = true;
            }
        }

        if flag {
            position +=
                str::from_utf8(&[Self::avg(Self::START_CHAR_CODE, Self::END_CHAR_CODE)]).unwrap();
        }
        return position;
    }
}

#[cfg(test)]
mod tests {
    use super::{LexoRank, LexoRankKind};
    use std::cmp;

    #[test]
    fn test_compare_positions() {
        let lexrank = LexoRank::new(LexoRankKind::FIGMA);
        assert_eq!(lexrank.compare_positions("AA", "AB"), cmp::Ordering::Less);
        assert_eq!(lexrank.compare_positions("AA", "AA"), cmp::Ordering::Equal);
        assert_eq!(
            lexrank.compare_positions("AA", "A0"),
            cmp::Ordering::Greater
        );
    }

    #[test]
    fn test_is_valid_position() {
        let lexrank = LexoRank::new(LexoRankKind::FIGMA);
        assert_eq!(lexrank.is_valid_position("AA"), true);
        assert_eq!(lexrank.is_valid_position("!"), true);
        assert_eq!(lexrank.is_valid_position("~"), true);
        // Character not in ASCII 32-126
        assert_eq!(lexrank.is_valid_position("¡"), false);
    }
    #[test]
    fn test_position_before() {
        let lexrank = LexoRank::new(LexoRankKind::FIGMA);
        assert_eq!(lexrank.position_before("C"), "B");
        assert_eq!(lexrank.position_before("AA"), "A@");
        assert_eq!(lexrank.position_before("!"), " ~");
    }
    #[test]
    fn test_position_after() {
        let lexrank = LexoRank::new(LexoRankKind::FIGMA);
        assert_eq!(lexrank.position_after("C"), "D");
        assert_eq!(lexrank.position_after("AA"), "AB");
        assert_eq!(lexrank.position_after("~"), "~!");
    }
    #[test]
    fn test_position_between() {
        let lexrank = LexoRank::new(LexoRankKind::FIGMA);
        assert_eq!(lexrank.position_between("A", "C"), "B");
        assert_eq!(lexrank.position_between("AA", "AB"), "AAO");
    }
}
