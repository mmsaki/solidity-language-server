**#90** · v0.1.20 · `7e1491a` · PR
**v0.1.20** · v0.1.19 · `96b5608` · Installed release (forge-only)

| Benchmark                        | v0.1.20 (7e1491a) | v0.1.19 (96b5608) |       Delta | RSS v0.1.20 (7e1491a) | RSS v0.1.19 (96b5608) |
|----------------------------------|-------------------|-------------------|-------------|-----------------------|-----------------------|
| initialize                       |            3.20ms |            4.95ms | 1.5x slower |                    -- |                    -- |
| textDocument/diagnostic          |           178.2ms |           159.4ms | 1.1x faster |                 9.0MB |                10.9MB |
| textDocument/definition          |                -- |                -- |          -- |                 8.9MB |                 8.6MB |
| textDocument/declaration         |                -- |                -- |          -- |                 9.0MB |                 9.3MB |
| textDocument/hover               |            2.42ms |            2.33ms | 1.0x (tied) |                 8.9MB |                 9.2MB |
| textDocument/references          |            1.54ms |            1.66ms | 1.1x slower |                 9.0MB |                 8.4MB |
| textDocument/completion          |            0.21ms |            0.20ms | 1.0x (tied) |                 9.1MB |                 9.3MB |
| textDocument/rename              |            2.92ms |            2.89ms | 1.0x (tied) |                 9.1MB |                 8.7MB |
| textDocument/prepareRename       |            0.11ms |            0.10ms | 1.1x faster |                 9.0MB |                 8.8MB |
| textDocument/documentSymbol      |            1.06ms |            1.04ms | 1.0x (tied) |                 8.9MB |                10.8MB |
| textDocument/documentLink        |            9.47ms |            12.6ms | 1.3x slower |                 8.9MB |                 9.8MB |
| textDocument/formatting          |           invalid |            15.3ms |          -- |                 9.1MB |                 9.4MB |
| textDocument/semanticTokens/full |            1.56ms |            1.51ms | 1.0x (tied) |                 8.9MB |                10.1MB |
| workspace/symbol                 |            0.94ms |            1.05ms | 1.1x slower |                 8.9MB |                 7.9MB |
