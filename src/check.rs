extern crate tempdir;
extern crate tempfile;

use tempdir::TempDir;
use tempfile::tempfile;

/// Checks to perform before starting copy of each file.
/// We make sure the source still exists (wasn't moved or deleted after initial scan).
/// Also we make sure the destination directory/ies get created before the copy.
pub fn check_before_copy() -> bool {
    if source_exists() {
        // Source exists, let's continue.
        if destination_directories_exist() {
            return true;
        } else {
            // Destination directories don't exist, try to create.
            create_destination_directory();
            if !destination_directories_exist() {
                // If they still don't exist, this indicates problems, skip.
                return false;
            }
        }
    }

    // Checks failed, skip the file.
    false
}

fn source_exists() -> bool {
    true
}

fn destination_directories_exist() -> bool {
    true
}

/// Creates full directory path for destination file.
fn create_destination_directory() {

}