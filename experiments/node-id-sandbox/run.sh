#!/usr/bin/env bash
set -euo pipefail

ROOT="/tmp/solc-nodeid-sandbox"
WORK="$ROOT/work"
OUT="$ROOT/out"

rm -rf "$ROOT"
mkdir -p "$WORK" "$OUT"
cp -R "$(dirname "$0")/contracts/." "$WORK/"

build_stdjson() {
  local mode="$1"
  local json="$2"
  if [[ "$mode" == "full" ]]; then
    jq -n '{
      language: "Solidity",
      sources: {
        "A.sol": { urls: ["A.sol"] },
        "B.sol": { urls: ["B.sol"] },
        "C.sol": { urls: ["C.sol"] }
      },
      settings: {
        outputSelection: { "*": { "": ["ast"] } }
      }
    }' > "$json"
  else
    jq -n '{
      language: "Solidity",
      sources: {
        "A.sol": { urls: ["A.sol"] }
      },
      settings: {
        outputSelection: { "*": { "": ["ast"] } }
      }
    }' > "$json"
  fi
}

compile() {
  local in_json="$1"
  local out_json="$2"
  (cd "$WORK" && solc --standard-json --base-path . --allow-paths . < "$in_json" > "$out_json")
}

extract_snapshot() {
  local src_json="$1"
  local out_tsv="$2"
  jq -r '
    def nodes($f): (.sources[$f].ast.nodes // []);
    def pick($f;$n;$k):
      nodes($f)[]
      | select(.nodeType==$n and .name==$k)
      | .id;
    def pick_contract_fn($f;$c;$fn):
      nodes($f)[]
      | select(.nodeType=="ContractDefinition" and .name==$c)
      | .nodes[]
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
      ["A.sol","MyStruct","StructDefinition", (pick("A.sol";"StructDefinition";"MyStruct"))],
      ["A.sol","MyError","ErrorDefinition", (pick("A.sol";"ErrorDefinition";"MyError"))],
      ["A.sol","MyEvent","EventDefinition", (pick("A.sol";"EventDefinition";"MyEvent"))],
      ["A.sol","MY_CONST","VariableDeclaration", (pick("A.sol";"VariableDeclaration";"MY_CONST"))],
      ["A.sol","MyInterface","ContractDefinition", (pick("A.sol";"ContractDefinition";"MyInterface"))],
      ["A.sol","MyLib","ContractDefinition", (pick("A.sol";"ContractDefinition";"MyLib"))],
      ["A.sol","A","ContractDefinition", (pick("A.sol";"ContractDefinition";"A"))],
      ["A.sol","A.foo","FunctionDefinition", (pick_contract_fn("A.sol";"A";"foo"))],
      ["B.sol","ref:A","Identifier.referencedDeclaration", (pick_ref("B.sol";"A"))],
      ["C.sol","C","ContractDefinition", (pick("C.sol";"ContractDefinition";"C"))]
    ] | .[] | @tsv
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

printf "\n// unrelated edit for stability check\n" >> "$WORK/C.sol"
build_stdjson full "$OUT/full.after-edit.in.json"
compile "$OUT/full.after-edit.in.json" "$OUT/full.3.json"
extract_snapshot "$OUT/full.3.json" "$OUT/full.3.tsv"

build_stdjson subset "$OUT/subset.in.json"
compile "$OUT/subset.in.json" "$OUT/subset.a.json"
extract_snapshot "$OUT/subset.a.json" "$OUT/subset.a.tsv"

# Edit A.sol to force ID reallocation in declaration graph.
perl -0777 -pe 's/pragma solidity \^0\.8\.26;\n/pragma solidity ^0.8.26;\n\nerror NewTopLevelError();\n/s' "$WORK/A.sol" > "$WORK/A.sol.tmp"
mv "$WORK/A.sol.tmp" "$WORK/A.sol"
build_stdjson full "$OUT/full.after-a-edit.in.json"
compile "$OUT/full.after-a-edit.in.json" "$OUT/full.4.json"
extract_snapshot "$OUT/full.4.json" "$OUT/full.4.tsv"

{
  echo "solc node.id sandbox report"
  echo "root: $ROOT"
  echo
  echo "[scenario 1] identical full-project recompilation"
  echo "key	id_run1	id_run2	status"
  compare_snapshots "$OUT/full.1.tsv" "$OUT/full.2.tsv"
  echo
  echo "[scenario 2] unrelated edit in C.sol then full-project recompile"
  echo "key	id_before	id_after	status"
  compare_snapshots "$OUT/full.1.tsv" "$OUT/full.3.tsv" | grep '^A\.sol|' || true
  echo
  echo "[scenario 3] A.sol full-project vs A.sol subset compile"
  echo "key	id_full	id_subset	status"
  compare_snapshots "$OUT/full.1.tsv" "$OUT/subset.a.tsv" | grep '^A\.sol|' || true
  echo
  echo "[scenario 4] edit A.sol and recompile full project"
  echo "key	id_before	id_after	status"
  compare_snapshots "$OUT/full.1.tsv" "$OUT/full.4.tsv" | grep -E '^(A\.sol|B\.sol\|ref:A)' || true
} | tee "$OUT/report.txt"

echo
echo "Saved report: $OUT/report.txt"
