# Usage

> cargo run <day number>

## Notes

- for formatting and linting you have:

```
cargo fmt
cargo clippy -- -Dwarnings
```

- it's usually less common to have both `lib` and `bin` in the same project, although sometimes needed. `cargo workspace` exists if you want to separate into multiple bins. In this case your common code can exist within a `shared` module inside your bin

- (aoc specific) `day01` instead of `day1` will make the files order properly when you get to later days

- probably want lower case on folder names too, snake_case for file names like `file_reader`

- error handling: typically best to avoid unwrap wherever possible. the crate `anyhow` exists to make error propagation easier for simple binaries, and typically people would descend into `thiserror` crate to do things more robustly/verbosely

- module structure can be fiddly at first. Here I've moved to a simple binary, which requires a `main` inside a `src` folder. From there, you define other "modules" in files and folders. If you have a file, you declare it using `mod` (like I have in main for the shared file). If using a folder you have a `mod.rs` file declaring the other file modules, almost like a re-export in a way (like I have with `solutions`). Then you use the `pub` keyword to define if it should be public to other modules.

- (aoc specific) I went for a hashmap approach to first count the col2, I think saves on complexity

- traits are an interesting one with subtle differences to interfaces in other languages. rust works on 'data driven' approach where structs are data rather than traditional "objects", and traits define behaviour that data is capable of. I think the way you did it makes sense where a day has some underlying data structure, I also added a trait that defines a "Solution" i.e. something that can be called and return an answer to a part 1 and part 2 

- main acts as the driver and calls out to "something that implements Solution". This way the answer printing logic is centralised, and requires the result of `run()` to return a tuple of values that implement the IntoString type for printing

- I think the Entry API for hashmaps is really neat and included an example in the code

- the `#[derive(Default)]` macro on the Day1 struct provides a nice easy way to instantiate the struct via ::default()
