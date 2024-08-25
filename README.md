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

