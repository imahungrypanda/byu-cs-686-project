<!-- 16392484-c9c1-41a4-870d-86658f4aebcf f9d91d11-b5a7-4d34-80da-c5767b713513 -->
# Represent Purchase Workflow in Verus

## Current State

The code has a basic workflow structure with `State` enum, `Step` struct, and `Workflow` struct, but needs to represent the specific purchase workflow with labeled transitions and conditional guards.

## Workflow Structure Analysis

**States:**

- StartState (initial)
- Init Purchase Pending
- Negotiations (appears twice in diagram - will need to handle this)
- Purchase Failed (terminal)
- Ownerships Switched (terminal)

**Transitions with Conditions:**

1. StartState → Init Purchase Pending: `paymentOwner == buyerName && backpackOwner == sellerName`
2. Init Purchase Pending → Negotiations: `terms != pending || paymentOffered != pending`
3. Negotiations → Purchase Failed: `terms == noRetry || paymentOffered == noRetry`
4. Negotiations → Negotiations: `terms == agreed && paymentOffered == paymentAmount`
5. Negotiations → Ownerships Switched: `paymentOwner == sellerName && backpackOwner == buyerName`

## Implementation Plan

1. **Update State enum** (`src/main.rs` lines 5-9)

- Replace placeholder states with actual workflow states
- Add: StartState, InitPurchasePending, Negotiations, PurchaseFailed, OwnershipsSwitched
- Note: Handle "Negotiations" appearing twice (may need distinct states or represent as different positions)

2. **Enhance Step to support transition conditions** (`src/main.rs` lines 11-13)

- Current: `next_states: Set<State>`
- New: Support transitions with conditions/guards
- Options:
a) Add `transitions: Set<(State, spec fn(State) -> bool)>` to Step
b) Create `Transition` struct with target and condition predicate
c) Use `Map<State, spec fn(State) -> bool>` mapping target state to condition

3. **Update Workflow methods** (`src/main.rs` lines 23-47)

- Update `next_states()` to filter by satisfied conditions (needs context/parameters)
- Add `can_transition_with_context()` for conditional transitions
- Update `is_terminal()` to check against `final_states` set
- Update `valid()` to ensure initial_state exists, final_states are valid, and transitions are well-formed

4. **Add example workflow construction** (`src/main.rs` after line 47)

- Create `example_purchase_workflow()` spec function
- Define all states and transitions with their conditions
- Add proof functions to verify workflow properties

5. **Consider transition conditions representation**

- Conditions reference variables: `paymentOwner`, `buyerName`, `sellerName`, `backpackOwner`, `terms`, `paymentOffered`, `paymentAmount`
- These are runtime values, so conditions need to be predicates over a context/state
- May need a `WorkflowContext` type to hold these values
- Or use closure-style predicates that take context parameters

### To-dos

- [ ] Update State enum to match purchase workflow states (StartState, InitPurchasePending, Negotiations, PurchaseFailed, OwnershipsSwitched)
- [ ] Enhance Step struct or create Transition type to support conditional transitions with guards/predicates
- [ ] Update Workflow impl methods to handle conditional transitions and validate against final_states
- [ ] Create example_purchase_workflow() spec function that constructs the full purchase workflow with all states and transitions
- [ ] Add proof functions to verify workflow properties (reachability, terminal states, valid transitions)