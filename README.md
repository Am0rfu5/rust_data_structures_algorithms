# Data Structures and Algorithms (DSA) using Rust

This repository contains implementations of various data structures and algorithms in Rust. This is a way to learn Rust by example.

## Setup

- You need to have Rust installed. I recommend using [rustup](https://rustup.rs/).
- Clone this repository.

## Usage

- Run `cargo test` to run all tests.
- Run `cargo test <test-function-name> -- --nocapture` to run a specific test.

Each test can print to the console and by uncommenting the println!() statements you can see the output at various stages of the algorithm.  This is a great way to learn how the algorithm works.

A more interesting way to use this is to run a step debugger like [gdb](https://www.gnu.org/software/gdb/) or [lldb](https://lldb.llvm.org/) and step through the code.

To set this up in VS Code you can use the [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) extension. You can then set breakpoints and step through the code by clicking the Debug link above the test function.

## Big O Notation

It is important to have an understanding of Big O Notation before looking at the code.  This is a way to describe the performance of an algorithms by estimating the worst case scenario for runtime or memory usage scaled with respect to the size of the input data.

For example, if you have an algorithm that takes 1 second to run no matter what the input size is, then it is O(1). If it runs or memory grow as a multiple of the input of size 100, then it is O(n).  If it the runtime or memory increase in as the square of the input, then it is O(n^2).  The following table shows the most common Big O runtimes in order of fastest to slowest.

| Big O | Name | Example |
| --- | --- | --- |
| O(1) | Constant | Accessing an array element by index |
| O(log n) | Logarithmic | Binary search |
| O(n) | Linear | Finding the maximum value in an unsorted list |
| O(n log n) | Linearithmic | Merge sort |
| O(n^2) | Quadratic | Bubble sort |
| O(n^3) | Cubic | Matrix multiplication |
| O(2^n) | Exponential | Finding all subsets of a set |
| O(n!) | Factorial | Finding all permutations of a string |

An exmaple of O(1) is the [src/data_structures/array.rs](src/data_structures/array.rs) file.  It contains an array of integers and a function that returns the value at a given index.  The function takes the array and the index as input parameters and returns the value at that index.  It does not matter how big the array is, it will always take the same amount of time to return the value at a given index.  This is O(1) or constant time.

## Recursion

Understanding recursion is essential to exploring algorithms. It is also a great way to learn about the Rust compiler and how it optimizes code.  The [src/recursion/path_finding.rs](src/recursion/path_finding.rs) file contains an example of recursion for a maze game using a path-finding algorithm.

## Data Structures

See the DATASTRUCTURES.md file for more on the subject including a list of data structures and links to the source code.

## Algorithms

See the ALGORITHMS.md file for more on the subject including a list of algorithms and links to the source code.

## Use of Helper functions (traits and macros)

We utilize some helper functions to keep the code clean and readable. We recommend examining these more closely if you don't know what is happening under the hood. It can be assumed that these have negligible impact on the time or memory complexity of the example algorithm but you can not have a thorough understanding of what is happening close to the metal if you gloss over the specifics. Looking at the source code of these helper functions can also be a great way to learn about Rust and will reveal some interesting things about the language.

For example the use of array.swap() has a O(1) time complexity. In a simplistic form the code would take two index values and an array as input parameters, it would take the value for the first one and hold it in temporary storage, write the the value of the second one to the first slot, then write the value of the second to the first slot and return the modified array. 

For an array of integers this would look something like this:

```rust
fn swap(array: &mut [i32], a: usize, b: usize) -> &mut [i32] {
    let temp = array[a];
    array[a] = array[b];
    array[b] = temp;
    array
}
```

However the current version of swap in Rust contains some very specific code including using raw pointers to avoid issues with double borrowing and a panic!() statement which will cause the program to crash if the index is out of bounds.  This is a great example of how Rust handles these condition errors.  The panic!() statement is a macro that will print a message to the console and then crash the program.  This is a great way to handle errors in development but not in production.  In production we would want to use the Result type to handle errors.  This is a great way to learn about Rust and how it handles errors.

```rust
    /// Swaps two elements in the slice.
    ///
    /// If `a` equals to `b`, it's guaranteed that elements won't change value.
    ///
    /// # Arguments
    ///
    /// * a - The index of the first element
    /// * b - The index of the second element
    ///
    /// # Panics
    ///
    /// Panics if `a` or `b` are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut v = ["a", "b", "c", "d", "e"];
    /// v.swap(2, 4);
    /// assert!(v == ["a", "b", "e", "d", "c"]);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_unstable(feature = "const_swap", issue = "83163")]
    #[inline]
    #[track_caller]
    pub const fn swap(&mut self, a: usize, b: usize) {
        // FIXME: use swap_unchecked here (https://github.com/rust-lang/rust/pull/88540#issuecomment-944344343)
        // Can't take two mutable loans from one vector, so instead use raw pointers.
        let pa = ptr::addr_of_mut!(self[a]);
        let pb = ptr::addr_of_mut!(self[b]);
        // SAFETY: `pa` and `pb` have been created from safe mutable references and refer
        // to elements in the slice and therefore are guaranteed to be valid and aligned.
        // Note that accessing the elements behind `a` and `b` is checked and will
        // panic when out of bounds.
        unsafe {
            ptr::swap(pa, pb);
        }
    }
```

This function contains a macro called ptr::addr_of_mut!() which is a macro that returns a raw pointer to a mutable reference that again can be used to learn about Rust and how it handles memory.  The macro is defined in the [src/libcore/ptr.rs](src/libcore/ptr.rs) file and looks like this:

```rust
/// Create a `mut` raw pointer to a place, without creating an intermediate reference.
///
/// Creating a reference with `&`/`&mut` is only allowed if the pointer is properly aligned
/// and points to initialized data. For cases where those requirements do not hold,
/// raw pointers should be used instead. However, `&mut expr as *mut _` creates a reference
/// before casting it to a raw pointer, and that reference is subject to the same rules
/// as all other references. This macro can create a raw pointer *without* creating
/// a reference first.
///
/// Note, however, that the `expr` in `addr_of_mut!(expr)` is still subject to all
/// the usual rules. In particular, `addr_of_mut!(*ptr::null_mut())` is Undefined
/// Behavior because it dereferences a null pointer.
///
/// # Examples
///
/// **Creating a pointer to unaligned data:**
///
/// ```
/// use std::ptr;
///
/// #[repr(packed)]
/// struct Packed {
///     f1: u8,
///     f2: u16,
/// }
///
/// let mut packed = Packed { f1: 1, f2: 2 };
/// // `&mut packed.f2` would create an unaligned reference, and thus be Undefined Behavior!
/// let raw_f2 = ptr::addr_of_mut!(packed.f2);
/// unsafe { raw_f2.write_unaligned(42); }
/// assert_eq!({packed.f2}, 42); // `{...}` forces copying the field instead of creating a reference.
/// ```
///
/// **Creating a pointer to uninitialized data:**
///
/// ```rust
/// use std::{ptr, mem::MaybeUninit};
///
/// struct Demo {
///     field: bool,
/// }
///
/// let mut uninit = MaybeUninit::<Demo>::uninit();
/// // `&uninit.as_mut().field` would create a reference to an uninitialized `bool`,
/// // and thus be Undefined Behavior!
/// let f1_ptr = unsafe { ptr::addr_of_mut!((*uninit.as_mut_ptr()).field) };
/// unsafe { f1_ptr.write(true); }
/// let init = unsafe { uninit.assume_init() };
/// ```
#[stable(feature = "raw_ref_macros", since = "1.51.0")]
#[rustc_macro_transparency = "semitransparent"]
#[allow_internal_unstable(raw_ref_op)]
pub macro addr_of_mut($place:expr) {
    &raw mut $place
}

```

As you can see digging in deeply is beyond the scope of understanding the algorithm but it is a great way to learn about Rust and how it handles memory and errors.

## Contributing

If you would like to contribute to this repository please see the [CONTRIBUTING.md](CONTRIBUTING.md) file.

## Roadmap

Phase 1 - Basic Modules
- [ ] create mods for each data structure
- [ ] create basic sorting algorithms mods
- [ ] create basic searching algorithms mods
- [ ] create basic graph algorithms mods
- [ ] create basic tree algorithms mods

Phase 2 - Charting of Algorithm Performance (time and memory)
- [ ] Create of a Charting project
- [ ] Implement existing tools for measuring and recording performance
- [ ] Implement visualization of gathered data