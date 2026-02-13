#![allow(dead_code)]
use std::ffi::OsStr;
use std::fs;
use std::io::ErrorKind;
use std::path::{Component, Path, PathBuf};
use std::sync::OnceLock;

use napi::bindgen_prelude::Either;
use napi_derive::napi;

type InputArg = Either<String, Vec<String>>;
type BoolOutput = Either<bool, Vec<bool>>;
type ExistsValue = Either<String, bool>;
type ExistsOutput = Either<ExistsValue, Vec<ExistsValue>>;
type ReadValue = Either<String, Either<Vec<String>, bool>>;
type ReadOutput = Either<ReadValue, Vec<ReadValue>>;

const DEFAULT_ENCODING: &str = "utf8";
static SANDBOX_ROOT: OnceLock<Result<PathBuf, String>> = OnceLock::new();

enum MetadataState {
    Missing,
    Present(fs::Metadata),
}

#[inline(always)]
fn invalid_input(message: &str) -> napi::Error {
    napi::Error::from_reason(message.to_string())
}

#[inline(always)]
fn io_error(context: &str, error: std::io::Error) -> napi::Error {
    napi::Error::from_reason(format!("{context}: {error}"))
}

#[inline(always)]
fn collapse<T>(mut values: Vec<T>) -> Either<T, Vec<T>> {
    if values.len() == 1 {
        return Either::A(values.remove(0));
    }
    Either::B(values)
}

#[inline(always)]
fn flatten_inputs(inputs: Vec<InputArg>) -> napi::Result<Vec<String>> {
    if inputs.is_empty() {
        return Err(invalid_input("At least one path is required"));
    }

    let mut output = Vec::new();
    for input in inputs {
        match input {
            Either::A(path) => output.push(path),
            Either::B(paths) => output.extend(paths),
        }
    }

    if output.is_empty() {
        return Err(invalid_input("At least one path is required"));
    }
    Ok(output)
}

#[inline(always)]
fn map_paths_to_output<T, F>(paths: Vec<String>, mut op: F) -> napi::Result<Either<T, Vec<T>>>
where
    F: FnMut(&str) -> napi::Result<T>,
{
    let mut output = Vec::with_capacity(paths.len());
    for path in paths {
        output.push(op(&path)?);
    }
    Ok(collapse(output))
}

#[inline(always)]
fn map_pairs_to_output<T, F>(
    pairs: Vec<(String, String)>,
    mut op: F,
) -> napi::Result<Either<T, Vec<T>>>
where
    F: FnMut(&str, &str) -> napi::Result<T>,
{
    let mut output = Vec::with_capacity(pairs.len());
    for (path, value) in pairs {
        output.push(op(&path, &value)?);
    }
    Ok(collapse(output))
}

#[inline(always)]
fn sandbox_root() -> napi::Result<PathBuf> {
    let cached = SANDBOX_ROOT.get_or_init(|| {
        let current = std::env::current_dir()
            .map_err(|error| format!("Failed to get cwd: {error}"))?;
        current
            .canonicalize()
            .map_err(|error| format!("Failed to canonicalize cwd: {error}"))
    });

    match cached {
        Ok(root) => Ok(root.clone()),
        Err(message) => Err(invalid_input(message)),
    }
}

#[inline(always)]
fn ensure_inside_root(root: &Path, path: &Path) -> napi::Result<()> {
    if path.starts_with(root) {
        return Ok(());
    }
    Err(invalid_input("Path resolves outside the repository root"))
}

#[inline(always)]
fn validate_existing_segment(root: &Path, path: &Path) -> napi::Result<()> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(io_error("Failed to inspect path segment", error)),
    };
    if !metadata.file_type().is_symlink() {
        return Ok(());
    }

    let canonical = path
        .canonicalize()
        .map_err(|error| io_error("Failed to canonicalize symlink path segment", error))?;
    ensure_inside_root(root, &canonical)
}

#[inline(always)]
fn resolve_normal_component(
    root: &Path,
    resolved: &mut PathBuf,
    segment: &OsStr,
) -> napi::Result<()> {
    resolved.push(segment);
    validate_existing_segment(root, resolved)
}

