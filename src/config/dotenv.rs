use std::env;
use std::fs;
use std::io;
use std::path::Path;

/// Reads `KEY=VALUE` pairs from `path` (if it exists) and inserts them into
/// the process environment. Variables that are ALREADY set in the real
/// environment win, so `GEMINI_API_KEY=… cargo run` always overrides the
/// file.
///
/// Supports:
///   - blank lines and `# comments`
///   - `KEY=value`, `KEY="quoted"`, `KEY='quoted'`
///   - leading `export KEY=value` (so the same file can be `source`d)
///
/// Returns the number of keys it set. A missing file is NOT an error —
/// the loader is optional by design.
pub fn load_dotenv<P: AsRef<Path>>(path: P) -> io::Result<usize> {
    let contents = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(0),
        Err(e) => return Err(e),
    };

    let mut loaded = 0;
    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let line = line.strip_prefix("export ").unwrap_or(line);

        let Some(eq) = line.find('=') else {
            continue;
        };
        if eq == 0 {
            continue;
        }

        let key = line[..eq].trim();
        let mut val = line[eq + 1..].trim();

        if val.len() >= 2 {
            let bytes = val.as_bytes();
            let (first, last) = (bytes[0], bytes[bytes.len() - 1]);
            if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
                val = &val[1..val.len() - 1];
            }
        }

        if env::var_os(key).is_some() {
            continue;
        }
        // SAFETY: process-startup config load, no other threads racing.
        env::set_var(key, val);
        loaded += 1;
    }
    Ok(loaded)
}
