# advent of code 2023
educational endeavour for me to:
- generally practise writing Rust code more, and
- more specifically skill up in writing text parsers

some of the days' answers are inspired by https://github.com/ChristopherBiscardi/advent-of-code,
from whom I'm grateful to have learned some neat iterator funcs like `find_map()`
and `rfind()`, as well as how to actually use [`nom`](https://github.com/rust-bakery/nom).


## how to run
```sh
$ git clone https://github.com/ndinata/aoc2023.git
$ cd aoc2023

# example: running tests for day 2's solutions (for both part 1 and 2)
$ make test day=02

# example: printing the solution for day 4 part 2
$ make run day=04 part=2
```


## license
licensed under [MIT](./LICENSE).
