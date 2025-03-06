# Unreleased

# 0.6.0

* Minimum supported Rust version: 1.81.0
* chore: bump base64 dependency

# 0.5.3

* Minimum supported Rust version: 1.71.1
* Fix default-src behavior with eval

# 0.5.2

* Minimum supported Rust version: 1.70
* Update logic for policy violation to match spec
* Remove `prefetch-src`, since that's deprecated

# 0.5.1

* Minimum supported Rust version: 1.60.0
* chore: bump base64 dependency

# 0.5.0

* feat: allow getting the document's sandboxing flag set
* feat: implement base URI
* Minimum supported Rust version: 1.50.0
* chore: switch to edition 2018
* chore: bump dependencies

# 0.4.2

* Fix a bug in port parsing

# 0.4.1

* Use the `sha2` crate for hashing (more portable)

# 0.4.0

* Implement hashing functions
* Minimum supported Rust version: 1.39.0

# 0.3.0

* Add `Destination::is_script_like`

# 0.2.0

* Remove quickcheck (apparently, it was forcing everybody to download it, even if they didn't need it)
* Bump minimum supported Rust version
* Add serde implementation for `Destination`

# 0.1.0

* Implement request checks, inline checks, and inline checks for everything in the specification

# 0.0.1

* Initial release
