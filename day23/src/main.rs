use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Metapod {
    AMBER,
    BRONZE,
    COPPER,
    DESERT
}

impl From<char> for Metapod {
    fn from(c: char) -> Self {
        match c { 'A' => Metapod::AMBER, 'B' => Metapod::BRONZE, 'C' => Metapod::COPPER, 'D' => Metapod::DESERT, _ => panic!() }
    }
}

impl Metapod {
    fn char(&self) -> char {
        match self { Metapod::AMBER => 'A', Metapod::BRONZE => 'B', Metapod::COPPER => 'C', Metapod::DESERT => 'D' }
    }

    fn step_cost(&self) -> usize {
        match self { Metapod::AMBER => 1, Metapod::BRONZE => 10, Metapod::COPPER => 100, Metapod::DESERT => 1000 }
    }

    fn room(&self) -> usize {
        match self { Metapod::AMBER => 0, Metapod::BRONZE => 1, Metapod::COPPER => 2, Metapod::DESERT => 3 }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Room<const N: usize> (Metapod, [Option<Metapod>; N]);

impl<const N: usize> Room<N> {
    fn complete(&self) -> bool {
        self.1.iter().all(|&x| match x { Some(x) => x == self.0, None => false })
    }


    fn top(&self) -> Option<(usize, Metapod)> {
        if self.1.iter().all(|&x| match x { Some(x) => x == self.0, None => true }) {
            return None;
        }

        for (idx, metapod) in self.1.iter().enumerate() {
            if let Some(metapod) = metapod {
                return Some((idx, *metapod));
            }
        }
        None
    }

    fn accept(&self, metapod: Metapod) -> Option<usize> {
        if metapod != self.0 {
            return None;
        }

        for (idx, x) in self.1.iter().enumerate().rev() {
            if let Some(x) = x {
                if *x != metapod {
                    return None;
                }
            } else {
                return Some(idx);
            }
        }
        None
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Hallway([Option<Metapod>; 11]);

impl Hallway {
    // List of metapods in hallway that have an unobstructed path to their room.
    fn waiting(&self) -> Vec<(Metapod, usize)> {
        let mut candidates = vec![];
        for i in 0..11 {
            if let Some(metapod) = self.0[i] {
                let desired_room = metapod.room();
                if self.is_clear(i, Hallway::room_pos(desired_room)) {
                    candidates.push((metapod, i));
                }
            }
        }
        candidates
    }

    fn is_clear(&self, from: usize, to: usize) -> bool {
        let orig_from = from;
        let (from, to) = (std::cmp::min(from, to), std::cmp::max(from, to));

        (from..=to).all(|x| x == orig_from || self.0[x].is_none())
    }

    fn room_pos(room: usize) -> usize {
        room * 2 + 2
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State<const N: usize> {
    hallway: Hallway,
    rooms: [Room<N>; 4],
    cost: usize,
    prev: Option<Box<State<N>>>
}

impl <const N: usize> FromStr for State<N> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        for _ in 0..13 {
            assert_eq!(chars.next().unwrap(), '#');
        }
        assert_eq!(chars.next().unwrap(), '\n');
        assert_eq!(chars.next().unwrap(), '#');

        let mut hallway: [Option<Metapod>; 11] = [None; 11];
        for i in 0..11 {
            let c = chars.next().unwrap();
            hallway[i] = if c == '.' { None } else { Some(c.into()) };
        }
        assert_eq!(chars.next().unwrap(), '#');
        assert_eq!(chars.next().unwrap(), '\n');

        let hallway = Hallway(hallway);

        let mut rooms: [Room<N>; 4] = [
            Room(Metapod::AMBER, [None; N]),
            Room(Metapod::BRONZE, [None; N]),
            Room(Metapod::COPPER, [None; N]),
            Room(Metapod::DESERT, [None; N]),
        ];

        for i in 0..N {
            assert_eq!(chars.next().unwrap(), if i == 0 { '#' } else { ' ' });
            assert_eq!(chars.next().unwrap(), if i == 0 { '#' } else { ' ' });
            assert_eq!(chars.next().unwrap(), '#');

            for j in 0..4 {
                let c = chars.next().unwrap();
                rooms[j].1[i] = if c == '.' { None } else { Some(c.into()) };
                assert_eq!(chars.next().unwrap(), '#');
            }

            if i == 0 {
                assert_eq!(chars.next().unwrap(), '#');
                assert_eq!(chars.next().unwrap(), '#');
            }
            assert_eq!(chars.next().unwrap(), '\n');
        }

        Ok(State{hallway, rooms, cost: 0, prev: None})
    }
}

impl <const N: usize> Display for State<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("#############\n#")?;
        f.write_str(&self.hallway.0.iter().map(|x|
            match x { Some(x) => x.char(), None => '.' }).collect::<String>())?;
        f.write_str("#\n")?;

        for i in 0..N {
            f.write_str(if i == 0 { "###" } else { "  #" })?;
            for j in 0..4 {
                f.write_str(&format!("{}#", match self.rooms[j].1[i] { Some(x) => x.char(), None => '.' }))?;
            }
            f.write_str(if i == 0 { "##\n" } else { "\n" })?;
        }

        f.write_str("  #########")
    }
}

impl <const N: usize> State<N> {
    fn complete(&self) -> bool {
        self.rooms.iter().all(|x| x.complete())
    }

    fn next_states(&self) -> Vec<State<N>> {
        // Anything in hallway that can move straight to its room?
        for (metapod, hallway_pos) in self.hallway.waiting() {
            if let Some(x) = self.rooms[metapod.room()].accept(metapod) {
                let mut new_state = self.clone();
                new_state.prev = Some(Box::new(self.clone()));
                new_state.hallway.0[hallway_pos] = None;
                new_state.rooms[metapod.room()].1[x] = Some(metapod);
                let cost = ((Hallway::room_pos(metapod.room()) as i32 - hallway_pos as i32).abs() as usize + 1 + x) * metapod.step_cost();
                new_state.cost += cost;
                // panic!("hahahaha {}", cost);
                return vec![new_state];
            }
        }

        // Anything in a room that can move straight to its proper room?
        let mut tops = vec![];
        for (room_idx, room) in self.rooms.iter().enumerate() {
            if room.complete() {
                continue;
            }
            if let Some((old_pos, metapod)) = room.top() {
                if self.hallway.is_clear(Hallway::room_pos(room_idx), Hallway::room_pos(metapod.room())) {
                    if let Some(new_pos) = self.rooms[metapod.room()].accept(metapod) {
                        let mut new_state = self.clone();
                        new_state.prev = Some(Box::new(self.clone()));
                        new_state.rooms[room_idx].1[old_pos] = None;
                        new_state.rooms[metapod.room()].1[new_pos] = Some(metapod);
                        new_state.cost += (((((metapod.room() as i32 - room_idx as i32).abs()) as usize * 2) + 1) + new_pos + old_pos + 1) * metapod.step_cost();
                        return vec![new_state];
                    }
                }
                tops.push((room_idx, old_pos, metapod));
            }
        }

        let mut next_states = vec![];
        for (room_idx, pos, metapod) in tops {
            for i in 0..11 {
                // Moving into the space above a room is invalid.
                if i > 1 && i < 9 && i % 2 == 0 {
                    continue;
                }

                let room_hallway_pos = Hallway::room_pos(room_idx);
                if self.hallway.is_clear(room_hallway_pos, i) {
                    let mut new_state = self.clone();
                    new_state.prev = Some(Box::new(self.clone()));
                    new_state.rooms[room_idx].1[pos] = None;
                    new_state.hallway.0[i] = Some(metapod);
                    new_state.cost += (1 + pos + (i as i32 - room_hallway_pos as i32).abs() as usize) * metapod.step_cost();
                    next_states.push(new_state);
                }
            }
        }

        next_states
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();

    let state = input.parse::<State<4>>().unwrap();
    println!("Initial state:\n{}", state);


    // println!("Next states:");
    // for next_state in state.next_states() {
    //     println!("{} ({})", next_state, next_state.cost);
    // }
    // panic!();

    let mut candidates = vec![state];

    let mut cheapest: Option<State<4>> = None;

    while !candidates.is_empty() {
        let candidate = candidates.pop().unwrap();
        if candidate.complete() {
            if let Some(x) = &cheapest {
                if x.cost > candidate.cost {
                    cheapest = Some(candidate);
                }
            } else {
                cheapest = Some(candidate);
            }
            continue;
        }

        if let Some(cheapest) = &cheapest {
            if cheapest.cost < candidate.cost {
                continue;
            }
        }

        for next_state in candidate.next_states() {
            // println!("{}", &next_state);
            candidates.push(next_state);
        }
    }

    let cheapest = cheapest.unwrap();

    let mut steps = vec![];
    let mut walk = Some(&cheapest);
    while let Some(step) = walk {
        steps.push(step);
        walk = step.prev.as_ref().map(|x| x.borrow());
    }
    steps.reverse();
    println!("Solution: {}", cheapest.cost);
    println!("Steps:");
    for step in steps {
        println!("{} ({})\n", step, step.cost);
    }
}
