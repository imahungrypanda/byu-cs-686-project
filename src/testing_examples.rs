
// Example 1: A simple spec function
spec fn add_one(x: int) -> int {
    x + 1
}

// Example 2: A lemma that states a property about add_one
proof fn add_one_lemma(x: int)
    ensures
        add_one(x) == x + 1,
{
    // The proof is trivial - just the definition
}

// Example 3: A more interesting lemma about addition
proof fn addition_commutative(a: int, b: int)
    ensures
        a + b == b + a,
{
    // This is a fundamental property of addition
}

// Example 4: A spec function with preconditions
spec fn divide(x: int, y: int) -> int
    recommends
        y != 0,
{
    x / y
}

// Example 5: A lemma about division
proof fn division_lemma(x: int, y: int)
    requires
        y != 0,
    ensures
        divide(x, y) == x / y,
{
    // Proof follows from the definition
}

// ============================================
// PROOF FUNCTIONS (like unit tests)
// ============================================

// Test 1: Verify add_one_lemma works
proof fn test_add_one_lemma()
    ensures
        true,
{
    // Call the lemma with specific values
    add_one_lemma(5);
    // Now we can assert the property holds for 5
    assert(add_one(5) == 6);

    // Test with another value
    add_one_lemma(0);
    assert(add_one(0) == 1);

    // Test with negative
    add_one_lemma(-3);
    assert(add_one(-3) == -2);
}

// Test 2: Verify addition_commutative works
proof fn test_addition_commutative()
    ensures
        true,
{
    // Test with specific values
    addition_commutative(3, 5);
    assert(3 + 5 == 5 + 3);

    addition_commutative(10, 20);
    assert(10 + 20 == 20 + 10);
}

// Test 3: Verify division_lemma works
proof fn test_division_lemma()
    ensures
        true,
{
    // Test with valid division
    division_lemma(10, 2);
    assert(divide(10, 2) == 5);

    division_lemma(15, 3);
    assert(divide(15, 3) == 5);
}

// Test 4: A more complex example - testing multiple lemmas together
proof fn test_combined_properties()
    ensures
        true,
{
    let x = 7;
    let y = 3;

    // Use commutativity
    addition_commutative(x, y);

    // Use add_one
    add_one_lemma(x);
    assert(add_one(x) == 8);

    // Combine properties
    assert(add_one(x) + y == y + add_one(x));
}

// Test 5: Testing with different scenarios
proof fn test_edge_cases()
    ensures
        true,
{
    // Test zero
    add_one_lemma(0);
    assert(add_one(0) == 1);

    // Test negative
    add_one_lemma(-1);
    assert(add_one(-1) == 0);

    // Test large numbers (conceptually)
    add_one_lemma(100);
    assert(add_one(100) == 101);
}
