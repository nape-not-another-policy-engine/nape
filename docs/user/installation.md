# NAPE CLI 2.0 Installation and Evaluator Preflight

Build NAPE:

```bash
cargo build --release --workspace
```

Install NAPE Evaluator from its sibling repository according to its current
installation guide. NAPE selects an exact installed evaluator using an
absolute executable, an absolute installed-build record, and the expected
SHA-256 of that record.

Minimal/Evaluator V2 definitions use:

```bash
export NAPE_EVALUATOR_V2_EXECUTABLE=/absolute/path/to/nape-eval
export NAPE_EVALUATOR_V2_BUILD_RECORD=/absolute/path/to/evaluator-build.json
export NAPE_EVALUATOR_V2_BUILD_RECORD_SHA256=sha256:<64-lowercase-hex>
```

Functional/Evaluator V3 definitions use:

```bash
export NAPE_EVALUATOR_V3_EXECUTABLE=/absolute/path/to/nape-eval-v3
export NAPE_EVALUATOR_V3_BUILD_RECORD=/absolute/path/to/evaluator-v3-build.json
export NAPE_EVALUATOR_V3_BUILD_RECORD_SHA256=sha256:<64-lowercase-hex>
```

NAPE selects V2 or V3 from the admitted Procedure closure; users do not choose
an evaluator contract flag. Both internal contracts preserve the Test ABI:

```python
evaluate(evidence, evaluations, metadata)
```

Verify the command surface:

```bash
nape --version
nape --help
nape package --help
nape verify --help
```

The help output must not expose `collect`.
