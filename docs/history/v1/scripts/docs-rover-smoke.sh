#!/usr/bin/env bash
set -euo pipefail

WORK="${NAPE_DOCS_SMOKE_WORK:-/private/tmp/nape-docs-rover-smoke}"
KEEP="${NAPE_DOCS_SMOKE_KEEP:-0}"
NAPE_BIN="${NAPE_BIN:-nape}"
NAPE_EVAL_BIN="${NAPE_EVAL_BIN:-nape-eval}"

SYSTEM_REPO="${NAPE_DOCS_SMOKE_SYSTEM_REPO:-https://github.com/attestify/rover-medical-system.git}"
CATALOG_REPO="${NAPE_DOCS_SMOKE_CATALOG_REPO:-https://github.com/attestify/rover-medical-catalog.git}"
EVIDENCE_REPO="${NAPE_DOCS_SMOKE_EVIDENCE_REPO:-https://github.com/attestify/rover-medical-evidence-collection.git}"

SYSTEM_REF="${NAPE_DOCS_SMOKE_SYSTEM_REF:-db6702721a5d0d2a56591989647139784829fcc2}"
CATALOG_REF="${NAPE_DOCS_SMOKE_CATALOG_REF:-ba6413314e9ca02198cc9216f60bbc65a58f80c9}"
EVIDENCE_REF="${NAPE_DOCS_SMOKE_EVIDENCE_REF:-87be7e5b97dfb78d80be08f99f3401cd365aae5b}"

RUN_DIR_NAME="nrn_procedure_rover-medical_-_rover-medicine-system"

log() {
  printf '[docs-rover-smoke] %s\n' "$*"
}

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf 'Missing required command: %s\n' "$1" >&2
    exit 1
  fi
}

