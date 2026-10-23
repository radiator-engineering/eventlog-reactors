# Agent guide: running reactors

Moved from the `event-log-coordination` skill in the eventlog repo.

## Reactors (`eventlog-reactors`)

Skip this section in a repo without `.context/eventlog-setup.toml` or
reactor panes. Up to eventlog 0.5 these commands were `eventlog react`,
`eventlog action` and the reactor part of `eventlog setup`; they now live in
the separate `eventlog-reactors` binary (`cargo install eventlog-reactors`).

A reactor acts on matching events with no prompt. The runtime owns lock,
resume, intent, voter, action, violation check and ack; you supply the
action. Drive a committer on `result`, which carries `paths=`
(`decision` has no `paths` field):

```sh
eventlog-reactors react --as committer --on result --git -- bash .context/bin/commit-action.sh
eventlog-reactors react test <seq> --as committer --git -- bash .context/bin/commit-action.sh   # dry run
```

### Reactor setup

Run `eventlog-reactors setup preview`, inspect the output, then
`eventlog-reactors setup apply`. It runs the log setup, then writes the
project-managed `.context/eventlog-setup.toml` and the Drove helper
`.context/eventlog-reactors.star`, and ignores `.context/*.reactor.lock/`.
The policy declares the committer/doc-worker identities, default models
(`composer-2.5-fast` and `claude-sonnet`), timeouts, documentation roots, an
argv-form docs command, and `[invocation]` (`eventlog`, and the optional
`reactors`, default `eventlog-reactors`). `setup` never rewrites an existing
`Drovefile`, restarts a reactor, removes a lock/history file, or advances a
checkpoint. `upgrade` is conservative: it preserves valid project edits,
regenerates an unedited helper (a 0.5 helper moves to `eventlog-reactors`),
and fails before writing when the configuration is malformed.
`eventlog-reactors doctor` reports the setup and each reactor lock.

The generated helper's hooks call `eventlog lifecycle start <id> --role
reactor ...`; add `--role reactor` to a hand-written reactor hook to keep the
0.5 role.

### Packaged actions

Package reactor actions as direct argv:

```sh
eventlog-reactors react --as committer --on result --git -- eventlog-reactors action commit
eventlog-reactors react --as doc-worker --on ack --filter by=committer --filter outcome=committed --git -- eventlog-reactors action docs
```

`action commit` stages and commits only validated `EVENTLOG_PATHS`; it rejects
target paths that were already staged and keeps unrelated staged work intact.
An optional `[commit].command` argv invokes the project's commit-author model
in a disposable clone, after authorized additions, edits and deletions have
been copied and staged. The command must create commits there. It receives
`EVENTLOG_MODEL`, the concrete `EVENTLOG_PATHS`, the driving event on stdin,
and `EVENTLOG_COMMIT_MESSAGE`; an argv entry exactly `{model}` is replaced
with `[commit].model`. Model labels alone do not invoke a model. For example,
configure `command = ["python3", ".context/bin/model-command.py", "commit"]`
and have that thin command invoke Cursor with `EVENTLOG_MODEL`.

Every produced commit is checked against the authorized paths before its
history is published to the source branch. Unrelated staging is preserved;
changed source HEAD/index/files abort publication. Multiple accepted commits
are reported with all their OIDs. A command that commits successfully and
then exits nonzero still reports the verified commits, with its exit status
in the detail. No commit after a command failure is a failed action. A clean
authorized scope skips without invoking the command. Without a configured
command, direct `--message` behavior is retained. Commands are trusted local
programs; the disposable clone is file/commit isolation, not an OS sandbox.

`action docs` executes `[docs].command` as argv (no shell interpolation) and
reports the added, deleted, or modified files below `[docs].roots`, including
files already dirty before the command. The active log and its reserved
sidecars are excluded from content snapshots so concurrent reactor writes
do not fail the docs action; other files outside the roots remain checked,
except paths listed in `[docs].exclude` (prefixes or globs, default empty).
No documentation changes returns `skipped` without appending a result.
Set `docs.command` to a local stub in
tests. An empty command or a failing model command is a failed action, never a
successful no-op. Under a reactor (when `EVENTLOG_LOG` and `EVENTLOG_REF` are
set), it appends one `result` attributed to `[docs].identity`; when that result
comes back through the doc worker it is skipped, preventing the doc-result
loop. Native `react` still owns intent/ack/resume handling.

Sanction reactors once: `decision key=log-writers value=controller-plus-reactors`.
Run each reactor in a terminal pane that outlives any agent's turn, and record
its `spawn` and `claim`. Details: [agent-log-reactors.md](agent-log-reactors.md); script:
[agent-reactor-example.md](agent-reactor-example.md).

