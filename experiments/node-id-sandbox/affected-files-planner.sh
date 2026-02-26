#!/usr/bin/env zsh
set -euo pipefail

if [[ $# -lt 2 ]]; then
  cat <<'EOF'
Usage:
  ./experiments/node-id-sandbox/affected-files-planner.sh <project_dir> <changed_file> [changed_file...]

Example:
  ./experiments/node-id-sandbox/affected-files-planner.sh ./v4-core src/Extsload.sol
EOF
  exit 1
fi

PROJECT_DIR="$(cd "$1" && pwd)"
shift

TMP_DIR="$(mktemp -d /tmp/solidity-lsp-affected.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

EDGES_TSV="$TMP_DIR/edges.tsv"
touch "$EDGES_TSV"

REMAPS_FILE="$PROJECT_DIR/remappings.txt"
SOL_FILES=()
while IFS= read -r line; do
  SOL_FILES+=("$line")
done < <(cd "$PROJECT_DIR" && find src test script -type f -name '*.sol' 2>/dev/null | sort)

to_abs() {
  local p="$1"
  if [[ "$p" = /* ]]; then
    perl -MCwd=abs_path -e 'print abs_path(shift)' "$p" 2>/dev/null || true
  else
    perl -MCwd=abs_path -e 'print abs_path(shift)' "$PROJECT_DIR/$p" 2>/dev/null || true
  fi
}

resolve_import_spec() {
  local importer_abs="$1"
  local spec="$2"
  local importer_dir candidate

  importer_dir="$(dirname "$importer_abs")"

  if [[ "$spec" == ./* || "$spec" == ../* ]]; then
    candidate="$(to_abs "$importer_dir/$spec")"
    [[ -n "$candidate" ]] && echo "$candidate"
    return 0
  fi

  if [[ -f "$REMAPS_FILE" ]]; then
    while IFS= read -r remap; do
      [[ -z "$remap" ]] && continue
      local prefix="${remap%%=*}"
      local target="${remap#*=}"
      if [[ "$spec" == "$prefix"* ]]; then
        local suffix="${spec#"$prefix"}"
        candidate="$(to_abs "$PROJECT_DIR/$target$suffix")"
        if [[ -n "$candidate" ]]; then
          echo "$candidate"
          return 0
        fi
      fi
    done < "$REMAPS_FILE"
  fi

  candidate="$(to_abs "$PROJECT_DIR/$spec")"
  [[ -n "$candidate" ]] && echo "$candidate"
  return 0
}

extract_import_specs() {
  local file="$1"
  perl -ne '
    while (/import\s+(?:[^;]*?\s+from\s+)?["'\'']([^"'\'']+)["'\'']/g) {
      print "$1\n";
    }
  ' "$file"
}

for rel in "${SOL_FILES[@]}"; do
  importer_abs="$(to_abs "$PROJECT_DIR/$rel")"
  [[ -z "$importer_abs" ]] && continue
  while IFS= read -r spec; do
    [[ -z "$spec" ]] && continue
    resolved="$(resolve_import_spec "$importer_abs" "$spec")"
    [[ -z "$resolved" ]] && continue
    if [[ "$resolved" == "$PROJECT_DIR/"* ]]; then
      printf "%s\t%s\n" "$resolved" "$importer_abs" >> "$EDGES_TSV"
    fi
  done < <(extract_import_specs "$PROJECT_DIR/$rel")
done

cp "$EDGES_TSV" /tmp/solidity-lsp-affected-last-edges.tsv

AFFECTED_ABS="$TMP_DIR/affected_abs.txt"
touch "$AFFECTED_ABS"

for changed in "$@"; do
  changed_abs="$(to_abs "$changed")"
  [[ -z "$changed_abs" ]] && continue
  if [[ "$changed_abs" == "$PROJECT_DIR/"* ]]; then
    echo "$changed_abs" >> "$AFFECTED_ABS"
  fi
done

sort -u "$AFFECTED_ABS" -o "$AFFECTED_ABS"

while true; do
  PREV="$TMP_DIR/affected_prev.txt"
  NEXT="$TMP_DIR/affected_next.txt"
  cp "$AFFECTED_ABS" "$PREV"
  cp "$AFFECTED_ABS" "$NEXT"

  while IFS= read -r current; do
    [[ -z "$current" ]] && continue
    awk -F'\t' -v key="$current" '$1==key {print $2}' "$EDGES_TSV" >> "$NEXT"
  done < "$PREV"

  sort -u "$NEXT" -o "$NEXT"
  if cmp -s "$PREV" "$NEXT"; then
    mv "$NEXT" "$AFFECTED_ABS"
    break
  fi
  mv "$NEXT" "$AFFECTED_ABS"
done

{
  echo "# affected files (reverse import closure)"
  echo "# project: $PROJECT_DIR"
  echo "# changed: $*"
  echo
  while IFS= read -r abs; do
    [[ -z "$abs" ]] && continue
    rel="${abs#"$PROJECT_DIR"/}"
    echo "$rel"
  done < "$AFFECTED_ABS"
} | tee "$TMP_DIR/affected.txt"

jq -n \
  --arg project "$PROJECT_DIR" \
  --arg changed "$*" \
  --argjson files "$(cat "$AFFECTED_ABS" | jq -R -s --arg p "$PROJECT_DIR/" 'split("\n") | map(select(length>0)) | map(sub("^" + $p; ""))')" \
  '{project: $project, changed: $changed, affected: $files, count: ($files|length)}'
