use std::collections::HashSet;

// https://en.wikipedia.org/wiki/Nondeterministic_finite_automaton#Automaton
struct NFA {
    states: Vec<NFAState>,
    alphabet: Vec<char>,
    transitions: Vec<Vec<Vec<NFAState>>>,
    initial_state: usize,
    accepting_states: HashSet<usize>,
}

impl NFA {
    // https://en.wikipedia.org/wiki/Thompson%27s_construction
    fn new(regex: &str) -> Self {
        todo!()
    }
}

struct NFAState {}
