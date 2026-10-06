# Acceptance ledger

1. Voice loop spike: native adapters + Silero/whisper/Kokoro bridge; mock-backed
   end-to-end core turn verified. Real model/mic/hardware comparison not verified.
2. Worker: reads bounded tracked files, requests JSON edits, writes isolated
   worktree, returns patch, cleans worktree. Mock model edit and primary-tree
   safety verified. Tests are explicit, not auto-run; voice reports in session.
3. Projects: SQLite registry/FTS memory, multi-project voice selection, repo map.
   Registry and FTS tested; tree-sitter Rust/Python/JS, fallback for others.
4. Dashboard: loopback project cards, module symbols/imports, recent timeline,
   update check/install. Browser desktop/mobile tested; no resolved graph edges.
5. Router: rules and optional localhost classifier, cheap/strong choice,
   conservative per-project spend reservations. Safety/cap tests pass; classifier
   model not trained or benchmarked. No actual paid provider usage tested.
6. Parallel/polish: concurrent isolated worker patches tested; local endpoint
   fallback, local barge-in bridge, risk refusal, checksum update UX. Real barge-in
   and spoken confirmation acceptance not verified. Risky actions are refused,
   not automatically executed following an ambiguous spoken yes.

Checklists that require the user's machine:
- Install local speech dependencies and weights; grant mic access.
- Record actual coding identifiers and calculate WER using benchmark.py.
- Compare available native backends; record OS, device, model, timings and RSS.
- Test headphone barge-in, speech stop, vocabulary and project selection.
- Run native dashboard/update on macOS and Windows.
- Select provider, price configuration, project privacy and monthly limits.

No benchmark numbers or portability claims without these checks.
