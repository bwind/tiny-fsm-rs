use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

struct StateMachine<S> {
    current: S,
    transitions: HashMap<S, HashSet<S>>,
}

impl<S> StateMachine<S>
where
    S: Eq + Hash + Copy,
{
    pub fn new(current: S) -> Self {
        Self {
            current,
            transitions: HashMap::default(),
        }
    }

    pub fn current(&self) -> &S {
        &self.current
    }

    pub fn with_transition(&mut self, from: S, to: S) -> &mut Self {
        self.transitions.entry(from).or_default().insert(to);
        self
    }

    pub fn transition_to(&mut self, to: S) {
        if let Some(value) = self.transitions.get(&self.current) {
            if value.contains(&to) {
                self.current = to;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_machine() {
        let mut sm = StateMachine::new(TrafficLight::Green);
        sm.with_transition(TrafficLight::Green, TrafficLight::Yellow)
            .with_transition(TrafficLight::Yellow, TrafficLight::Red)
            .with_transition(TrafficLight::Red, TrafficLight::Green);

        assert_eq!(sm.current(), &TrafficLight::Green);
        sm.transition_to(TrafficLight::Yellow);
        assert_eq!(sm.current(), &TrafficLight::Yellow);
    }
}
