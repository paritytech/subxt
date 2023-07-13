# Release Checklist

These steps assume that you've checked out the Subxt repository and are in the root directory of it.

We also assume that ongoing work done is being merged directly to the `master` branch.

1.  Ensure that everything you'd like to see released is on the `master` branch.

2.  Create a release branch off `master`, for example `release-v0.17.0`. Decide how far the version needs to be bumped based
    on the changes to date. If unsure what to bump the version to (e.g. is it a major, minor or patch release), check with the
    Parity Tools team.

3.  Check that you're happy with the current documentation.

    ```
    cargo doc --open
    ```

    CI checks for broken internal links at the moment. Optionally you can also confirm that any external links
    are still valid like so:

    ```
    cargo install cargo-deadlinks
    cargo deadlinks --check-http
    ```

    If there are minor issues with the documentation, they can be fixed in the release branch.

4.  Bump the crate versions in the root `Cargo.toml` to whatever was decided in step 2 (basically a find and replace from old version to new version in this file should do the trick).

5.  Ensure the `Cargo.lock` file is up to date.

    ```
    cargo generate-lockfile
    ```

6.  Update `CHANGELOG.md` to reflect the difference between this release and the last. If you're unsure of
    what to add, check with the Tools team. See the `CHANGELOG.md` file for details of the format it follows.

    First, if there have been any significant changes, add a description of those changes to the top of the
    changelog entry for this release.

    Next, you can use the following script to generate the merged PRs between releases:

    ```
    ./scripts/generate_changelog.sh
    ```

    Ensure that the script picked the latest published release tag (e.g. if releasing `v0.17.0`, the script should
    provide `[+] Latest release tag: v0.16.0` ). Then group the PRs into "Fixed", "Added" and "Changed" sections, and make any
    other adjustments that you feel are necessary for clarity.

7.  Commit any of the above changes to the release branch and open a PR in GitHub with a base of `master`.

8.  Once the branch has been reviewed and passes CI, merge it.

9.  Now, we're ready to publish the release to crates.io.

    1.  Checkout `master`, ensuring we're looking at that latest merge (`git pull`).

        ```
        git checkout master && git pull
        ```

    2.  Perform a final sanity check that everything looks ok.

        ```
        cargo test --all-targets
        ```

    3.  Run the following command to publish each crate in the required order (allowing
        a little time in between each to let crates.io catch up with what we've published).

        ```
        (cd metadata && cargo publish) && \
            (cd codegen && cargo publish) && \
            (cd macro && cargo publish) && \
            (cd subxt && cargo publish) && \
            (cd signer && cargo publish) && \
            (cd cli && cargo publish);
        ```

10. If the release was successful, tag the commit that we released in the `master` branch with the
    version that we just released, for example:

    ```
    git tag -s v0.17.0 # use the version number you've just published to crates.io, not this one
    git push --tags
    ```

    Once this is pushed, go along to [the releases page on GitHub](https://github.com/paritytech/subxt/releases)
    and draft a new release which points to the tag you just pushed to `master` above. Copy the changelog comments
    for the current release into the release description.
