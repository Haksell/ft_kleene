use std::collections::HashSet;

// https://en.wikipedia.org/wiki/Nondeterministic_finite_automaton#Automaton
struct FiniteAutomaton {
    states: Vec<State>,
    alphabet: Vec<char>,
    transitions: Vec<Vec<Vec<State>>>,
    initial_state: usize,
    accepting_states: HashSet<usize>,
}

impl FiniteAutomaton {
    fn new(regex: &str) -> Self {
        todo!()
    }
}

struct State {}
