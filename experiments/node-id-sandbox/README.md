# solc Node ID Sandbox

This sandbox tests three behaviors:

1. Whether `node.id` is stable for identical recompilations.
2. Whether changing an unrelated file changes IDs in untouched files.
3. Whether full-project vs subset compilation produces different IDs for the same declaration.

## Run

```bash
./experiments/node-id-sandbox/run.sh
```

For real-project checks against `v4-core`:

```bash
./experiments/node-id-sandbox/run-v4-core.sh
```

To compute transitive affected files from changed declaration files:

```bash
./experiments/node-id-sandbox/affected-files-planner.sh ./v4-core src/Extsload.sol
```

## Output

- Raw compiler output and snapshots:
  - `/tmp/solc-nodeid-sandbox/out/`
- Human-readable summary:
  - `/tmp/solc-nodeid-sandbox/out/report.txt`
