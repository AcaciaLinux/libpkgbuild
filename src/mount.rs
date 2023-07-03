use crate::StdIOErrorExt;
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
) -> Result<UnmountDrop<Mount>, std::io::Error> {
    std::fs::create_dir_all(lower).err_prepend("When ensuring lower directory")?;
    std::fs::create_dir_all(upper).err_prepend("When ensuring upper directory")?;
    std::fs::create_dir_all(work).err_prepend("When ensuring work directory")?;
    std::fs::create_dir_all(merged).err_prepend("When ensuring merged directory")?;

    let lower_s = lower.to_string_lossy();
    let work_s = work.to_string_lossy();
    let upper_s = upper.to_string_lossy();

    let data = format!("lowerdir={lower_s},workdir={work_s},upperdir={upper_s}");
    info!(
        "Mounting overlay ({}) -> {}",
        &data,
        &merged.to_string_lossy()
    );

    Mount::builder()
        .fstype("overlay")
        .data(&data)
        .mount_autodrop("overlay", merged, UnmountFlags::DETACH)
        .err_prepend("When mounting overlayfs")
}

/// Mounts the following Linux virtual kernel filesystems from `source` to `destination`:
/// `/dev`, `/dev/pts`, `/proc`, `/sys`, `/tmp`
/// # Arguments
/// * `source` - The source to take the mounts from (usually `/`)
/// * `destination` - The destination to mount the vkfs into
pub fn mount_vkfs(
    source: &Path,
    destination: &Path,
) -> Result<Vec<UnmountDrop<Mount>>, std::io::Error> {
    // Create quick handlers
    let src_dev = source.join("dev");

    let dest_dev = destination.join("dev");
    let dest_dev_pts = dest_dev.join("pts");
    let dest_proc = destination.join("proc");
    let dest_sys = destination.join("sys");
    let dest_tmp = destination.join("tmp");

    // Ensure the target directories exist
    std::fs::create_dir_all(&destination).err_prepend("When ensuring destination")?;

    std::fs::create_dir_all(&dest_dev).err_prepend("When ensuring /dev")?;
    std::fs::create_dir_all(&dest_proc).err_prepend("When ensuring /proc")?;
    std::fs::create_dir_all(&dest_sys).err_prepend("When ensuring /sys")?;
    std::fs::create_dir_all(&dest_tmp).err_prepend("When ensuring /tmp")?;

    let flags = UnmountFlags::FORCE;

    // /dev
    info!(
        "[vkfs] Mounting dev {} -> {}",
        &src_dev.to_string_lossy(),
        &dest_dev.to_string_lossy()
    );
    let mount_dev = Mount::builder()
        .flags(MountFlags::BIND)
        .mount_autodrop(&src_dev, &dest_dev, flags)
        .err_prepend("When mounting vkfs dev /dev")?;

    // /dev/pts
    std::fs::create_dir_all(&dest_dev_pts).err_prepend("When ensuring /dev/pts")?;
    info!(
        "[vkfs] Mounting devpts to {}",
        &dest_dev_pts.to_string_lossy()
    );
    let mount_dev_pts = Mount::builder()
        .fstype("devpts")
        .mount_autodrop("devpts", &dest_dev_pts, flags)
        .err_prepend("When mounting vkfs devpts /dev/pts")?;

    // /proc
    info!("[vkfs] Mounting proc to {}", &dest_proc.to_string_lossy());
    let mount_proc = Mount::builder()
        .fstype("proc")
        .mount_autodrop("proc", &dest_proc, flags)
        .err_prepend("When mounting vkfs proc /proc")?;

    // /sys
    info!("[vkfs] Mounting sysfs to {}", &dest_sys.to_string_lossy());
    let mount_sys = Mount::builder()
        .fstype("sysfs")
        .mount_autodrop("sysfs", &dest_sys, flags)
        .err_prepend("When mounting vkfs sysfs /sys")?;

    // /tmp
    info!("[vkfs] Mounting tmpfs to {}", &dest_tmp.to_string_lossy());
    let mount_tmp = Mount::builder()
        .fstype("tmpfs")
        .mount_autodrop("tmpfs", &dest_tmp, flags)
        .err_prepend("When mounting tmpfs")
        .err_prepend("When mounting vkfs tmpfs /tmp")?;

    // Push the UnmountDrops to a vector
    let mut res: Vec<UnmountDrop<Mount>> = Vec::new();

    res.push(mount_proc);
    res.push(mount_dev);
    res.push(mount_dev_pts);
    res.push(mount_sys);
    res.push(mount_tmp);

    Ok(res)
}
