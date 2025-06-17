/*// ❌ This will not compile!
// r points to x, but x goes out of scope before r is used.

fn example_0() {
    let r: &i32; // Declare a reference 'r' to an i32 (uninitialized for now)

    {
        let x: i32 = 5; // 'x' is created inside this inner block
        r = &x;         // 'r' now points to 'x'
    } // ⚠️ 'x' is dropped here — its memory is freed!

    // ❌ Error: 'r' is now a dangling reference (points to freed memory)
    println!("r: {}", r);
}*/


// ✅ This works!
// 'x' and 'r' live in the same scope, so no dangling reference.

fn example_0_1() {
    let r: &i32;         // Declare a reference 'r'
    let x: i32 = 5;      // 'x' is declared in the same (outer) scope
    r = &x;              // 'r' now safely points to 'x'

    println!("r: {}", r); // ✅ OK — 'x' is still alive here
}

/// This example demonstrates using references in a function without requiring lifetime annotations.
///
/// In Rust, lifetime annotations are only needed when a function returns a reference.
/// Since this function returns an owned value (`i32`), the Rust compiler can infer everything safely.
///
/// `i32` implements the `Copy` trait, so dereferencing an `&i32` to get the value produces a copy.
/// This avoids ownership issues or the need to track lifetimes explicitly.
fn example_1() {
    // Declare a variable that will hold the result.
    let highest_age: i32;

    // Initialize two integers.
    let alice_age: i32 = 20;
    let bob_age: i32 = 21;

    // Call the `largest` function, passing references to the integer values.
    // Since we're passing `&alice_age` and `&bob_age`, the function receives `&i32` references.
    highest_age = largest(&alice_age, &bob_age);

    // Print the result — this is safe because the returned value is owned, not a reference.
    println!("Highest age is: {}", highest_age);

    /// This function accepts two references to `i32` and returns the larger value.
    ///
    /// Even though the parameters are references (`&i32`), the return type is just `i32`.
    /// This means the function returns a **copy** of the value, not a reference,
    /// so we don't need to worry about how long the original variables live.
    fn largest(compare_1: &i32, compare_2: &i32) -> i32 {
        // Compare the two referenced values. `compare_1 > compare_2` compares the actual values.
        if compare_1 > compare_2 {
            // Dereference compare_1 and return the value (copies the i32).
            *compare_1
        } else {
            // Dereference compare_2 and return the value (also copies).
            *compare_2
        }
    }
}

/// This example shows how to return a reference from a function safely using **explicit lifetimes**.
///
/// When a function returns a reference, Rust needs to know how the lifetimes of the inputs
/// relate to the output. Without this information, the compiler can't ensure the returned reference is valid.
///
/// The `'a` lifetime in this example tells Rust:
/// "The returned reference will live as long as the shorter of the two input references."
fn example_2() {
    // Declare a reference to an i32.
    // We haven't initialized it yet — it'll be assigned by the function call.
    let highest_age: &i32;

    // Create two i32 values.
    let alice_age: i32 = 20;
    let bob_age: i32 = 21;

    // Call the function and store the reference to the larger value.
    // We're passing references, and the function returns a reference,
    // so the compiler needs to know how long those references live.
    highest_age = largest(&alice_age, &bob_age);

    // Print the value pointed to by the reference.
    println!("Highest age is: {}", highest_age);

    /// This function compares two `&i32` references and returns a reference to the larger one.
    ///
    /// The `'a` lifetime annotation tells the compiler:
    /// "Both input references and the return reference must be valid for at least the same lifetime `'a`."
    /// This guarantees that the returned reference won't outlive either input.
    fn largest<'a>(compare_1: &'a i32, compare_2: &'a i32) -> &'a i32 {
        // Compare the values the references point to
        if compare_1 > compare_2 {
            // Return a reference to the larger value
            compare_1
        } else {
            compare_2
        }
    }
}

/*/// This example demonstrates a **lifetime mismatch** error — a common and important concept in Rust.
///
/// Even though the function `largest` is valid in isolation (it returns a reference with lifetime `'a`),
/// the way it's used here creates a situation where the returned reference could point to data that
/// goes out of scope.
///
/// Specifically:
/// - `alice_age` lives for the entire function (`'a`)
/// - `bob_age` lives only inside the inner block (`'b`)
///
/// When you call `largest(&alice_age, &bob_age)`, Rust tries to find a lifetime `'a`
/// that works for both inputs — in this case, it would need to be the **shorter** of the two,
/// which is `'b`. But then you assign the result to a variable that **outlives `'b`**.
/// This creates the possibility of a **dangling reference**, and Rust won't allow it.
fn example_2_2() {
    let highest_age: &i32; // Declare a reference that will live across the whole function

    let alice_age: i32 = 20; // 'a: valid until the end of the function

    {
        let bob_age: i32 = 21; // 'b: valid only inside this inner block

        // Problem: `largest` returns a reference that *could* point to `bob_age`,
        // but `bob_age` is about to go out of scope.
        highest_age = largest(&alice_age, &bob_age);
        // At this point, highest_age might refer to `bob_age`,
        // but we can't guarantee that's safe beyond this block.
    } // 'b ends here — `bob_age` is dropped

    // ❌ Error: Using `highest_age` here could lead to a dangling reference
    // if it points to the now-dropped `bob_age`.
    println!("Highest age is: {}", highest_age);

    /// Function to return the reference to the larger value.
    ///
    /// The lifetime `'a` tells Rust: "both inputs and the return value live at least `'a`."
    /// In this case, Rust will infer `'a` to be the **shorter** of the two lifetimes passed in,
    /// so it ends up being `'b`.
    fn largest<'a>(compare_1: &'a i32, compare_2: &'a i32) -> &'a i32 {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}*/


