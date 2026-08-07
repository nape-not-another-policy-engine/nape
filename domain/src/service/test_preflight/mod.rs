//! Deterministic Test-of-Detail preflight rules.

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};

use kernel_oss::error::{Error, Kind};

/// Effective immutable Test execution resource limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectiveTestResources {
    /// Maximum evaluator work units.
    pub maximum_execution_work_units: u64,
    /// Maximum evaluator response bytes.
    pub maximum_result_bytes: u64,
}

/// One declared helper module and its exact verified source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreflightTestModule {
    /// Exact declared import name.
    pub name: String,
    /// Exact verified Python source.
    pub source: Vec<u8>,
}

/// Admits one nonempty Python Test source and positive fixed limits.
pub fn preflight_test(
    test_source: &[u8],
    resources: EffectiveTestResources,
) -> Result<EffectiveTestResources, Error> {
    if test_source.is_empty() {
        return Err(Error::for_user(
            Kind::InvalidInput,
            "Test source must not be empty",
        ));
    }
    if resources.maximum_execution_work_units == 0 || resources.maximum_result_bytes == 0 {
        return Err(Error::for_user(
            Kind::BelowMin,
            "Test resource limits must be positive",
        ));
    }
    Ok(resources)
}

/// Proves the declared helper list is exactly the reachable, acyclic,
/// non-ambient direct-import closure of one Test of Detail.
pub fn preflight_module_closure(
    entry_source: &[u8],
    modules: &[PreflightTestModule],
) -> Result<(), Error> {
    let declared = modules
        .iter()
        .map(|module| module.name.clone())
        .collect::<BTreeSet<_>>();
    if declared.len() != modules.len() || declared.contains("json") {
        return Err(invalid(
            "helper modules must be unique and cannot shadow json",
        ));
    }
    let mut graph = BTreeMap::<Option<String>, BTreeSet<String>>::new();
    graph.insert(None, direct_source_imports(entry_source, &declared)?);
    for module in modules {
        graph.insert(
            Some(module.name.clone()),
            direct_source_imports(&module.source, &declared)?,
        );
    }
    let mut reachable = BTreeSet::new();
    let mut active = BTreeSet::new();
    walk(None, &graph, &mut active, &mut reachable)?;
    if reachable != declared {
        return Err(invalid(
            "declared helper modules are not the complete reachable closure",
        ));
    }
    Ok(())
}

fn walk(
    name: Option<&str>,
    graph: &BTreeMap<Option<String>, BTreeSet<String>>,
    active: &mut BTreeSet<String>,
    reachable: &mut BTreeSet<String>,
) -> Result<(), Error> {
    let imports = graph
        .get(&name.map(str::to_string))
        .ok_or_else(|| invalid("helper import is undeclared"))?;
    for dependency in imports {
        if !active.insert(dependency.clone()) {
            return Err(invalid("declared helper graph contains a cycle"));
        }
        reachable.insert(dependency.clone());
        walk(Some(dependency), graph, active, reachable)?;
        active.remove(dependency);
    }
    Ok(())
}

fn direct_source_imports(
    source: &[u8],
    declared: &BTreeSet<String>,
) -> Result<BTreeSet<String>, Error> {
    let source = std::str::from_utf8(source).map_err(|_| invalid("Test source is not UTF-8"))?;
    let mut imported = BTreeSet::new();
    for line in source.lines() {
        let trimmed = line.trim_start();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if trimmed.starts_with("from ") || trimmed.contains("__import__") {
            return Err(invalid(
                "only direct top-level import statements are admitted",
            ));
        }
        if let Some(import) = trimmed.strip_prefix("import ") {
            if line.len() != trimmed.len() {
                return Err(invalid(
                    "helper imports must be top-level and unconditional",
                ));
            }
            let target = import
                .split_once('#')
                .map_or(import.trim(), |(value, _)| value.trim());
            if target.is_empty()
                || target.contains(char::is_whitespace)
                || target.contains(',')
                || target.contains('.')
                || !target
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
            {
                return Err(invalid(
                    "helper import must name exactly one admitted module",
                ));
            }
            if target != "json" && !declared.contains(target) {
                return Err(invalid(
                    "Test source imports an ambient or undeclared module",
                ));
            }
            if target != "json" {
                imported.insert(target.to_string());
            }
        } else if trimmed.contains(';') && trimmed.contains("import") {
            return Err(invalid("compound import statements are not admitted"));
        }
    }
    Ok(imported)
}

fn invalid(detail: &str) -> Error {
    Error::for_user(Kind::InvalidInput, detail)
}
