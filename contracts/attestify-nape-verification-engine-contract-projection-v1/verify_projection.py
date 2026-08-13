#!/usr/bin/env python3
import hashlib
import json
from pathlib import Path

root = Path(__file__).resolve().parent
manifest = json.loads((root / "projection-manifest.json").read_text())
pin = json.loads((root / "projection-pin.json").read_text())
records = []
for item in manifest["files"]:
    raw = (root / item["path"]).read_bytes()
    actual = {"path": item["path"], "bytes": len(raw), "sha256": "sha256:" + hashlib.sha256(raw).hexdigest()}
    if actual != {key: item[key] for key in ("path", "bytes", "sha256")}:
        raise SystemExit(f"projection identity mismatch: {item['path']}")
    records.append(actual)
digest = hashlib.sha256()
for item in records:
    digest.update(item["path"].encode())
    digest.update(b"\0")
    digest.update(item["bytes"].to_bytes(8, "big"))
    digest.update(bytes.fromhex(item["sha256"].removeprefix("sha256:")))
actual = "sha256:" + digest.hexdigest()
if pin != {"contract": manifest["contract"], "aggregateIdentity": actual}:
    raise SystemExit("projection aggregate identity mismatch")
print(f"verified {len(records)} read-only I5 Engine projection files at {actual}")
