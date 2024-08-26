## To Do
- [x] How to run using the name e.g. `mentee` rather than `cargo run`? currently building to `target/debug` then running it like `./mentee_cli`
- [x] Create dummy data const and compile
- [x] Render a table
- [x] how to set the word that calls the CLI in the terminal? `cargo install`?
- [ ] `default_value_t` why `t`?
- [ ] Check all videos for tips
- [ ] just running `mentees` should show help subcommand, same for `mentees create`
- [ ] Video on subcommand [Code to the moon](https://youtu.be/fD9ptABVQbI?si=YrIbd2tBDegxCSLd)
- [ ] implement architecture from `minigrep` project
- [ ] how to design CLI?

## Design
- [ ] `mentee` should show all available options
- [ ] `mentees create` (multi step process for each field)
- [ ] `mentees delete`
- [ ] `mentees update`
- [ ] `mentees get`
- [ ] `name`, `calls/month`, `gross`, `net`, `status` (warm, cold, hot), `payment_date`
- [ ] help section for each commmand and subcommand
- [ ] https://clig.dev/
- [ ] every action has a confirmation e.g. Alex was created / Alex was deleted
- [ ] flags over args
- [ ] `mentee_cli init` works out if there is a db or not
- [ ] if init hasn't been run, then runs init first
- https://github.com/mikaelmello/inquire
- https://github.com/shuttle-hq/shuttle/tree/main/cargo-shuttle (example project)
- https://github.com/orhun/git-cliff (example project)

## Questions
- should main handle db connection or lib?
- should run be passed the connection or the path to create its own connection?

>>> The responsibilities that remain in the main function after this process should be limited to the following:

Calling the command line parsing logic with the argument values
Setting up any other configuration
Calling a run function in lib.rs
Handling the error if run returns an error
This pattern is about separating concerns: main.rs handles running the program, and lib.rs handles all the logic of the task at hand. Because you can’t test the main function directly, this structure lets you test all of your program’s logic by moving it into functions in lib.rs. The code that remains in main.rs will be small enough to verify its correctness by reading it. Let’s rework our program by following this process.

- [ ] deal with notes in obisidan
- [ ] error handling - same as minigrep project print to stderr not stdout
- [ ] does this work if you run the cli from different working directories? when installing in prod, is this gna create multiple dbs?
- [ ] post on reddit learnrust subreddit for code review and find a discord for learning rust 
- [ ] run strict formatter, clippy?
