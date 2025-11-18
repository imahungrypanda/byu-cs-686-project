use vstd::prelude::*;

verus! {

enum State {
  NotDone,    // still in pre-switch region
  Switched,   // success, ownership flipped
  Failed,     // terminal failure
}

struct Step {
    next_states: Set<State>, // The states that can be transitioned to from this state
}

// The key is the state you are currently in,
// and the value is the Step describing transitions *from* that state.
struct Workflow {
  initial_state: State,
  final_states: Set<State>, // The states that are terminal
  steps: Map<State, Step>,
}

impl Workflow {
    spec fn valid(self) -> bool {
        // TODO: Do I care about no self-loops, or ensuring certain states are reachable
        true
    }

    spec fn next_states(self, s: State) -> Set<State> {
        if self.steps.contains_key(s) {
            self.steps[s].next_states
        } else {
            Set::empty()
        }
    }

    spec fn can_transition(self, from: State, to: State) -> bool {
        self.next_states(from).contains(to)
    }

    spec fn is_terminal(self, s: State) -> bool {
        self.next_states(s).len() == 0
    }

    spec fn has_transitions(self, s: State) -> bool {
        self.steps.contains_key(s)
    }
}

fn main() {
  // TODO: Build out the workflow in terms of steps and states

  // Your main code here
  assert(true)
}

}