#[inline(always)]
fn resolve_path(root: &Path, raw_path: &str) -> napi::Result<PathBuf> {
    if raw_path.trim().is_empty() {
        return Err(invalid_input("Path must not be empty"));
    }

    let requested = Path::new(raw_path);
    if requested.is_absolute() {
        return Err(invalid_input("Absolute paths are not allowed"));
    }

    let mut resolved = root.to_path_buf();
    for component in requested.components() {
        match component {
            Component::CurDir => continue,
            Component::Normal(segment) => resolve_normal_component(root, &mut resolved, segment)?,
            _ => return Err(invalid_input("Path traversal is not allowed")),
        }
    }

    ensure_inside_root(root, &resolved)?;
    Ok(resolved)
}

#[inline(always)]
fn is_utf8_encoding(value: &str) -> bool {
    let normalized = value.to_ascii_lowercase();
    normalized == "utf8" || normalized == "utf-8"
}

#[inline(always)]
fn path_looks_like_file(path: &str) -> bool {
    Path::new(path).extension().is_some()
}

#[inline(always)]
fn take_single_path(paths: Vec<String>, message: &str) -> napi::Result<String> {
    if paths.len() != 1 {
        return Err(invalid_input(message));
    }

    paths
        .into_iter()
        .next()
        .ok_or_else(|| invalid_input("At least one path is required"))
}

enum CreatePlan {
    Folders(Vec<String>),
    Files(Vec<(String, String)>),
}

#[inline(always)]
fn build_create_plan(
    paths: Vec<String>,
    content: Option<Either<String, Vec<String>>>,
) -> napi::Result<CreatePlan> {
    let Some(payload) = content else {
        return Ok(CreatePlan::Folders(paths));
    };

    match payload {
        Either::B(values) => build_create_plan_from_arrays(paths, values),
        Either::A(value) => build_create_plan_from_string(paths, value),
    }
}

#[inline(always)]
fn build_create_plan_from_arrays(
    paths: Vec<String>,
    values: Vec<String>,
) -> napi::Result<CreatePlan> {
    if values.is_empty() {
        return Err(invalid_input("Second argument array must not be empty"));
    }
    if paths.len() == 1 {
        let mut folders = paths;
        folders.extend(values);
        return Ok(CreatePlan::Folders(folders));
    }
    if paths.len() != values.len() {
        return Err(invalid_input("Path and content arrays must have equal length"));
    }

    let files = paths.into_iter().zip(values).collect();
    Ok(CreatePlan::Files(files))
}

#[inline(always)]
fn build_create_plan_from_string(paths: Vec<String>, value: String) -> napi::Result<CreatePlan> {
    let first = take_single_path(paths, "Single content string only works with a single path")?;
    if path_looks_like_file(&first) {
        return Ok(CreatePlan::Files(vec![(first, value)]));
    }
    Ok(CreatePlan::Folders(vec![first, value]))
}

#[inline(always)]
fn metadata_state(path: &Path, context: &str) -> napi::Result<MetadataState> {
    match fs::metadata(path) {
        Ok(metadata) => Ok(MetadataState::Present(metadata)),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(MetadataState::Missing),
        Err(error) => Err(io_error(context, error)),
    }
}

#[inline(always)]
fn create_folder(root: &Path, raw_path: &str) -> napi::Result<bool> {
    let target = resolve_path(root, raw_path)?;
    if target == root {
        return Err(invalid_input("Refusing to create the repository root"));
    }

    fs::create_dir_all(&target).map_err(|error| io_error("Failed to create directory", error))?;
    Ok(true)
}

#[inline(always)]
fn create_file(root: &Path, raw_path: &str, content: &str) -> napi::Result<bool> {
    let target = resolve_path(root, raw_path)?;
    if target == root {
        return Err(invalid_input("Refusing to overwrite the repository root"));
    }

    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| io_error("Failed to create file parent directory", error))?;
    }

    fs::write(&target, content).map_err(|error| io_error("Failed to write file", error))?;
    Ok(true)
}

#[inline(always)]
fn create_folders(root: &Path, paths: Vec<String>) -> napi::Result<BoolOutput> {
    map_paths_to_output(paths, |path| create_folder(root, path))
}

