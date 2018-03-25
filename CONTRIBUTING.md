### Contributing

#### Getting started (section in README)

Contributions are welcome!

The easiest way for you to contribute right now is to use `Test Data Generation`, and provide constructive feedback, (e.g.: bugs, enhancement requests, product roadmap direction). 

If you've just filed an issue, or you want to work on an [existing one](https://github.com/dsietz/test-data-generation/issues), tag me with @dsietz on an issue, and get you setup as a contributor.

As a note, all contributions are expected to follow [the Rust Code of Conduct](https://www.rust-lang.org/en-US/conduct.html).

#### Project Structure

This project attempts to be an idiomatic rust library and to maintain a sane structure. All source code is located in `src/`, and tests are in `tests/`.

The source is split into four modules:
- `lib.rs` contains top-level traits, module documentation, and helper functions
- `builders.rs` contains all the configuration code
- `errors.rs` contains error handling for finishing configuration
- and `log_impl.rs` contains the implementation for `log::Log` which is created to run for the actual logging.

Hopefully these modules are fairly separated, and it's clear when you'll need to work on multiple sections. Adding a new log implementation, for instance, will need to touch `builders.rs` for configuration, and `log_impl.rs` for the implementation - both pieces of code will connect via `builders::Dispatch::into_dispatch`, but besides that, things should be fairly separate.

#### Pull requests

Pull requests are _the_ way to change code using git. If you aren't familiar with them in general, GitHub has some [excellent documentation](https://help.github.com/articles/about-pull-requests/).

There aren't many hard guidelines in this repository on how specifically to format your request. Main points:

- Please include a descriptive title for your pull request, and elaborate on what's changed in the description.
- Feel free to open a PR before the feature is completely ready, and commit directly to the PR branch.
  - This is also great for review of PRs before merging
  - All commits will be squashed together on merge, so don't worry about force pushing yourself.
- Please include at least a short description in each commit, and more of one in the "main" feature commit. Doesn't
  have to be much, but someone reading the history should easily tell what's different now from before.
- If you have `rustfmt-nightly` installed, using it is recommended. I can also format the code after merging the code,
  but formatting it consistently will make reviewing nicer.

### Testing

Building fern is as easy as is expected, `cargo build`.

Testing is somewhat more convoluted - mostly because using fern requires initializing a global logger. To test the two "integration" tests separately, you'll need to invoke two commands:

```sh
cargo test -- --skip test2
cargo test test2
```

To run the example program, use:

```sh
cargo run --example cmd-program # test less logging
cargo run --example cmd-program -- --verbose # test more logging
```

Feel free to add tests and examples demonstrating new features as you see fit. Pull requests which solely add new/interesting example programs are also welcome.

### Mentoring

With all that said, contributing to a library, especially if new to rust, can be daunting.

Feel free to email me at daboross @ daboross.net with any questions!