# 0squerk

Comment-driven merge bot for the `0nybl` fleet. Comment `/merge` on a pull
request and squerk rebuilds the squashed result in an isolated runner, runs the
tests, and squash-merges only if they pass — so the default branch stays green.

## Use

Comment `/merge` on a PR (you need write access). squerk will:
1. rebuild the squash of your PR onto the default branch,
2. run the test command (`.0merge.toml` `command`, default `cargo test --all`),
3. squash-merge if green, or comment the failure if not.

Merges are serialized, so concurrent `/merge`s can't race the base branch.

## Config (`.0merge.toml`, optional)

    command = "cargo test --all"

## CLI

    0squerk decide --event event.json --config .0merge.toml --out decision.json

`decide` is pure (no network): it turns the comment event + config into
`{proceed, reason, command}`. All git/merge actions live in `action/0squerk.yml`.
Merges are attributed to the `0squerk[bot]` GitHub App.

Cargo package/lib `squerk`; binary `0squerk`.