#[inline(always)]
fn create_files(root: &Path, files: Vec<(String, String)>) -> napi::Result<BoolOutput> {
    map_pairs_to_output(files, |path, content| create_file(root, path, content))
}

#[inline(always)]
fn exists_for_path(root: &Path, raw_path: &str) -> napi::Result<ExistsValue> {
    let target = resolve_path(root, raw_path)?;
    let state = metadata_state(&target, "Failed to inspect path")?;
    let MetadataState::Present(metadata) = state else {
        return Ok(Either::B(false));
    };

    if metadata.is_dir() {
        return Ok(Either::A("folder".to_string()));
    }
    if metadata.is_file() {
        return Ok(Either::A("file".to_string()));
    }

    Ok(Either::B(false))
}

#[inline(always)]
fn list_directory(path: &Path) -> napi::Result<Vec<String>> {
    let mut output = Vec::new();
    let entries = fs::read_dir(path).map_err(|error| io_error("Failed to read directory", error))?;
    for entry in entries {
        let entry = entry.map_err(|error| io_error("Failed to iterate directory", error))?;
        output.push(entry.file_name().to_string_lossy().into_owned());
    }

    output.sort_unstable();
    Ok(output)
}

#[inline(always)]
fn read_for_path(root: &Path, raw_path: &str, encoding: &str) -> napi::Result<ReadValue> {
    if !is_utf8_encoding(encoding) {
        return Err(invalid_input("Only UTF-8 encoding is supported"));
    }

    let target = resolve_path(root, raw_path)?;
    let state = metadata_state(&target, "Failed to inspect path")?;
    let MetadataState::Present(metadata) = state else {
        return Ok(Either::B(Either::B(false)));
    };

    if metadata.is_file() {
        let contents =
            fs::read_to_string(&target).map_err(|error| io_error("Failed to read file", error))?;
        return Ok(Either::A(contents));
    }
    if metadata.is_dir() {
        let entries = list_directory(&target)?;
        return Ok(Either::B(Either::A(entries)));
    }

    Ok(Either::B(Either::B(false)))
}

#[inline(always)]
fn remove_for_path(root: &Path, raw_path: &str) -> napi::Result<bool> {
    let target = resolve_path(root, raw_path)?;
    if target == root {
        return Err(invalid_input("Refusing to remove the repository root"));
    }

    let state = metadata_state(&target, "Failed to inspect path")?;
    let MetadataState::Present(metadata) = state else {
        return Ok(false);
    };

    if metadata.is_file() {
        fs::remove_file(&target).map_err(|error| io_error("Failed to remove file", error))?;
        return Ok(true);
    }
    if metadata.is_dir() {
        fs::remove_dir_all(&target)
            .map_err(|error| io_error("Failed to remove directory", error))?;
        return Ok(true);
    }

    Ok(false)
}

#[napi(namespace = "file")]
pub fn create(
    inputs: Vec<InputArg>,
    content: Option<Either<String, Vec<String>>>,
) -> napi::Result<BoolOutput> {
    let root = sandbox_root()?;
    let paths = flatten_inputs(inputs)?;
    let plan = build_create_plan(paths, content)?;

    match plan {
        CreatePlan::Folders(paths) => create_folders(&root, paths),
        CreatePlan::Files(files) => create_files(&root, files),
    }
}

#[napi(namespace = "file")]
pub fn exists(inputs: Vec<InputArg>) -> napi::Result<ExistsOutput> {
    let root = sandbox_root()?;
    let paths = flatten_inputs(inputs)?;
    map_paths_to_output(paths, |path| exists_for_path(&root, path))
}

#[napi(namespace = "file")]
pub fn read(inputs: Vec<InputArg>, encoding: Option<String>) -> napi::Result<ReadOutput> {
    let root = sandbox_root()?;
    let paths = flatten_inputs(inputs)?;
    let encoding = encoding.unwrap_or_else(|| DEFAULT_ENCODING.to_string());
    map_paths_to_output(paths, |path| read_for_path(&root, path, &encoding))
}

#[napi(namespace = "file")]
pub fn remove(inputs: Vec<InputArg>) -> napi::Result<BoolOutput> {
    let root = sandbox_root()?;
    let paths = flatten_inputs(inputs)?;
    map_paths_to_output(paths, |path| remove_for_path(&root, path))
}
