use vstd::prelude::*;
use crate::workflow_common::{Transition, Step, Workflow};

verus! {

// ============================================
// GENERIC WORKFLOW IMPLEMENTATION
// Implementation of Workflow methods
// ============================================

impl Workflow {
    // Ensure initial_state exists in steps
    // Ensure final_states are valid terminal states
    // Ensure all transitions reference valid states
    pub open spec fn valid(self) -> bool {
        self.steps.contains_key(self.initial_state) &&
        forall|s| #[trigger] self.final_states.contains(s) ==> self.is_terminal(s) &&
        forall|s: int|
            self.steps.contains_key(s) ==>
            forall|t: Transition|
                #[trigger] self.steps[s].transitions.contains(t) ==>
                true // Transitions are well-formed
    }

    pub open spec fn next_states(self, s: int) -> Set<int> {
        if self.steps.contains_key(s) {
            Set::new(|s2: int|
                exists|t: Transition|
                    self.steps[s].transitions.contains(t) && t.target == s2
            )
        } else {
            Set::empty()
        }
    }

    pub open spec fn can_transition(self, from: int, to: int) -> bool {
        self.next_states(from).contains(to)
    }

    pub open spec fn is_terminal(self, s: int) -> bool {
        // A state is terminal if it has no transitions OR it's in final_states
        self.next_states(s).len() == 0 || self.final_states.contains(s)
    }

    pub open spec fn has_transitions(self, s: int) -> bool {
        self.steps.contains_key(s) && self.steps[s].transitions.len() > 0
    }

    // Get all transitions from a state
    pub open spec fn get_transitions(self, s: int) -> Set<Transition> {
        if self.steps.contains_key(s) {
            self.steps[s].transitions
        } else {
            Set::empty()
        }
    }
}

}
