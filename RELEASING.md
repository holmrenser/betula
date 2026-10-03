# Releasing

One tag releases everything. Pushing `vX.Y.Z` runs `.github/workflows/release.yml`, which publishes
`betula-schema` X.Y.Z to PyPI, npm, and crates.io and creates the GitHub release. The schema `$id`s,
all three packages, and the tag always carry the same version (see the versioning policy in
`CHANGELOG.md`).

## Each release

1. Make sure the `## Unreleased` section of `CHANGELOG.md` describes the release.
2. `python3 scripts/bump_version.py X.Y.Z` sets every version (schemas, manifests, lockfiles, docs)
   and renames `## Unreleased` to `## X.Y.Z`.
3. `scripts/generate_bindings.sh && python3 scripts/check_versions.py`
4. Commit, push to `main`, and wait for CI. Besides the tests, it dry-runs all three packages
   (`python -m build` + `twine check`, `npm pack`, `cargo publish --dry-run`).
5. `git tag -a vX.Y.Z -m "betula X.Y.Z" && git push origin vX.Y.Z`

The release workflow then:

- checks that the tag, every version string, and the changelog agree;
- reruns the full bindings suite (tests, generated-code drift, packaging);
- publishes to each registry in its own job, skipping a version that's already published, so a
  partly failed release can be finished by re-running the failed jobs;
- creates the GitHub release with that version's changelog section as notes.

## One-time setup

The workflow uses trusted publishing: each registry trusts this repository's `release.yml` and
issues a short-lived token per run, so no registry secrets are stored in GitHub. Each registry
needs to know: owner `holmrenser`, repository `betula`, workflow `release.yml`, and the environment
named below. Check each registry's docs for the current steps; the outline:

- **PyPI**: add a *pending publisher* for the project `betula-schema` (account settings, Publishing)
  with environment `pypi`. This works before the project exists, so the workflow can do the first
  upload.
- **npm** (environment `npm`) and **crates.io** (environment `crates-io`): trusted publishing is
  configured from an existing package's settings, so publish the first version by hand from the
  release commit, then add the trusted publisher:
  - `cd bindings/typescript && npm ci && npm publish` while logged in to npm;
  - `cd bindings/rust && cargo publish` with a crates.io API token.

  Then push the tag. The npm and crates.io jobs see the version is already published and skip it,
  PyPI uploads, and the GitHub release is created.
- The GitHub environments `pypi`, `npm`, and `crates-io` are created on the first run. Adding
  required reviewers to them makes each publish wait for approval.

After the first release, remove the "not published yet" note from `docs/getting-started.md`.
