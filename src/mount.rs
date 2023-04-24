use std::io::Error;
use std::path::Path;
use sys_mount::*;

/// Mounts an overlayfs.
/// Returns a UnmountDrop Mount that will unmount once the value goes out of scope
/// # Arguments
/// * `lower` - The lower directory for the overlayfs
/// * `work` - The work directory for the overlayfs
/// * `upper` - The upper directory for the overlayfs
/// * `merged` - The merged directory for the overlayfs
pub fn mount_overlay(
    lower: &Path,
    work: &Path,
    upper: &Path,
    merged: &Path,
) -> Result<UnmountDrop<Mount>, Error> {
    std::fs::create_dir_all(lower)?;
    std::fs::create_dir_all(upper)?;
    std::fs::create_dir_all(work)?;
    std::fs::create_dir_all(merged)?;

    let lower_s = lower.to_string_lossy();
    let work_s = work.to_string_lossy();
    let upper_s = work.to_string_lossy();

    let mount_result = Mount::builder()
        .fstype("overlay")
        .data(format!("lowerdir={lower_s},workdir={work_s},upperdir={upper_s}").as_str())
        .mount("overlay", merged);

    match mount_result {
        Ok(mount) => Ok(mount.into_unmount_drop(UnmountFlags::DETACH)),
        Err(e) => Err(e),
    }
}
