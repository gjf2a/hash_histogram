# 0.12.0
* Added support of the [rust_decimal](https://crates.io/crates/rust_decimal) crate.
* Added normalization of `HashHistogram` objects using `Decimal` and `f64` counts.

# 0.11.0
* Added AddAssign for HashHistogram objects to combine them easily.
* Added FromIterator variant that uses `bump_by()`.

# 0.10.1
* Removed the approximate assertion from the doc-tests.
  * Using the supplementary crate in a doc-test seemed distracting.
* Assertion moved to standard tests.

# 0.10.0
* Relaxed a number of type restrictions on keys and values.
* This relaxation of restrictions now enables `f32` and `f64` to qualify as a `CounterType`. 

# 0.9.3
* Updated to Rust 2024

# 0.9.2
* Added `counts()`

# 0.9.1
* Fixed issue in README.md

# 0.9.0
* Added a type variable to allow any unsigned Copy type as the counter type.
  * The counter type defaults to `usize`, which previously was the only option.
  * Some situations may require type annotations as a result.

# 0.8.0
* Added `ranking_with_counts()`.

# 0.7.0
* `HashHistogram` and `KeyType` now implement `Default`.

# 0.6.2
* Fixed serious bug in `bump_by()`.

# 0.6.1
* Added `bump_by()`.

# 0.6.0
* Refactored `mode()` and `mode_values()` so that they only return the mode, rather than both mode and count.

# 0.5.2
* Added `mode_values()`

# 0.5.1
* Fixed some documentation.

# 0.5 
* Initial public release.

