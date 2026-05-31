# squerk — comment-driven merge bot (Rultor-like)

**Date:** 2026-05-31
**Status:** Design approved, ready for implementation plan
**Form:** GitHub Action for `0nybl` Rust repositories (no AI)

## Summary

squerk merges pull requests on a `/merge` comment, the Rultor way: it rebuilds
the squashed merge result in an isolated runner, runs the project's tests, and
only merges (squash) into the default branch if they pass. The default branch
stays green because the exact merged content is tested before it lands. Third
tool in the `0nybl` fleet.

The Rust engine is a small pure decision core; all git/network/issue actions
live in the workflow. Merges and comments are attributed to a dedicated
`0squerk` GitHub App (`0squerk[bot]`).

Naming: repository and command are `0squerk`. The Cargo package and lib are
`squerk`; the binary target is `0squerk` (`[[bin]] name = "0squerk"`).

## Goals

- A `/merge` comment by an authorized user merges the PR after a green rebuild.
- The merged result is tested in isolation before landing → default branch
  always green.
- Merges are serialized so concurrent `/merge`s never race the base branch.
- Unauthorized `/merge` attempts are rejected with a comment, no merge.
- Failures are reported back as a comment; nothing is merged.
- Pure, offline-testable decision core. Zero AI.

## Non-Goals

- No `deploy`/`release` commands (merge only).
- No queueing UI; serialization is via Actions `concurrency`.
- No rebase/merge-commit styles — squash only.
- The bot does not modify PR code.

## Trigger & authorization

- **Event:** `issue_comment` (created) on a pull request.
- **Trigger phrase:** the comment body, trimmed, equals `/merge`.
- **Authorization:** the comment author's `author_association` must be one of
  `OWNER`, `MEMBER`, `COLLABORATOR` (i.e. write access). Anyone else → rejection
  comment, no merge.

## Configuration: `.0merge.toml`

Optional, at repo root:

```toml
command = "cargo test --all"   # test command run on the rebuilt merge; this is the default
```

If the file is absent or the key is missing, the default `cargo test --all` is
used.

## Architecture

### Decision core (Rust binary `0squerk`, pure)

Single subcommand:

```
0squerk decide --event <event.json> --config <.0merge.toml> --out <decision.json>
```

- **Input:** `event.json` is the GitHub `issue_comment` event payload (written by
  the workflow from `${{ toJSON(github.event) }}`); `.0merge.toml` is the repo
  config (may be absent).
- **Logic (pure):**
  - `proceed = true` only if: the event is a comment on a pull request
    (`issue.pull_request` present), the trimmed comment body equals `/merge`,
    and `comment.author_association` ∈ {OWNER, MEMBER, COLLABORATOR}.
  - `command` = config `command` or the default `cargo test --all`.
  - `reason` explains a non-proceed (`"not a pull request"`, `"not a /merge
    command"`, `"author lacks write access"`).
- **Output:** `decision.json`:

  ```json
  { "proceed": true, "reason": "", "command": "cargo test --all" }
  ```

All file I/O is local; no network. Fully offline-testable with fixture event
payloads and configs.

**Modules:**

- `event` — deserialize the parts of the payload we need
  (`comment.body`, `comment.author_association`, `issue.pull_request`).
- `config` — parse `.0merge.toml` → optional command.
- `trigger` — is the comment body the `/merge` command?
- `auth` — is an `author_association` write-level?
- `decide` — combine the above into a `Decision`.

### Workflow (`0squerk.yml`)

```
on: issue_comment (types: [created])
concurrency:
  group: squerk-${{ github.repository }}
  cancel-in-progress: false      # serialize; never cancel an in-flight merge
permissions: contents: read
```

Job steps (only meaningful when the comment is `/merge` on a PR; a cheap `if`
prefilter on the body avoids spinning up for unrelated comments):

1. **Mint app token** — `actions/create-github-app-token` with
   `SQUERK_APP_ID` / `SQUERK_APP_PRIVATE_KEY` (the `0squerk` App).
2. **Checkout** the default branch (full history: `fetch-depth: 0`).
3. **Write** `event.json` from `${{ toJSON(github.event) }}`.
4. **Install** `0squerk` (`cargo install --git ... --bin 0squerk squerk`).
5. **Decide** — `0squerk decide --event event.json --config .0merge.toml --out decision.json`.
6. **Reject if not proceeding** — if `proceed=false`, post the `reason` as a PR
   comment and stop (success exit; nothing to do).
7. **Acknowledge** — post a "🛠 squerk is merging…" comment.
8. **Record base** — `B0=$(git rev-parse HEAD)` on the default branch.
9. **Rebuild squashed result** — fetch the PR head, `git merge --squash`,
   `git commit -m "<PR title> (#<n>)"` onto the default branch (local only).
10. **Test** — run the decision's `command`. On failure: post a comment with the
    last lines of output and stop (exit 1) — **no merge**.
11. **Guard against races** — re-fetch the remote default branch; if its head no
    longer equals `B0`, post "base moved, re-run /merge" and stop.
12. **Merge** — `gh pr merge <n> --squash --delete-branch` (App token). Because
    the base is unchanged and the squash content matches what was just tested,
    GitHub's squash merge equals the tested tree.
13. **Report** — post "✅ merged by squerk" comment.

All git/gh/network is in the workflow; the binary only computes the decision.

### Branch protection / App permissions

- App `0squerk` repository permissions: **Contents: read & write**,
  **Pull requests: read & write**, **Issues: read & write**, Metadata: read.
- The default branch is protected so that only the `0squerk` App can
  merge/push; humans land changes exclusively through `/merge`. This is what
  makes the "tested base is unchanged" guarantee hold.

## Components & boundaries

- `event`, `config`, `trigger`, `auth`, `decide` — each pure, single-purpose,
  unit-tested in isolation.
- `main` — clap CLI, JSON read/write, wiring.
- `0squerk.yml` — orchestration + all side effects.

## Testing

- `trigger`: `/merge` matches; `/merge please`, `merge`, empty do not.
- `auth`: OWNER/MEMBER/COLLABORATOR allowed; CONTRIBUTOR/NONE/FIRST_TIMER denied.
- `config`: present command, absent file → default, malformed → default.
- `event`: extract body/association/is-PR; non-PR comment detected.
- `decide`: proceed path; each rejection reason; default-vs-config command.
- Integration: fixture event + config → expected `decision.json`; binary exits 0.

## Open questions (deferred, non-blocking)

- Install tag/version (start `v0.1.0`).
- Configurable trigger phrase (fixed `/merge` for now).
- Reporting the rebuild log as a artifact vs inline tail (inline tail for now).
