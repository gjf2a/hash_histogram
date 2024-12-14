# 0.9.2
* Added `.counts()`

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

