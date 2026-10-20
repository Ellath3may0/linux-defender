//! Backup of the Splunk forwarder and indexer configuration.
//!
//! Not yet implemented. The paths this would cover are currently swept up by
//! [`super::all`], which backs up every known config path in one pass.

use super::paths::SPLUNK_PATHS;
use crate::{
    backups::all::run_with_paths,
    core::{DefenderContext, Result},
};
use std::path::PathBuf;

/// Back up every known config path in [`super::paths::SPLUNK_PATHS`].
pub fn run(_ctx: &DefenderContext, _save_location: &Option<PathBuf>) -> Result {
    run_with_paths(_ctx, _save_location, SPLUNK_PATHS)
}
