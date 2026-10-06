# voxcode / harness

Rust local-first coding harness. MVP building blocks, not a finished consumer app.
No telemetry or hosted dashboard. The native core does not load ML weights.

## Quick start

Download your platform's `harness` release. Python dependencies and models are
not bundled. Git is needed for worker isolation. Models stay local.

```
harness init config.json
harness doctor
harness add ~/.voxcode my-project /path/to/repo
harness serve ~/.voxcode                 # http://127.0.0.1:8484
harness session config.json ~/.voxcode   # local voice session
harness voice config.json --audio code.wav
harness work config.json /path/to/repo 'add a small function'
harness parallel config.json /path/to/repo 'task one' 'task two'
harness test /path/to/trusted/repo cargo test
harness map /path/to/repo
harness remember ~/.voxcode my-project 'Use small Rust modules'
harness recall ~/.voxcode my-project Rust
harness route 'review security architecture' --escalate
harness update --check
harness update
```

Voice commands: `switch project NAME`, `list projects`, `status`, `remember ...`,
`work on ...`, and `stop listening`. Workers generate patches under `.voxcode`,
not automatic merges or pushes. Review them before applying. Repo scripts are
not run automatically. The explicit `test` command executes your chosen
program in the chosen trusted directory and is NOT a security sandbox.
Worktrees isolate changes but do not sandbox processes or hide machine secrets.

## Local voice

Install Python 3 and `requirements-voice.txt`; install whisper.cpp and local
speech weights. Set `WHISPER_CLI`, `WHISPER_MODEL`, `KOKORO_CONFIG`,
`KOKORO_MODEL`, `KOKORO_VOICE_FILE`. Configure `speech.py`'s absolute path in the
voice adapter arrays. Windows may use `python` rather than `python3`.
Silero VAD uses its packaged model. Whisper and Kokoro use explicitly provided
local files; no weight downloader is included. Adapter programs run without a
shell and have timeouts. `speech.py` requires local microphone permissions.

Kokoro's phonemizer may need platform dependencies. Use headphones for barge-in:
there is no echo cancellation. Barge-in captures the next utterance while
stopping playback; live microphone acceptance is still required. Model processes
exit after turns; persistent model loading/idle unloading is not implemented.
Whisper vocabulary comes from project symbols. Hardware backend selection is
configured in the external speech/model engine, not automatically benchmarked.

## Gateway, privacy and budgets

The default is a localhost OpenAI-compatible model endpoint, suitable for
llama.cpp or a LiteLLM gateway. Set a model identifier appropriate to your server.
Keys come from `key_env`, never files or logs. Native provider-specific APIs need
an OpenAI-compatible proxy; "every provider directly" is not claimed.

Cloud is off by default. Enable in gateway config and explicitly per project
using `harness privacy HOME PROJECT --allow-cloud`. Direct `work` calls have no
registry, so gateway `allow_cloud` itself is that command's privacy consent.
Cloud requires a budget object (cap_usd, max_call_usd, input_per_million,
output_per_million), budget_home and project. Prices must match the selected
provider/model. Reservations are conservative estimates and remain charged on
failure; this favors stopping early over overspending. They are not billing
reconciliation or a guarantee against a provider changing its prices.

Optional `cheap_model` and `strong_model` configure the rules router. Optional
`classifier` is a localhost gateway that may escalate, never bypass safety.
Classifier must not contain another classifier (do not make recursive configs).
A localhost `fallback` may answer when the selected endpoint fails. No trained
router model or fallback weights are shipped. Routes and voice task results go
to local SQLite events. There is no paid API test or cost-performance claim.

## Projects and dashboard

SQLite stores registry, FTS memory, timelines and spend reservations. Tree-sitter
parses Rust, Python and JavaScript symbols. Other supported file extensions use a
bounded line parser. Architecture cards show module symbols and import text,
not a resolved dependency graph. Index is recomputed on dashboard refresh;
embeddings/sqlite-vec, MCP tools and Tauri/tray packaging are not implemented.
The dashboard binds loopback only, is read-only except explicit update install,
and refuses unexpected Host/Origin on mutations. Don't expose its port publicly.

## Update integrity

New releases include SHA256SUMS. Updates refuse missing/bad checksums before
extracting/replacing the executable. CLI and dashboard both use this path.
Checksums detect corruption, not a compromised publisher. No signatures yet.
Dashboard install asks for confirmation and needs an app restart. Releases older
than checksum support are refused for installation by the hardened updater.

## What was actually tested

Linux x64 native compilation/tests; fixture-backed voice gateway turn; isolated
worker edits, patch output and cleanup; concurrent workers; registry, privacy,
SQLite FTS; local dashboard desktop/mobile layouts; tampered checksums rejected.
CI builds/tests Linux x64/arm64, macOS arm64/Intel, Windows x64.
No real mic, speech weights, GPU backend, identifier accuracy or voice latency
benchmark. Run `benchmark.py` with real code-term recordings per configured
backend; it reports measured WER and timings rather than invented targets.
The release binary and updater acceptance are verified separately from local
builds. Check release notes for the latest verification state.

## Scope still open

No complete zero-setup desktop installer, wake word, speech echo cancellation,
worker sandbox, autonomous git merge/push, fine-tuned System 1 classifier,
provider billing reconciliation, graph edge resolution, or embedded speech
runtime. These are gaps, not completed features. See ACCEPTANCE.md.

Release acceptance (2026-10-05): v0.1.22 passed native tests and executable
smoke checks on all five CI runners. Linux x64 archive checksum independently
verified, then released CLI version/doctor/update-check ran successfully.
The CI GNU/Linux binary may warn about GLIBC_2.39 on older distributions;
compatibility on older distros is not claimed. Hardened self-replacement is
being checked against the next release; real speech/hardware checks remain open.
