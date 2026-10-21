//! Backup of the user, group and shadow databases.
//!
//! Not yet implemented. The paths this would cover are currently swept up by
//! [`super::all`], which backs up every known config path in one pass.

use crate::{
    backups::{
        all::run_with_paths,
        paths::USERS,
    },
    core::{DefenderContext, Result},
};
use std::path::PathBuf;

/// Placeholder until this backup is implemented.
pub fn run(_ctx: &DefenderContext, _save_location: &Option<PathBuf>) -> Result {
    run_with_paths(_ctx, _save_location, USERS)
}
