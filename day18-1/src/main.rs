use std::fmt::{Display, Formatter};
use std::io::BufRead;
use std::ops::Add;
use std::str::FromStr;

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

impl Add for Pair {
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

fn do_homework(mut lines: Vec<&str>) -> i64 {
    lines.reverse();

    let mut sum = None;

    while !lines.is_empty() {
        let pair = lines.pop().unwrap().parse::<Pair>().unwrap();

        if sum.is_none() {
            sum = Some(pair);
        } else {
            let l = sum.take().unwrap();
            let r = pair;
            let mut new_sum = l.clone() + r.clone();
            println!("  {}\n + {}", l, r);
            while let Some(_) = reduce(&mut new_sum) {}
            println!("= {}", new_sum);
            sum = Some(new_sum);
        }
    }

    sum.unwrap().magnitude()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{Pair, PairElement, reduce, do_homework};

    #[test]
    fn display_pair() {
        assert_eq!(format!("{}", Pair::new(1, 2)), "[1,2]");
    }

    #[test]
    fn add_pair() {
        let lhs = Pair::new(1, 2);
        let rhs = Pair::new(Pair::new(3, 4), 5);
        assert_eq!(format!("{}", lhs + rhs), "[[1,2],[[3,4],5]]")
    }

    #[test]
    fn parse() {
        let pair = "[1,2]".parse::<Pair>().unwrap();

        match pair.lhs { PairElement::NUM(n) => assert_eq!(n, 1), _ => panic!("lhs not NUM") }
        match pair.rhs { PairElement::NUM(n) => assert_eq!(n, 2), _ => panic!("rhs not NUM") }
    }

    #[test]
    fn parse_nested() {
        let pair = "[[1,2],[3,4]]".parse::<Pair>().unwrap();

        match pair.lhs { PairElement::PAIR(nested) => {
            match nested.lhs { PairElement::NUM(n) => assert_eq!(n, 1), _ => panic!("nested.lhs not NUM") }
            match nested.rhs { PairElement::NUM(n) => assert_eq!(n, 2), _ => panic!("nested.rhs not NUM") }
        }, _ => panic!("lhs not PAIR") }
        match pair.rhs { PairElement::PAIR(nested) => {
            match nested.lhs { PairElement::NUM(n) => assert_eq!(n, 3), _ => panic!("nested.lhs not NUM") }
            match nested.rhs { PairElement::NUM(n) => assert_eq!(n, 4), _ => panic!("nested.rhs not NUM") }
        }, _ => panic!("rhs not PAIR") }
    }

    #[test]
    fn parse_complex() {
        let str = "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";
        let p = str.parse::<Pair>().unwrap();
        assert_eq!(str, format!("{}", p));
    }

    #[test]
    fn test_nested_addition() {
        let sum = Pair::new(1, 1) + Pair::new(2, 2) + Pair::new(3, 3) + Pair::new(4, 4);
        assert_eq!(format!("{}", sum), "[[[[1,1],[2,2]],[3,3]],[4,4]]");
    }

    #[test]
    fn test_reduce_explode() {
        let mut tests = HashMap::new();
        tests.insert("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        tests.insert("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        tests.insert("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        tests.insert("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        tests.insert("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
        // tests.insert("", "");

        for (orig, expected) in tests.iter() {
            let mut pair = orig.parse::<Pair>().unwrap();
            reduce(&mut pair);
            assert_eq!(&format!("{}", pair), expected);
        }
    }

    #[test]
    fn test_split() {
        let mut pair = Pair::new(11, 123);
        reduce(&mut pair);
        assert_eq!(format!("{}", pair), "[[5,6],123]");


        let mut pair = Pair::new(9, 11);
        reduce(&mut pair);
        assert_eq!(format!("{}", pair), "[9,[5,6]]");
    }

    #[test]
    fn test_i_think_im_sick_of_this_exercise_now() {
        let left = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse::<Pair>().unwrap();
        let right = Pair::new(1, 1);
        let mut pair = left + right;

        println!("{}", pair);

        loop {
            let result = reduce(&mut pair);
            println!("{} (result: {:?})", pair, result);
            if result.is_none() {
                break;
            }
        }

        assert_eq!(format!("{}", pair), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(Pair::new(9, 1).magnitude(), 29);

        let pair = "[[1,2],[[3,4],5]]".parse::<Pair>().unwrap();
        assert_eq!(pair.magnitude(), 143);
    }

    #[test]
    fn test_pls_make_it_end_this_is_a_very_pointless_way_to_spend_my_limited_time() {
        let mut sum = Pair::new(1, 1) + Pair::new(2, 2);
        for x in 3..=5 {
            sum = sum + Pair::new(x, x);
            while let Some(_) = reduce(&mut sum) {}
        }
        assert_eq!(format!("{}", sum), "[[[[3,0],[5,3]],[4,4]],[5,5]]");

        let mut sum = Pair::new(1, 1) + Pair::new(2, 2);
        for x in 3..=6 {
            sum = sum + Pair::new(x, x);
            while let Some(_) = reduce(&mut sum) {}
        }
        assert_eq!(format!("{}", sum), "[[[[5,0],[7,4]],[5,5]],[6,6]]");
    }

    #[test]
    fn test_pls_kill_me_i_am_in_great_pain() {
        let l = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]".parse::<Pair>().unwrap();
        let r = "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]".parse::<Pair>().unwrap();

        let mut sum = l + r;

        println!("START: {}", sum);
        while let Some(op) = reduce(&mut sum) {
            println!("{} ({:?})", sum, op);
        }

        assert_eq!(format!("{}", sum), "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
    }

    #[test]
    fn test_example_homework() {
        let lines = vec!["[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                             "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                             "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                             "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                             "[7,[5,[[3,8],[1,4]]]]",
                             "[[2,[2,2]],[8,[8,1]]]",
                             "[2,9]",
                             "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                             "[[[5,[7,4]],7],1]",
                             "[[[[4,2],2],6],[8,7]]",];

        assert_eq!(do_homework(lines), 3488);
    }

    #[test]
    fn test_example_homework2() {
        let lines = vec!["[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
                             "[[[5,[2,8]],4],[5,[[9,9],0]]]",
                             "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
                             "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
                             "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
                             "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
                             "[[[[5,4],[7,7]],8],[[8,3],8]]",
                             "[[9,3],[[9,9],[6,[4,9]]]]",
                             "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
                             "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",];

        assert_eq!(do_homework(lines), 4140);
    }
}

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap()).collect::<Vec<String>>();

    println!("{}", do_homework(lines.iter().map(|x| x.as_str()).collect::<Vec<&str>>()));
}
