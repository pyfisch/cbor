# Contributing to Serde CBOR
Thanks for your interest!

There are many ways to help:

* write an issue about a problem you encountered
* submit a pull request
* add documentation and examples

## Pull Requests

- Code should be easy to understand and documented.
- For new features and fixed bugs please add a test to one of the files in `test/`.
- The tests are run on Travis CI to catch regressions early.
- Format your code with `cargo fmt` before committing.
- Currently Serde CBOR does not contain `unsafe` code and I would like to keep it this way.
- We squash all commits and use conventional commits. Start your PR titles with either
  `feat` for a new feature, `fix` for a bug fix, `test` for new or fixed tests, `docs`
  for documentation changes, and `chore` for general purpose changes.
- Always have an issue to work off with for a PR. Design discussions happen in issues, while
  code itself is reviewed in PR.

## Making a Release

* [ ] Make sure the crate compiles and all tests pass.
* [ ] (Optional) Test that the fuzzer works and fuzz the crate for some time.
* [ ] Write a list with all changes made since the last release
* [ ] Increment the version number in `Cargo.toml` and the `README.md`. Bugfixes increase the patch version while new features or an increased minimum Rust version require a new minor version.
* [ ] Check that the file `examples/readme.rs` and the example from the `README.md` match.
* [ ] Commit the changes.
* [ ] Add a git tag with the new version number:
    `git tag "v42.0.2"`
* [ ] Push the changes: `git push --tags`
* [ ] Run `cargo publish`
* [ ] Add a new release to GitHub with a list of changes.
