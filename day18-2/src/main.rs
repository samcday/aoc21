use std::fmt::{Display, Formatter};
use std::io::BufRead;
use std::ops::Add;
use std::str::FromStr;
use itertools::Itertools;

#[derive(Clone, Debug)]
enum PairElement {
    NUM(i64),
    PAIR(Box<Pair>),
}

impl PairElement {
    fn num(&self) -> i64 {
        match self {
            PairElement::NUM(n) => *n,
            _ => panic!("PairElement is not a number"),
        }
    }
}

impl Display for PairElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PairElement::NUM(n) => f.write_str(&n.to_string()),
            PairElement::PAIR(p) => write!(f, "{}", p),
        }
    }
}

impl From<i64> for PairElement {
    fn from(n: i64) -> Self {
        PairElement::NUM(n)
    }
}

impl From<&Pair> for PairElement {
    fn from(p: &Pair) -> Self {
        PairElement::PAIR(Box::new(p.clone()))
    }
}

impl From<Pair> for PairElement {
    fn from(p: Pair) -> Self {
        PairElement::PAIR(Box::new(p))
    }
}

#[derive(Clone, Debug)]
struct Pair {
    lhs: PairElement,
    rhs: PairElement,
}

impl Pair {
    fn new<LHS, RHS>(lhs: LHS, rhs: RHS) -> Pair where LHS: Into<PairElement>, RHS: Into<PairElement> {
        Pair{lhs: lhs.into(), rhs: rhs.into()}
    }

    fn is_regular(&self) -> bool {
        let lhs = match self.lhs { PairElement::NUM(_) => true, _ => false };
        let rhs = match self.rhs { PairElement::NUM(_) => true, _ => false };
        lhs && rhs
    }

    fn magnitude(&self) -> i64 {
        let lhs = match &self.lhs {
            PairElement::NUM(n) => *n,
            PairElement::PAIR(nested) => nested.magnitude(),
        };
        let rhs = match &self.rhs {
            PairElement::NUM(n) => *n,
            PairElement::PAIR(nested) => nested.magnitude(),
        };
        (lhs * 3) + (rhs * 2)
    }
}

impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.as_bytes();

        if s[0] != ('[' as u8) {
            return Err(format!("expected pair to begin with '[', got '{}'", s[0] as char));
        }

        let mut idx = 1usize;

        fn consume_nested_pair(s: &[u8], mut idx: usize) -> Option<(usize, usize)> {
            let start = idx;
            let mut nest_level = 1;
            idx += 1;

            while nest_level > 0 {
                if idx >= s.len() {
                    return None;
                }

                if s[idx] == ('[' as u8) {
                    nest_level += 1;
                }
                if s[idx] == (']' as u8) {
                    nest_level -= 1;
                }

                idx += 1;
            }

            Some((start, idx))
        }

        let lhs: PairElement = if s[idx] == ('[' as u8) {
            let nested = consume_nested_pair(s, idx);
            if nested.is_none() {
                return Err(format!("invalid nested pair starting at idx {}", idx));
            }
            let (start, end) = nested.unwrap();
            idx = end;
            let str = std::str::from_utf8(&s[start..end]).unwrap();
            str.parse::<Pair>()?.into()
        } else {
            let start = idx;
            while s[idx] != (',' as u8) {
                if idx >= s.len() {
                    return Err(format!("malformed pair"));
                }
                idx += 1;
            }
            let str = std::str::from_utf8(&s[start..idx]).unwrap();
            str.parse::<i64>().unwrap().into()
        };

        idx += 1;

        let rhs: PairElement = if s[idx] == ('[' as u8) {
            let nested = consume_nested_pair(s, idx);
            if nested.is_none() {
                return Err(format!("invalid nested pair starting at idx {}", idx));
            }
            let (start, end) = nested.unwrap();
            idx = end;
            Pair::from_str(std::str::from_utf8(&s[start..end]).unwrap())?.into()
        } else {
            let start = idx;
            while s[idx] != (']' as u8) {
                if idx >= s.len() {
                    return Err(format!("malformed pair"));
                }
                idx += 1;
            }
            let str = std::str::from_utf8(&s[start..idx]).unwrap();
            str.parse::<i64>().unwrap().into()
        };

        if s[idx] != (']' as u8) {
            return Err(format!("pair did not close correctly"));
        }

        Ok(Pair::new(lhs, rhs))
    }
}

