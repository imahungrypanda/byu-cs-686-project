use vstd::prelude::*;

verus! {

// ============================================
// COMMON WORKFLOW DEFINITIONS
// Shared type definitions used by all workflows
// ============================================

// Represents a transition from one state to another
pub struct Transition {
    pub target: int,
}

pub struct Step {
    pub transitions: Set<Transition>,
}

// Generic workflow structure
pub struct Workflow {
    pub initial_state: int,
    pub final_states: Set<int>, // The states that are terminal
    pub steps: Map<int, Step>,
}

}
