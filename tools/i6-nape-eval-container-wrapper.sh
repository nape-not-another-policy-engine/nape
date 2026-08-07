#!/bin/sh
set -eu

: "${ATTESTIFY_WORKSPACE_ROOT:?ATTESTIFY_WORKSPACE_ROOT is required}"

exec docker run --rm -i \
  --network none \
  --read-only \
  --tmpfs /tmp:rw,nosuid,nodev,noexec,size=64m \
  -e PYTHONDONTWRITEBYTECODE=1 \
  -e PYTHONPATH=/workspace/applications/nape-evaluator/src \
  -v "${ATTESTIFY_WORKSPACE_ROOT}:/workspace:ro" \
  -v /private/tmp:/private/tmp:rw \
  python:3.11.6-slim-bookworm \
  python3 /workspace/applications/nape-evaluator/main.py "$@"
