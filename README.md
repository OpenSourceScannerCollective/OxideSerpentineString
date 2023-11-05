
# Introduction

OxideSerpentineString is a library with a goal to reduce False Positive Rate (FPR) of detecting RegEx patterns in source
 code (Python, JavaScript, etc.) and data (JSON, TOML, CSV, etc.)


## Features

* **[_High performance_](https://github.com/mariomka/regex-benchmark)** Rust Library built with Python 3.x bindings 
allowing for the Python scripts to use **import** to access functionality.
* **Extract string literals** from supported languages (Python, JavaScript) and data structures (JSON, TOML, CSV)
* **Secret Detection** via Regular Expression matching against known secrets (Google API Key, etc.)
* **Metadata** for string literal extracts, and regex matches:
  * Line of Code (start, end)
  * Character position (start, end)
  * Position information in match, and source
* **Rust RegEx engine** provided to Python via function calls, which is _**upto 440%**_ times faster than the native 
Python 3.x regular expression library.
* **Source-code language detection** via a fork of [Hyperpolyglot](https://github.com/monkslc/hyperpolyglot) which is an 
implementation of the GitHub [Linguist](https://github.com/github-linguist/linguist) project, which is used by GitHub 
as the engine of its own language detection feature. A custom fork was necessary in order to analyse strings without 
providing direct access to the file on a local filesystem.

## Recommended Improvements

#### Features
* Add parsers for more languages (Java, C, C++, etc)
  * Make parsers more generic to the _style_ to avoid creating parsers for every language
* Procedural Macro (or other methods) to reduce redundant code
* Support for loading regular expressions from a configuration file
* Additional parser grammar to extract variable or argument names
  * This can be used with additional logic to check if the string is being assigned to a 'password' variable

#### Performance
* Migrate to using Hyperscan Regular Expression engine
  * The Rust [Regex crate](https://crates.io/crates/regex) is very high performance (44x Python), but there may be
    (an) even more performant option(s) available, but they are not as portable. 
  * [Hyperscan](https://github.com/intel/hyperscan) is the fastest Regular Expression engine available 
  ([rust crate](https://docs.rs/hyperscan/latest/hyperscan/)). It is made by Intel, so they have no interest in 
  supporting non-intel platforms such as ARM. This means that Hyperscan is not available for Modern Macs, embedded 
  hardware or certain low-cost kinds of cloud architecture. To fix this gap, 
  [vectorscan](https://github.com/VectorCamp/vectorscan) is a community fork of Hyperscan to support multiple 
  architectures, explicitly AArch64 (Arm 64 bit). The hyperscan rust crate had a 
  [pull request](https://github.com/flier/rust-hyperscan/pull/28) to provide this capability, but it was not 
  accepted/merged. [Nosey Parker](https://github.com/praetorian-inc/noseyparker) is a rust based secret scanning tool 
  which incorporates vectorscan into the project starting from 
  [Nosey Parker v0.13.0](https://github.com/praetorian-inc/noseyparker/releases/tag/v0.13.0).
  * This is a relatively trivial task if ARM is not a required deployment target

## Examples

### String literal extraction
_The following is a regular expression match produced against a string extract._

![alt text](docs/screenshot_1.png)

### Regular Expression pattern match
_The following is a regular expression match produced by matching a pattern against an **extracted string**._

![alt text](docs/screenshot_2.png)