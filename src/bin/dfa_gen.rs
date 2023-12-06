use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Input {
    Char(char),
    Epsilon,
}

type State = u64;

#[derive(Debug)]
struct Nfa<O> {
    transitions: BTreeMap<(State, Input), BTreeSet<State>>,
    end_states: BTreeMap<State, O>,
    state_count: u64,
}

impl<O: Clone> Nfa<O> {
    fn new() -> Self {
        Self {
            transitions: BTreeMap::new(),
            end_states: BTreeMap::new(),
            state_count: 1,
        }
    }

    fn start_state(&self) -> State {
        0
    }

    fn add_state(&mut self) -> State {
        let state = self.state_count;
        self.state_count += 1;
        state
    }

    fn add_end_state(&mut self, output: O) -> State {
        let state = self.add_state();
        self.end_states.insert(state, output);
        state
    }

    fn make_end_state(&mut self, state: State, output: O) {
        self.end_states.insert(state, output);
    }

    fn add_transition(&mut self, input: Input, from: State, to: State) {
        let to_states = self
            .transitions
            .entry((from, input))
            .or_insert(BTreeSet::new());
        to_states.insert(to);
    }

    fn simulate_step(
        &self,
        current_states: BTreeSet<State>,
        input: char,
    ) -> (BTreeSet<State>, Vec<O>) {
        let mut next_states = BTreeSet::new();
        let mut outputs = Vec::new();
        for &state in current_states.iter().flat_map(|&s| {
            self.transitions
                .get(&(s, Input::Char(input)))
                .into_iter()
                .flatten()
        }) {
            self.add_epsilon_neighbours(state, &mut next_states, &mut outputs);
        }
        (next_states, outputs)
    }

    fn add_epsilon_neighbours(
        &self,
        state: State,
        next_states: &mut BTreeSet<State>,
        outputs: &mut Vec<O>,
    ) {
        outputs.extend(self.end_states.get(&state).cloned());
        if next_states.insert(state) {
            for &neighbour in self
                .transitions
                .get(&(state, Input::Epsilon))
                .into_iter()
                .flatten()
            {
                self.add_epsilon_neighbours(neighbour, next_states, outputs);
            }
        }
    }
}

fn main() {
    let digits = [
        ("one", '1', 1),
        ("two", '2', 2),
        ("three", '3', 3),
        ("four", '4', 4),
        ("five", '5', 5),
        ("six", '6', 6),
        ("seven", '7', 7),
        ("eight", '8', 8),
        ("nine", '9', 9),
    ];

    let mut nfa = Nfa::new();
    let start = nfa.start_state();
    for (name, digit, val) in &digits {
        let mut previous_state = start;
        for c in name.chars() {
            let current_state = nfa.add_state();
            nfa.add_transition(Input::Char(c), previous_state, current_state);
            nfa.add_transition(Input::Epsilon, current_state, start);
            previous_state = current_state;
        }
        nfa.make_end_state(previous_state, val);
        nfa.add_transition(Input::Char(*digit), start, previous_state);
    }
    for c in 'a'..='z' {
        nfa.add_transition(Input::Char(c), start, start);
    }

    let mut states = BTreeSet::new();
    nfa.add_epsilon_neighbours(start, &mut states, &mut Vec::new());
    let line = "8seveninyeightwox4";
    println!("scanning line: {line}");
    println!();
    for c in line.chars() {
        let outputs;
        (states, outputs) = nfa.simulate_step(states, c);
        println!("{states:?} -> {outputs:?}");
    }
}