validate_work_dir() {
  if [[ -z "$WORK" || "$WORK" == "/" || "$WORK" == "." ]]; then
    printf 'Unsafe NAPE_DOCS_SMOKE_WORK value: %s\n' "$WORK" >&2
    exit 1
  fi

  case "$WORK" in
    /tmp/*|/private/tmp/*)
      ;;
    *)
      printf 'NAPE_DOCS_SMOKE_WORK must be under /tmp or /private/tmp. Got: %s\n' "$WORK" >&2
      exit 1
      ;;
  esac
}

clone_repo() {
  local repo_url="$1"
  local ref="$2"
  local target="$3"

  git clone --quiet "$repo_url" "$target"
  git -C "$target" checkout --quiet "$ref"
}

summary_value() {
  local report="$1"
  local key="$2"

  awk -v key="$key" '
    /^summary:/ { in_summary = 1; next }
    in_summary && /^[^[:space:]]/ { in_summary = 0 }
    in_summary {
      line = $0
      sub(/^[[:space:]]+/, "", line)
      split(line, parts, ":")
      if (parts[1] == key) {
        value = substr(line, length(parts[1]) + 2)
        sub(/^[[:space:]]+/, "", value)
        sub(/[[:space:]]+$/, "", value)
        print value
        exit
      }
    }
  ' "$report"
}

assert_summary() {
  local report="$1"
  local activity_count="$2"
  local action_count="$3"
  local actions_run="$4"
  local pass_count="$5"
  local fail_count="$6"
  local inconclusive_count="$7"
  local outcome="$8"

  assert_summary_field "$report" activity_count "$activity_count"
  assert_summary_field "$report" action_count "$action_count"
  assert_summary_field "$report" actions_run "$actions_run"
  assert_summary_field "$report" pass "$pass_count"
  assert_summary_field "$report" fail "$fail_count"
  assert_summary_field "$report" inconclusive "$inconclusive_count"
  assert_summary_field "$report" outcome "$outcome"
  assert_report_shape "$report"
}

assert_summary_field() {
  local report="$1"
  local key="$2"
  local expected="$3"
  local actual

  actual="$(summary_value "$report" "$key")"
  if [[ "$actual" != "$expected" ]]; then
    printf 'Unexpected %s in %s: expected [%s], got [%s]\n' "$key" "$report" "$expected" "$actual" >&2
    exit 1
  fi
}

assert_report_shape() {
  local report="$1"

  if ! grep -q '^apiVersion: 1.0.0$' "$report"; then
    printf 'Report %s does not declare apiVersion: 1.0.0\n' "$report" >&2
    exit 1
  fi

  if ! grep -q '^kind: AssuranceReport$' "$report"; then
    printf 'Report %s does not declare kind: AssuranceReport\n' "$report" >&2
    exit 1
  fi

  if ! grep -q 'signature: SHA256\[[0-9a-f]\{64\}\]' "$report"; then
    printf 'Report %s does not contain a SHA256 signature in the expected shape\n' "$report" >&2
    exit 1
  fi
}

find_report() {
  local run_root="$1"
  local report

  report="$(find "$run_root/$RUN_DIR_NAME" -name assurance_report.yaml -print | sort | tail -n 1)"
  if [[ -z "$report" ]]; then
    printf 'No assurance_report.yaml found under %s\n' "$run_root/$RUN_DIR_NAME" >&2
    exit 1
  fi
  printf '%s\n' "$report"
}

run_release_1() {
  local run_root="$WORK/rover-medical-system-release-1"
  local report

  log "running Rover Release 1 smoke"
  cp -R "$WORK/rover-medical-system" "$run_root"
  cd "$run_root"

  "$NAPE_BIN" collect start \
    --subject "nrn:procedure:rover-medical/rover-medicine-system" \
    --subject-id "localrelease1" \
    --procedure-link "file://$WORK/rover-medical-catalog" \
    --procedure-directory "rover-mediical-system/release-1" \
    --meta system-owner "Docs Smoke"

  "$NAPE_BIN" collect evidence \
    --control-activity "pet-medicine-app" \
    --file-path "pet-medicine-app/app-config.toml"

  "$NAPE_BIN" collect report

  report="$(find_report "$run_root")"
  assert_summary "$report" 1 1 1 0 0 1 inconclusive
  log "Release 1 summary matched expected values: $report"
}

run_release_3() {
  local run_root="$WORK/rover-medical-system-release-3"
  local evidence_root="$WORK/rover-medical-evidence-collection/rover-medical-system/release-3"
  local report

  log "running Rover Release 3 smoke"
  cp -R "$WORK/rover-medical-system" "$run_root"
  cp -R "$evidence_root/pet-medicine-db" "$run_root/"
  cp -R "$evidence_root/pet-medicine-host" "$run_root/"
  cp -R "$evidence_root/rover-cloud" "$run_root/"
  cp -R "$evidence_root/exa-doo-dc" "$run_root/"
  cd "$run_root"

  "$NAPE_BIN" collect start \
    --subject "nrn:procedure:rover-medical/rover-medicine-system" \
    --subject-id "localrelease3" \
    --procedure-link "file://$WORK/rover-medical-catalog" \
    --procedure-directory "rover-mediical-system/release-3" \
    --meta system-owner "Docs Smoke"

  "$NAPE_BIN" collect evidence --control-activity "pet-medicine-app" --file-path "pet-medicine-app/app-config.toml"
  "$NAPE_BIN" collect evidence --control-activity "pet-medicine-db" --file-path "pet-medicine-db/my-db.ini"
  "$NAPE_BIN" collect evidence --control-activity "pet-medicine-host" --file-path "pet-medicine-host/stdout-user-exists-app-user.txt"
  "$NAPE_BIN" collect evidence --control-activity "pet-medicine-host" --file-path "pet-medicine-host/stdout-user-exists-db-user.txt"
  "$NAPE_BIN" collect evidence --control-activity "pet-medicine-host" --file-path "pet-medicine-host/stdout-no-login-db-user.txt"
  "$NAPE_BIN" collect evidence --control-activity "pet-medicine-host" --file-path "pet-medicine-host/stdout-no-login-app-user.txt"
  "$NAPE_BIN" collect evidence --control-activity "pet-medicine-host" --file-path "pet-medicine-host/stdout-root-check-admin-user.txt"
  "$NAPE_BIN" collect evidence --control-activity "pet-medicine-host" --file-path "pet-medicine-host/stdout-root-check-app-user.txt"
  "$NAPE_BIN" collect evidence --control-activity "pet-medicine-host" --file-path "pet-medicine-host/stdout-root-check-db-user.txt"
  "$NAPE_BIN" collect evidence --control-activity "pet-medicine-host" --file-path "pet-medicine-host/stdout-auditctl-installed.txt"
  "$NAPE_BIN" collect evidence --control-activity "pet-medicine-host" --file-path "pet-medicine-host/stdout-auditctl-user-commands-logged.txt"
  "$NAPE_BIN" collect evidence --control-activity "rover-cloud" --file-path "rover-cloud/cloud-config.yaml"
  "$NAPE_BIN" collect evidence --control-activity "exa-doo-dc" --file-path "exa-doo-dc/soc1-report-analysis.json"

  "$NAPE_BIN" collect report

  report="$(find_report "$run_root")"
  assert_summary "$report" 5 22 22 18 0 4 inconclusive
  log "Release 3 summary matched expected values: $report"
}

cleanup() {
  if [[ "$KEEP" != "1" ]]; then
    rm -rf "$WORK"
  else
    log "keeping workspace: $WORK"
  fi
}

main() {
  validate_work_dir
  require_cmd git
  require_cmd "$NAPE_BIN"
  require_cmd "$NAPE_EVAL_BIN"

  trap cleanup EXIT
  rm -rf "$WORK"
  mkdir -p "$WORK/home"
  export HOME="$WORK/home"

  log "cloning Rover Medical repositories into $WORK"
  clone_repo "$SYSTEM_REPO" "$SYSTEM_REF" "$WORK/rover-medical-system"
  clone_repo "$CATALOG_REPO" "$CATALOG_REF" "$WORK/rover-medical-catalog"
  clone_repo "$EVIDENCE_REPO" "$EVIDENCE_REF" "$WORK/rover-medical-evidence-collection"

  "$NAPE_EVAL_BIN" --check-install >/dev/null

  run_release_1
  run_release_3

  log "Rover docs smoke completed"
}

main "$@"
