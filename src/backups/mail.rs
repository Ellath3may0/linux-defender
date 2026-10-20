//! Backup of the mail stack (Postfix and Dovecot).
//!
//! Not yet implemented. The paths this would cover are currently swept up by
//! [`super::all`], which backs up every known config path in one pass.

use super::paths::MAIL_PATHS;
use crate::{
    backups::all::run_with_paths,
    core::{DefenderContext, Result},
};
use std::path::PathBuf;

/// Back up every known config path in [`super::paths::MAIL_PATHS`].
pub fn run(ctx: &DefenderContext, save_location: &Option<PathBuf>) -> Result {
    run_with_paths(ctx, save_location, MAIL_PATHS)
}
