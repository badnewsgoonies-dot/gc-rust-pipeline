pub struct GenEntry { pub generation: u64, pub snapshot_hash: String, pub parent_hash: Option<String> }

pub struct GenerationLog { entries: Vec<GenEntry> }

impl GenerationLog:
- pub fn new() -> Self with empty entries.
- pub fn head(&self) -> Option<&GenEntry> returning self.entries.last().
- pub fn next_generation(&self) -> u64 returning self.head().map(|e| e.generation + 1).unwrap_or(0).
- pub fn append(&mut self, snapshot_hash: String, parent_hash: Option<String>) -> Result<&GenEntry, String>:
  - If parent_hash differs from self.head().map(|h| &h.snapshot_hash), return Err with message "parent mismatch".
  - Otherwise push GenEntry{generation: self.next_generation(), snapshot_hash, parent_hash}, return Ok(self.entries.last().unwrap()).
- pub fn len(&self) -> usize.
- pub fn get(&self, generation: u64) -> Option<&GenEntry> linear scan.

impl Default for GenerationLog returning Self::new().

#[cfg(test)] mod tests with three tests:
- fn empty_log(): new log, head() is None, next_generation() == 0, len() == 0.
- fn append_chain(): new log, append("h0", None), append("h1", Some("h0".into())), append("h2", Some("h1".into())), assert len()==3, head().unwrap().generation == 2.
- fn parent_mismatch_rejected(): new log, append("h0", None), then append("h1", Some("WRONG".into())) returns Err containing "parent mismatch".

Std only. Syntactically valid Rust 2021.
