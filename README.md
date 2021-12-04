# Overview

`HashHistogram` creates histograms with keys of any hashable data type. Features include:

* Check histogram count for a key.
* Check total histogram counts across all keys.
* Provide all keys in descending ranked order.
* Find the mode of the histogram (i.e., an item with the largest number of counts)
* Find the mode of any `IntoIterator` type, bulding a `HashHistogram` as an intermediate step.

# Updates
* **0.5**: Initial public release.