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

7.  If any of the differences impact the minimum version of `rustc` that the code will run on, please update the `rust-version`
    field in the root `Cargo.toml` accordingly.

8.  Commit any of the above changes to the release branch and open a PR in GitHub with a base of `master`.

9.  Once the branch has been reviewed and passes CI, merge it.

10. Publish the release. Crates are published by
    [paritytech/crates_publish_automation](https://github.com/paritytech/crates_publish_automation) as
    `parity-crate-owner`; the `Release` workflow in this repository only packages them and hands them over.

    1.  On the latest `master`, check that the crates package:

        ```
        git checkout master && git pull
        cargo package --locked --exclude-lockfile --no-verify \
          $(cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.publish == null) | "-p \(.name)"')
        ```

    2.  Tag the commit and push the tag:

        ```
        git tag -s v0.17.0 # use the version you are releasing, not this one
        git push origin v0.17.0
        ```

    3.  On [the releases page](https://github.com/paritytech/subxt/releases), publish a release from that
        tag with the changelog section as description. Tick "Set as a pre-release" for a `-beta`.

    Publishing the release runs the `Release` workflow: it packages the crates and dispatches the
    publisher, which publishes whatever crates.io does not have yet, in dependency order. Follow the run
    here, then the `Publish paritytech/subxt` run in `crates_publish_automation`, then check crates.io.

11. If it fails, the `Release` run is red (nothing was published) or the publisher opens an issue labelled
    `failure` here. Fix the cause and re-run the `Release` workflow; already published versions are skipped.
    Never move a pushed tag: if code has to change, release the next patch version.

    Backports use the same flow from a `v0.N.x` branch, with "Set as the latest release" unticked.
