#!/usr/bin/env bash
set -euo pipefail

PROJECT_DIR="$(cd "$(dirname "$0")/../../v4-core" && pwd)"
OUT="/tmp/solc-nodeid-v4-core"

mkdir -p "$OUT"

TARGET_DECL_FILE="src/Extsload.sol"
TARGET_REF_FILE="src/PoolManager.sol"
UNRELATED_FILE="src/libraries/CustomRevert.sol"

backup_and_edit() {
  local file="$1"
  local marker="$2"
  cp "$PROJECT_DIR/$file" "$PROJECT_DIR/$file.bak.nodeid"
  printf "\n%s\n" "$marker" >> "$PROJECT_DIR/$file"
}

backup_and_prepend_after_pragma() {
  local file="$1"
  local marker="$2"
  cp "$PROJECT_DIR/$file" "$PROJECT_DIR/$file.bak.nodeid"
  perl -0777 -pe "s/(pragma solidity [^;]+;\\n)/\$1\\n$marker\\n/s" "$PROJECT_DIR/$file" > "$PROJECT_DIR/$file.tmp.nodeid"
  mv "$PROJECT_DIR/$file.tmp.nodeid" "$PROJECT_DIR/$file"
}

restore_edit() {
  local file="$1"
  if [[ -f "$PROJECT_DIR/$file.bak.nodeid" ]]; then
    mv "$PROJECT_DIR/$file.bak.nodeid" "$PROJECT_DIR/$file"
  fi
}

cleanup() {
  restore_edit "$UNRELATED_FILE"
  restore_edit "$TARGET_DECL_FILE"
}
trap cleanup EXIT

build_stdjson() {
  local mode="$1"
  local json="$2"
  local remaps files
  remaps=$(jq -R -s 'split("\n")|map(select(length>0))' "$PROJECT_DIR/remappings.txt")
  if [[ "$mode" == "full" ]]; then
    files=$(cd "$PROJECT_DIR" && find src -name '*.sol' | sort | jq -R -s 'split("\n")|map(select(length>0))')
  else
    files=$(jq -n --arg f "$TARGET_DECL_FILE" '[$f]')
  fi

  jq -n \
    --argjson remaps "$remaps" \
    --argjson files "$files" '
      def to_sources(xs): xs | map({key: ., value: {urls: [.]}}) | from_entries;
      {
        language: "Solidity",
        sources: to_sources($files),
        settings: {
          optimizer: {enabled: true, runs: 44444444},
          viaIR: true,
          evmVersion: "cancun",
          metadata: {bytecodeHash: "none"},
          remappings: $remaps,
          outputSelection: {"*": {"": ["ast"]}}
        }
      }
    ' > "$json"
}

compile() {
  local in_json="$1"
  local out_json="$2"
  (
    cd "$PROJECT_DIR"
    solc --standard-json --base-path . --include-path lib --include-path node_modules --allow-paths . < "$in_json" > "$out_json"
  )
}

extract_snapshot() {
  local src_json="$1"
  local out_tsv="$2"
  jq -r '
    def nodes($f): (.sources[$f].ast.nodes // []);
    def pick_contract($f;$name):
      nodes($f)[]
      | select(.nodeType=="ContractDefinition" and .name==$name)
      | .id;
    def pick_fn($f;$c;$fn):
      nodes($f)[]
      | select(.nodeType=="ContractDefinition" and .name==$c)
      | .nodes[]?
      | select(.nodeType=="FunctionDefinition" and .name==$fn)
      | .id;
    def pick_ref($f;$name):
      [
        .sources[$f].ast
        | .. | objects
        | select(.nodeType=="Identifier" and .name==$name and has("referencedDeclaration"))
        | .referencedDeclaration
      ][0];
    [
      ["src/Extsload.sol","Extsload","ContractDefinition", (pick_contract("src/Extsload.sol";"Extsload"))],
      ["src/Extsload.sol","Extsload.extsload(bytes32)","FunctionDefinition", (pick_fn("src/Extsload.sol";"Extsload";"extsload"))],
      ["src/PoolManager.sol","ref:Extsload","Identifier.referencedDeclaration", (pick_ref("src/PoolManager.sol";"Extsload"))]
    ] | .[] | select(.[3] != null) | @tsv
  ' "$src_json" > "$out_tsv"
}

compare_snapshots() {
  local lhs="$1"
  local rhs="$2"
  join -t $'\t' -j 1 \
    <(awk -F'\t' '{print $1"|" $2"|" $3"\t"$4}' "$lhs" | sort) \
    <(awk -F'\t' '{print $1"|" $2"|" $3"\t"$4}' "$rhs" | sort) \
    | awk -F'\t' '{status=($2==$3?"same":"changed"); print $1"\t"$2"\t"$3"\t"status}'
}

build_stdjson full "$OUT/full.in.json"
compile "$OUT/full.in.json" "$OUT/full.1.json"
extract_snapshot "$OUT/full.1.json" "$OUT/full.1.tsv"

build_stdjson full "$OUT/full.rebuild.in.json"
compile "$OUT/full.rebuild.in.json" "$OUT/full.2.json"
extract_snapshot "$OUT/full.2.json" "$OUT/full.2.tsv"

backup_and_edit "$UNRELATED_FILE" "// node-id-sandbox: unrelated edit"
build_stdjson full "$OUT/full.after-unrelated.in.json"
compile "$OUT/full.after-unrelated.in.json" "$OUT/full.3.json"
extract_snapshot "$OUT/full.3.json" "$OUT/full.3.tsv"
restore_edit "$UNRELATED_FILE"

build_stdjson subset "$OUT/subset.in.json"
compile "$OUT/subset.in.json" "$OUT/subset.1.json"
extract_snapshot "$OUT/subset.1.json" "$OUT/subset.1.tsv"

backup_and_prepend_after_pragma "$TARGET_DECL_FILE" "error NodeIdSandboxMarker();"
build_stdjson full "$OUT/full.after-decl-edit.in.json"
compile "$OUT/full.after-decl-edit.in.json" "$OUT/full.4.json"
extract_snapshot "$OUT/full.4.json" "$OUT/full.4.tsv"
restore_edit "$TARGET_DECL_FILE"

{
  echo "solc node.id report (v4-core)"
  echo "project: $PROJECT_DIR"
  echo
  echo "[scenario 1] identical full-project recompilation"
  echo "key	id_run1	id_run2	status"
  compare_snapshots "$OUT/full.1.tsv" "$OUT/full.2.tsv"
  echo
  echo "[scenario 2] unrelated edit then full-project recompile"
  echo "key	id_before	id_after	status"
  compare_snapshots "$OUT/full.1.tsv" "$OUT/full.3.tsv"
  echo
  echo "[scenario 3] full-project vs subset compile"
  echo "key	id_full	id_subset	status"
  compare_snapshots "$OUT/full.1.tsv" "$OUT/subset.1.tsv" | grep '^src/Extsload\.sol|' || true
  echo
  echo "[scenario 4] declaration-file edit then full-project recompile"
  echo "key	id_before	id_after	status"
  compare_snapshots "$OUT/full.1.tsv" "$OUT/full.4.tsv"
} | tee "$OUT/report.txt"

echo
echo "Saved report: $OUT/report.txt"
