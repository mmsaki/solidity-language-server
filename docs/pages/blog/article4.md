# v0.1.26: Cleaner Auto-Import and Better Bench Coverage

v0.1.26 improves file operations, auto-import, and benchmark coverage.

## What shipped

### 1) File operations are now configurable

You can control file operation behavior with three settings:

- `fileOperations.templateOnCreate`
- `fileOperations.updateImportsOnRename`
- `fileOperations.updateImportsOnDelete`

Defaults are enabled in v0.1.26.

In plain language:

- `templateOnCreate`: when you create a new `.sol` file, the server fills it with a Solidity starter template and derives the contract name from the filename.
- `updateImportsOnRename`: when you rename a Solidity file, the server updates matching import paths in other files.
- `updateImportsOnDelete`: when you delete a Solidity file, the server removes imports that point to the deleted file.

These defaults work out of the box. Teams that prefer manual workflows can disable them.

Naming behavior for `templateOnCreate`:

- `Vault.sol` → `contract Vault {}`
- `Vault.t.sol` → `contract VaultTest is Test {}`
- `Vault.s.sol` → `contract VaultScript is Script {}`

Example (Neovim):

```lua
settings = {
  ["solidity-language-server"] = {
    fileOperations = {
      templateOnCreate = true,
      updateImportsOnRename = true,
      updateImportsOnDelete = true,
    },
  },
}
```

### 2) `templateOnCreate` is the canonical naming

Scaffolding settings now use `templateOnCreate` as the single name across clients.

### 3) Create-file scaffolding lifecycle was fixed

`willCreateFiles` and `didCreateFiles` handling was fixed to avoid timing issues between disk and editor buffers.
New files now receive scaffold content without empty-file races or duplicate insertion.

Example scaffold produced for `MyToken.sol`:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract MyToken {

}
```

What generated files look like in v0.1.26:

`Vault.sol`

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Vault {

}
```

`Vault.t.sol`

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Test} from "forge-std/Test.sol";

contract VaultTest is Test {

}
```

`Vault.s.sol`

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Script} from "forge-std/Script.sol";

contract VaultScript is Script {

}
```

### 4) Auto-import completion is more reliable

Top-level symbol completion and import-edit attachment were improved.
Completions that should add imports now attach those edits more consistently.

Example:

```solidity
// In A.sol
contract A {
    function f() external {}
}

// In B.sol (typing `A`)
contract B {
    A a;
}
```

Completion now more reliably attaches import edits like:

```solidity
import {A} from "./A.sol";
```

### 5) Benchmarks expanded around file operations

Coverage was added/updated for file operation lifecycle flows:

- `workspace/willCreateFiles`
- `workspace/willRenameFiles`
- `workspace/willDeleteFiles`

Representative lifecycle request shapes:

```json
{
  "method": "workspace/willCreateFiles",
  "params": { "files": [{ "uri": "file:///.../C.sol" }] }
}
```

```json
{
  "method": "workspace/willRenameFiles",
  "params": {
    "files": [{
      "oldUri": "file:///.../A.sol",
      "newUri": "file:///.../AA.sol"
    }]
  }
}
```

```json
{
  "method": "workspace/willDeleteFiles",
  "params": { "files": [{ "uri": "file:///.../A.sol" }] }
}
```

## Current benchmark snapshot context

Benchmark summaries for v0.1.26 now include file-operation methods in the reporting flow.

## Upgrade notes

If you’re already on `solidity-language-server`, v0.1.26 is a straightforward update:

```sh
cargo install solidity-language-server
```

Or download binaries from:

- [CHANGELOG.md](https://github.com/mmsaki/solidity-language-server/blob/main/CHANGELOG.md)

If your team prefers manual file operation behavior, you can toggle the settings above.
