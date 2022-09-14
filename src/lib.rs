mod algorithm;

pub use algorithm::Algo;

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}

pub struct Guess<'a> {
    word: &'a str,
    mask: [Correctness; 5],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Correctness {
    Correct,
    Misplaced,
    Wrong,
}

impl Correctness {
    pub fn compute(answer: &str, guess: &str) -> [Self; 5] {
        use std::collections::HashMap;

        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);
        let mut res: [Self; 5] = [Correctness::Wrong; 5];
        let mut used: [bool; 5] = [false; 5];
        let mut track: HashMap<char, u8> = HashMap::new();
        for (i, (a, g)) in answer.chars().zip(guess.chars()).enumerate() {
            let count = track.entry(a).or_insert(0);
            *count += 1;
            if a == g {
                res[i] = Correctness::Correct;
                used[i] = true;
                *count -= 1;
            }
        }
        for (i, g) in guess.chars().enumerate() {
            if used[i] {
                continue;
            }
            if let Some(count) = track.get_mut(&g) {
                if *count > 0 {
                    res[i] = Correctness::Misplaced;
                    *count -= 1;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    mod compute_correctness {
        use crate::Correctness;

        macro_rules! mask {
            (C) => {$crate::Correctness::Correct};
            (M) => {$crate::Correctness::Misplaced};
            (W) => {$crate::Correctness::Wrong};
            ($($x:tt)*) => {
                [$(mask!($x)),*]
            }
        }

        #[test]
        fn two_misplaced() {
            assert_eq!(Correctness::compute("abacd", "aabcd"), mask![C M M C C]);
        }

        #[test]
        fn all_green() {
            assert_eq!(Correctness::compute("abcde", "abcde"), mask![C C C C C]);
        }

        #[test]
        fn all_gray() {
            assert_eq!(Correctness::compute("abcde", "fghij"), mask![W W W W W]);
        }

        #[test]
        fn all_yellow() {
            assert_eq!(Correctness::compute("abcde", "eabcd"), mask![M M M M M]);
        }

        #[test]
        fn repeat_green() {
            assert_eq!(Correctness::compute("aabbb", "aaccc"), mask![C C W W W]);
        }

        #[test]
        fn repeat_yellow() {
            assert_eq!(Correctness::compute("aabbb", "ccaac"), mask![W W M M W]);
        }

        #[test]
        fn repeat_some_green() {
            assert_eq!(Correctness::compute("aabbb", "caacc"), mask![W C M W W]);
        }

        #[test]
        fn dremann_from_chat() {
            assert_eq!(Correctness::compute("azzaz", "aaabb"), mask![C M W W W]);
        }

        #[test]
        fn itsapoque_from_chat() {
            assert_eq!(Correctness::compute("baccc", "aaddd"), mask![W C W W W]);
        }

        #[test]
        fn ricoello_from_chat() {
            assert_eq!(Correctness::compute("abcde", "aacde"), mask![C W C C C]);
        }

        #[test]
        fn thyme_thine() {
            assert_eq!(Correctness::compute("thyme", "thine"), mask![C C W W C]);
        }

        #[test]
        fn brand_frank() {
            assert_eq!(Correctness::compute("brand", "frank"), mask![W C C C W]);
        }

        #[test]
        fn towel_rouge() {
            assert_eq!(Correctness::compute("towel", "rouge"), mask![W C W W M]);
        }
    }
}