impl Add for &Pair {
    type Output = Pair;

    fn add(self, rhs: Self) -> Self::Output {
        Pair{lhs: self.into(), rhs: rhs.into()}
    }
}

impl Display for Pair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.lhs, self.rhs)
    }
}

#[derive(Debug)]
enum ReduceOp {
    EXPLODE,
    SPLIT,
}

fn explode(p: &mut Pair, depth: usize) -> Option<(Option<i64>, Option<i64>)> {
    fn do_explode(el: &mut PairElement) -> (i64, i64) {
        let (explode_left, explode_right): (i64, i64) = if let PairElement::PAIR(nested) = el {
            (nested.lhs.num(), nested.rhs.num())
        } else { unreachable!() };
        *el = 0.into();

        return (explode_left, explode_right);
    }

    fn propagate_explode(el: &mut PairElement, lhs_first: bool, val: i64) -> bool {
        return match el {
            PairElement::NUM(n) => {
                *n += val;
                true
            }
            PairElement::PAIR(nested) => {
                if lhs_first && propagate_explode(&mut nested.lhs, lhs_first, val) {
                    return true;
                }
                if propagate_explode(&mut nested.rhs, lhs_first, val) {
                    return true;
                }
                if !lhs_first && propagate_explode(&mut nested.lhs, lhs_first, val) {
                    return true;
                }
                false
            }
        }
    }

    let explode_lhs = if let PairElement::PAIR(nested) = &mut p.lhs {
        depth >= 4 && nested.is_regular()
    } else { false };

    if explode_lhs {
        let (explode_left, explode_right) = do_explode(&mut p.lhs);
        let explode_right = if propagate_explode(&mut p.rhs, true,explode_right) {
            None
        } else { Some(explode_right) };

        return Some((Some(explode_left), explode_right));
    }

    let explode_rhs = if let PairElement::PAIR(nested) = &mut p.rhs {
        depth >= 4 && nested.is_regular()
    } else { false };

    if explode_rhs {
        let (explode_left, explode_right) = do_explode(&mut p.rhs);
        let explode_left = if propagate_explode(&mut p.lhs, false, explode_left) {
            None
        } else { Some(explode_left) };

        return Some((explode_left, Some(explode_right)));
    }

    if let PairElement::PAIR(nested) = &mut p.lhs {
        if let Some((l, r)) = explode(nested, depth + 1) {
            let r = if let Some(explode_right) = r {
                propagate_explode(&mut p.rhs, true,explode_right);
                None
            } else { r };

            return Some((l, r));
        }
    }

    if let PairElement::PAIR(nested) = &mut p.rhs {
        if let Some((l, r)) = explode(nested, depth + 1) {
            let l = if let Some(explode_left) = l {
                propagate_explode(&mut p.lhs, false, explode_left);
                None
            } else { l };
            return Some((l, r));
        }
    }

    None
}

fn split(p: &mut Pair) -> bool {
    match &mut p.lhs {
        PairElement::NUM(v) => {
            if *v >= 10 {
                let half = (*v as f64) / 2.0;
                let l = half.floor();
                let r = half.ceil();
                p.lhs = Pair::new(l as i64, r as i64).into();
                return true;
            }
        }
        PairElement::PAIR(nested) => {
            if split(nested) {
                return true;
            }
        }
    }

    match &mut p.rhs {
        PairElement::NUM(v) => {
            if *v >= 10 {
                let half = (*v as f64) / 2.0;
                let l = half.floor();
                let r = half.ceil();
                p.rhs = Pair::new(l as i64, r as i64).into();
                return true;
            }
        }
        PairElement::PAIR(nested) => {
            if split(nested) {
                return true;
            }
        }
    }

    false
}

fn reduce(p: &mut Pair) -> Option<ReduceOp> {
    if explode(p, 1).is_some() {
        return Some(ReduceOp::EXPLODE);
    }

    if split(p) {
        return Some(ReduceOp::SPLIT);
    }

    None
}

fn main() {
    let stdin = std::io::stdin();
    let pairs = stdin.lock().lines().map(|x| {
        x.unwrap().parse::<Pair>().unwrap()
    }).collect::<Vec<Pair>>();

    let max_magnitude = pairs.iter().permutations(2).map(|x| {
        let mut sum = x[0] + x[1];
        while let Some(_) = reduce(&mut sum) {}
        sum.magnitude()
    }).max().unwrap();
    println!("{}", max_magnitude);
}
