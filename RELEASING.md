# Release Checklist

These steps assume that you've checked out the Subxt repository and are in the root directory of it.

We also assume that ongoing work done is being merged directly to the `master` branch.

1.  Ensure that everything you'd like to see released is on the `master` branch.

2.  Create a release branch off `master`, for example `release-v0.17.0`. Decide how far the version needs to be bumped based
    on the changes to date. If unsure what to bump the version to (e.g. is it a major, minor or patch release), check with the
    Parity Tools team.

3.  Check that you're happy with the current documentation.

    ```
    cargo doc --open --all-features
    ```

    CI checks for broken internal links at the moment. Optionally you can also confirm that any external links
    are still valid like so:

    ```
    cargo install cargo-deadlinks
    cargo deadlinks --check-http -- --all-features
    ```

    If there are minor issues with the documentation, they can be fixed in the release branch.

4.  Bump the crate version in `Cargo.toml` to whatever was decided in step 2 for `subxt-codegen`, `subxt-macro`, `subxt` and `subxt-cli`.

5.  Update `CHANGELOG.md` to reflect the difference between this release and the last. If you're unsure of
    what to add, check with the Tools team. See the `CHANGELOG.md` file for details of the format it follows.

    Any [closed PRs](https://github.com/paritytech/subxt/pulls?q=is%3Apr+is%3Aclosed) between the last release and
    this release branch should be noted.

6.  Commit any of the above changes to the release branch and open a PR in GitHub with a base of `master`.

7.  Once the branch has been reviewed and passes CI, merge it.

8.  Now, we're ready to publish the release to crates.io.

    Checkout `master`, ensuring we're looking at that latest merge (`git pull`).

    The crates in this repository need publishing in a specific order, since they depend on each other.
    Additionally, `subxt-macro` has a circular dev dependency on `subxt`, so we use `cargo hack` to remove
    dev dependencies (and `--allow-dirty` to ignore the git changes as a result) to publish it.

    So, first install `cargo hack` with `cargo install cargo hack`. Next, you can run something like the following
    command to publish each crate in the required order (allowing a little time inbetween each to let `crates.io` catch up)
    with what we've published).

    ```
    (cd codegen && cargo publish) && \
        sleep 10 && \
        (cd macro && cargo hack publish --no-dev-deps --allow-dirty) && \
        sleep 10 && \
        cargo publish && \
        sleep 10 && \
        (cd cli && cargo publish);
    ```

    If you run into any issues regarding crates not being able to find suitable versions of other `subxt-*` crates,
    you may just need to wait a little longer and then run the remaining portion of that command.

9.  If the release was successful, tag the commit that we released in the `master` branch with the
    version that we just released, for example:

    ```
    git tag -s v0.17.0 # use the version number you've just published to crates.io, not this one
    git push --tags
    ```

    Once this is pushed, go along to [the releases page on GitHub](https://github.com/paritytech/subxt/releases)
    and draft a new release which points to the tag you just pushed to `master` above. Copy the changelog comments
    for the current release into the release description.