/// This example shows how to avoid a lifetime error by **copying the value** from the reference
/// before the reference goes out of scope.
///
/// In the inner block, we call a function that returns a reference to one of two inputs.
/// One of those inputs (`bob_age`) is about to go out of scope. However, we dereference the result
/// (`*highest_age`) while everything is still valid, storing the copied value in `new_value`.
///
/// This avoids the problem of holding onto a potentially dangling reference.
///
/// Since `i32` implements the `Copy` trait, dereferencing gives us a full, safe copy of the value.
fn example_2_3() {
    // Declare a reference and a value we'll use after the inner block.
    let highest_age: &i32; // Reference to one of the two ages
    let new_value: i32;    // Copy of the value, safe to use later

    // Define the first input, which will live the entire function.
    let alice_age: i32 = 20; // Lifetime `'a`

    {
        // Define the second input, with a shorter lifetime `'b`
        let bob_age: i32 = 21;

        // Call the function. It returns a reference to either alice_age or bob_age.
        highest_age = largest(&alice_age, &bob_age);

        // Immediately copy the value from the reference.
        // This is safe because both alice_age and bob_age are still valid here.
        new_value = *highest_age;
    } // bob_age is dropped here — so highest_age would now be unsafe to use

    // ✅ This is safe because we’re using the copied value, not the reference itself.
    println!("Highest age is: {}", new_value);

    /// Returns a reference to the larger of two `i32` values.
    ///
    /// The `'a` lifetime ensures that the returned reference is valid
    /// as long as both inputs are.
    fn largest<'a>(compare_1: &'a i32, compare_2: &'a i32) -> &'a i32 {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}

/// This example demonstrates a **generic function** that compares two references and returns
/// a reference to the larger one. It uses:
///
/// - Generics (`T`) so it can work with any type
/// - Trait bounds (`T: PartialOrd`) so the values can be compared
/// - Explicit lifetimes (`'a`) so the compiler knows how long the returned reference is valid
///
/// The caller safely dereferences the result **before** any inputs go out of scope,
/// ensuring there's no lifetime violation.
fn example_3_generics() {
    // Declare a reference to a value and a place to store the copied value.
    let highest_age: &i32;  // Will point to either alice_age or bob_age
    let new_value: i32;     // Will store a safe, copied value

    // First value lives throughout the function
    let alice_age: i32 = 20;

    {
        // Second value lives only within this inner scope
        let bob_age: i32 = 21;

        // Call the generic function, explicitly specifying the type as i32
        // This is optional — Rust can infer `T = i32` here, so `::<i32>` can be omitted.
        highest_age = largest::<i32>(&alice_age, &bob_age);

        // Immediately dereference the returned reference while both inputs are still valid
        new_value = *highest_age;
    } // bob_age is dropped here — highest_age would be invalid beyond this point

    // Use the safe, copied value
    println!("Highest age is: {}", new_value);

    /// Generic function that returns a reference to the larger of two inputs
    ///
    /// `'a` is the lifetime of both input references and the returned reference.
    /// `T` is any type that can be compared using `>`, as guaranteed by `T: PartialOrd`.
    fn largest<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}

/// This example demonstrates how to use **structs with lifetimes** and safely compare
/// borrowed data from two `Person` instances.
///
/// The key concepts are:
/// - Structs holding references need lifetime annotations
/// - Generic functions using lifetimes and trait bounds
/// - Safely copying values before the data they're borrowed from goes out of scope
struct Person<'p> {
    name: &'p str,    // Borrowed string slice
    points: &'p f32,  // Borrowed float reference
}

fn example_4_structs() {
    // Reference to the highest points value (lifetime-bound)
    let highest_age: &f32;

    // Variable to store a safe, copied version of the highest points
    let new_value: f32;

    // Alice's data is valid throughout the function, so her references live long enough
    let alice: Person = Person {
        name: "Alice",
        points: &50.2,
    };

    {
        // Bob’s data only lives in this block
        let bob: Person = Person {
            name: "Bob",
            points: &40.5,
        };

        // Call the generic largest function to compare Alice’s and Bob’s points
        // It returns a reference to the larger value
        highest_age = largest::<f32>(alice.points, bob.points);

        // Copy the value pointed to by the reference while still in the valid scope
        new_value = *highest_age;
    } // bob is dropped here, so `bob.points` is no longer valid

    // Use the copied value — this is safe
    println!("Highest age is: {}", new_value);

    /// Generic function that compares two references and returns the larger one
    ///
    /// `'a` ensures the returned reference does not outlive either input
    fn largest<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}
