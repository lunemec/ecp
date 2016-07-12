extern crate tempdir;
extern crate tempfile;

//use tempdir::TempDir;
//use tempfile::tempfile;

use std::path::Path;

/// Checks to perform before starting copy of each file.
/// We make sure the source still exists (wasn't moved or deleted after initial scan).
/// Also we make sure the destination directory/ies get created before the copy.
pub fn check_before_copy(src: &Path) -> bool {
    if source_exists(src) {
        // Source exists, let's continue.
        if destination_directories_exist(src) {
            return true;
        } else {
            // Destination directories don't exist, try to create.
            create_destination_directory(src);
            if !destination_directories_exist(src) {
                // If they still don't exist, this indicates problems, skip.
                return false;
            } else {
                return true;
            }
        }
    }

    // Checks failed, skip the file.
    false
}

fn source_exists(src: &Path) -> bool {
    src.exists()
}

fn destination_directories_exist(src: &Path) -> bool {
    true
}

/// Creates full directory path for destination file.
fn create_destination_directory(src: &Path) {

}