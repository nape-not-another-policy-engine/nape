//! Strict Command Receipt V2 stdout adapter.

#[cfg(test)]
mod tests;

use std::io::Write;

use kernel_oss::error::{Error, Kind};
use nape_domain::diagnostic::NapeDiagnostic;
use serde_json::{json, Value};

const CONTRACT: &str = "attestify.nape.command-receipt/v2";
const QUALIFICATION: &str = "attestify-verification-engine-v2-development";
const MAX_BYTES: usize = 65_536;

/// Fixed producer projection selected once by the composition root.
#[derive(Clone)]
pub struct ReceiptAdapter {
    producer: Value,
}

impl ReceiptAdapter {
    /// Creates the adapter for one exact running NAPE build.
    pub fn new(build_digest: impl Into<String>) -> Self {
        Self {
            producer: json!({
                "buildDigest": build_digest.into(),
                "name": "nape",
                "qualification": QUALIFICATION,
                "version": env!("CARGO_PKG_VERSION"),
            }),
        }
    }

    /// Emits one successful command Receipt and returns exit zero.
    pub fn success(&self, command: &str, result: Value) -> i32 {
        self.emit(
            json!({
                "command": command,
                "contract": CONTRACT,
                "producer": self.producer,
                "result": result,
                "status": "succeeded",
            }),
            0,
        )
    }

    /// Emits one governed failed command Receipt and returns exit one.
    pub fn failed(
        &self,
        command: &str,
        diagnostic: &NapeDiagnostic,
        invocation: Option<(&str, &str)>,
    ) -> i32 {
        let mut receipt = json!({
            "command": command,
            "contract": CONTRACT,
            "diagnostic": {
                "code": diagnostic.code(),
                "phase": diagnostic.phase(),
                "reason_template": diagnostic.reason_template(),
            },
            "producer": self.producer,
            "status": "failed",
        });
        if let Some((invocation_id, source)) = invocation {
            let object = receipt.as_object_mut().expect("Receipt is an object");
            object.insert("invocationId".to_string(), json!(invocation_id));
            object.insert("source".to_string(), json!(source));
        }
        self.emit(receipt, 1)
    }

    /// Reports an unexpected post-dispatch failure without fabricating a
    /// governed diagnostic mapping.
    pub fn unexpected(&self, error: &Error) -> i32 {
        eprintln!("NAPE internal failure: {}", error.message());
        1
    }

    fn emit(&self, value: Value, exit: i32) -> i32 {
        let mut bytes = match canonical_json(&value) {
            Ok(value) => value.into_bytes(),
            Err(error) => return self.unframed(error),
        };
        bytes.push(b'\n');
        if bytes.len() > MAX_BYTES {
            return self.unframed(Error::for_system(
                Kind::ExceedsMax,
                "Command Receipt exceeds 64 KiB",
            ));
        }
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();
        if lock.write_all(&bytes).and_then(|()| lock.flush()).is_err() {
            eprintln!("NAPE Command Receipt delivery failed after command completion");
            return 1;
        }
        exit
    }

    fn unframed(&self, error: Error) -> i32 {
        eprintln!("NAPE Command Receipt failure: {}", error.message());
        1
    }
}

/// Serializes the strict JCS subset used by Command Receipt V2.
pub fn canonical_json(value: &Value) -> Result<String, Error> {
    fn encode(value: &Value, output: &mut String) -> Result<(), Error> {
        match value {
            Value::Null => output.push_str("null"),
            Value::Bool(true) => output.push_str("true"),
            Value::Bool(false) => output.push_str("false"),
            Value::Number(number) => output.push_str(&number.to_string()),
            Value::String(text) => output.push_str(&serde_json::to_string(text).map_err(|_| {
                Error::for_system(Kind::ProcessingFailure, "Receipt string cannot be encoded")
            })?),
            Value::Array(values) => {
                output.push('[');
                for (index, item) in values.iter().enumerate() {
                    if index > 0 {
                        output.push(',');
                    }
                    encode(item, output)?;
                }
                output.push(']');
            }
            Value::Object(values) => {
                let mut entries = values.iter().collect::<Vec<_>>();
                entries.sort_by_key(|(key, _)| key.encode_utf16().collect::<Vec<_>>());
                output.push('{');
                for (index, (key, item)) in entries.into_iter().enumerate() {
                    if index > 0 {
                        output.push(',');
                    }
                    output.push_str(&serde_json::to_string(key).map_err(|_| {
                        Error::for_system(Kind::ProcessingFailure, "Receipt key cannot be encoded")
                    })?);
                    output.push(':');
                    encode(item, output)?;
                }
                output.push('}');
            }
        }
        Ok(())
    }
    let mut output = String::new();
    encode(value, &mut output)?;
    Ok(output)
}
