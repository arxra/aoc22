# AOC 2022

## How to run

Solutions look for a "input/%d.txt" file, so for day 4 the input should be in "input/04.txt".
Same with sample inputs from the task except they are placed in "small/".

All output from each day is given from tests which output is usually captured by the tests, but this is easily bypassed.
Note that rust tests filter on regex given, so to run tests for day 4 without capturing output:

```bash
cargo test 04 -- --nocapture
```
