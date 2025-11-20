use vstd::prelude::*;
use crate::workflow_common::{Transition, Step, Workflow};

verus! {

// ============================================
// PURCHASE WORKFLOW SPECIFIC
// ============================================

// Purchase workflow states
enum State {
    StartState,
    InitPurchasePending,
    Negotiations,
    PurchaseFailed,
    OwnershipsSwitched,
}

// Mapping function: convert State enum to integer representation
spec fn state_to_int(s: State) -> int {
    match s {
        State::StartState => 0,
        State::InitPurchasePending => 1,
        State::Negotiations => 2,
        State::PurchaseFailed => 3,
        State::OwnershipsSwitched => 4,
    }
}

// Purchase workflow context - used to evaluate transition conditions
struct PurchaseContext {
    payment_owner: int,
    buyer_name: int,
    seller_name: int,
    backpack_owner: int,
    terms: int,          // Values: pending, agreed, noRetry
    payment_offered: int, // Values: pending, paymentAmount, noRetry
    payment_amount: int,
}

// Constants for purchase workflow condition values
spec const PENDING: int = 0;
spec const AGREED: int = 1;
spec const NORETRY: int = 2;

// Helper functions to evaluate purchase workflow transition conditions
spec fn purchase_condition_1(ctx: PurchaseContext) -> bool {
    // payment_owner == buyer_name && backpack_owner == seller_name
    ctx.payment_owner == ctx.buyer_name && ctx.backpack_owner == ctx.seller_name
}

spec fn purchase_condition_2(ctx: PurchaseContext) -> bool {
    // terms != pending || payment_offered != pending
    ctx.terms != PENDING || ctx.payment_offered != PENDING
}

spec fn purchase_condition_3(ctx: PurchaseContext) -> bool {
    // terms == noRetry || payment_offered == noRetry
    ctx.terms == NORETRY || ctx.payment_offered == NORETRY
}

spec fn purchase_condition_4(ctx: PurchaseContext) -> bool {
    // terms == agreed && payment_offered == payment_amount
    ctx.terms == AGREED && ctx.payment_offered == ctx.payment_amount
}

spec fn purchase_condition_5(ctx: PurchaseContext) -> bool {
    // payment_owner == seller_name && backpack_owner == buyer_name
    ctx.payment_owner == ctx.seller_name && ctx.backpack_owner == ctx.buyer_name
}

// Create the purchase workflow with all states and transitions
spec fn example_purchase_workflow() -> Workflow {
    let start_state = state_to_int(State::StartState);
    let init_state = state_to_int(State::InitPurchasePending);
    let negotiations_state = state_to_int(State::Negotiations);
    let failed_state = state_to_int(State::PurchaseFailed);
    let switched_state = state_to_int(State::OwnershipsSwitched);

    let start_step = Step {
        transitions: set![Transition { target: init_state }],
    };
    let init_step = Step {
        transitions: set![Transition { target: negotiations_state }],
    };
    let negotiations_step = Step {
        transitions: set![
            Transition { target: failed_state },
            Transition { target: negotiations_state },  // Self-loop (terms agreed)
            Transition { target: switched_state },
        ],
    };
    let failed_step = Step {
        transitions: Set::empty(),  // Terminal state
    };
    let switched_step = Step {
        transitions: Set::empty(),  // Terminal state
    };

    let steps_map = Map::empty()
        .insert(start_state, start_step)
        .insert(init_state, init_step)
        .insert(negotiations_state, negotiations_step)
        .insert(failed_state, failed_step)
        .insert(switched_state, switched_step);

    Workflow {
        initial_state: start_state,
        final_states: set![failed_state, switched_state],
        steps: steps_map,
    }
}

// ============================================
// PROOF FUNCTIONS
// ============================================

// Verify that the example workflow is valid
proof fn proof_workflow_valid()
    ensures
        example_purchase_workflow().valid(),
{
    let wf = example_purchase_workflow();
    assert(wf.steps.contains_key(wf.initial_state));
    assert(wf.final_states.contains(state_to_int(State::PurchaseFailed)));
    assert(wf.final_states.contains(state_to_int(State::OwnershipsSwitched)));
    assert(wf.is_terminal(state_to_int(State::PurchaseFailed)));
    assert(wf.is_terminal(state_to_int(State::OwnershipsSwitched)));
}

// Verify terminal states
proof fn proof_terminal_states()
    ensures
        true,
{
    let wf = example_purchase_workflow();
    let failed_state = state_to_int(State::PurchaseFailed);
    let switched_state = state_to_int(State::OwnershipsSwitched);
    let start_state = state_to_int(State::StartState);
    let init_state = state_to_int(State::InitPurchasePending);
    let negotiations_state = state_to_int(State::Negotiations);

    assert(wf.is_terminal(failed_state));
    assert(wf.next_states(failed_state).len() == 0);
    assert(wf.is_terminal(switched_state));
    assert(wf.next_states(switched_state).len() == 0);
    assert(!wf.is_terminal(start_state));
    assert(!wf.is_terminal(init_state));
    assert(!wf.is_terminal(negotiations_state));
}

// Verify valid transitions
proof fn proof_valid_transitions()
    ensures
        true,
{
    let wf = example_purchase_workflow();
    let start_state = state_to_int(State::StartState);
    let init_state = state_to_int(State::InitPurchasePending);
    let negotiations_state = state_to_int(State::Negotiations);
    let failed_state = state_to_int(State::PurchaseFailed);
    let switched_state = state_to_int(State::OwnershipsSwitched);

    assert(wf.can_transition(start_state, init_state));
    assert(wf.can_transition(init_state, negotiations_state));
    assert(wf.can_transition(negotiations_state, failed_state));
    assert(wf.can_transition(negotiations_state, negotiations_state));  // Self-loop
    assert(wf.can_transition(negotiations_state, switched_state));
    assert(!wf.can_transition(start_state, negotiations_state));
    assert(!wf.can_transition(failed_state, switched_state));
}

// Verify next_states returns correct sets
proof fn proof_next_states()
    ensures
        true,
{
    let wf = example_purchase_workflow();
    let start_state = state_to_int(State::StartState);
    let init_state = state_to_int(State::InitPurchasePending);
    let negotiations_state = state_to_int(State::Negotiations);
    let failed_state = state_to_int(State::PurchaseFailed);
    let switched_state = state_to_int(State::OwnershipsSwitched);

    assert(wf.next_states(start_state).contains(init_state));
    assert(wf.next_states(start_state).len() == 1);
    assert(wf.next_states(init_state).contains(negotiations_state));
    assert(wf.next_states(init_state).len() == 1);
    assert(wf.next_states(negotiations_state).contains(failed_state));
    assert(wf.next_states(negotiations_state).contains(negotiations_state));
    assert(wf.next_states(negotiations_state).contains(switched_state));
    assert(wf.next_states(negotiations_state).len() == 3);
    assert(wf.next_states(failed_state).len() == 0);
    assert(wf.next_states(switched_state).len() == 0);
}

// Verify initial state is correctly set
proof fn proof_initial_state()
    ensures
        true,
{
    let wf = example_purchase_workflow();
    assert(wf.initial_state == state_to_int(State::StartState));
    assert(wf.steps.contains_key(wf.initial_state));
}

// Verify final states are correctly set
proof fn proof_final_states()
    ensures
        true,
{
    let wf = example_purchase_workflow();
    let failed_state = state_to_int(State::PurchaseFailed);
    let switched_state = state_to_int(State::OwnershipsSwitched);
    assert(wf.final_states.contains(failed_state));
    assert(wf.final_states.contains(switched_state));
    assert(wf.final_states.len() == 2);
}

}
