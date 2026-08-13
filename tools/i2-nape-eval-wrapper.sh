#!/bin/sh
export PYTHONPATH=/workspace/applications/nape-evaluator/src
exec python3 /workspace/applications/nape-evaluator/main.py "$@"
