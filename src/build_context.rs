use crate::util::*;
use std::path::PathBuf;

mod sources;

use leaf::error::LError;
use leaf::error::LErrorClass;
use leaf::Leaf;
use sys_mount::Mount;
use sys_mount::UnmountDrop;

use crate::StdIOErrorExt;
use crate::{mount, BuilderConfiguration, PackageBuild};

/// A build context with valid mounts, config and packagebuild
#[allow(dead_code)]
pub struct BuildContext<'a> {
    pkgbuild: &'a PackageBuild,
    config: &'a BuilderConfiguration,
    mounts: Vec<UnmountDrop<Mount>>,
}

/// All possible kinds of build context errors
#[derive(Debug)]
pub enum BCErrorKind {
    IO(std::io::ErrorKind),
    Leaf(LErrorClass),
    ZIP(zip::result::ZipError),
}

/// A build context error
#[derive(Debug)]
#[allow(dead_code)]
pub struct BCError {
    kind: BCErrorKind,
    message: String,
}

impl PackageBuild {
    /// Create a build context from a packagebuild
    /// * `config` - The configuration to use for the context
    /// * `leaf` - The leaf instance to use for installing packages
    pub fn build_context<'a>(
        &'a mut self,
        config: &'a BuilderConfiguration,
        leaf: &mut Leaf,
    ) -> Result<BuildContext, BCError> {
        let mut mounts: Vec<UnmountDrop<Mount>> = Vec::new();

        info!("Ensuring directories...");
        clean_dir(&config.get_overlay_upper_dir())?;
        clean_dir(&config.get_build_dir(&self))?;
        clean_dir(&config.get_target_dir(&self))?;

        info!(
            "Installing '{}' environment packages to {}",
            &config.environment.name,
            &config.get_environment_root_dir().to_string_lossy()
        );
        leaf.config.root = Some(config.get_environment_root_dir());
        match leaf.update() {
            Ok(_) => {}
            Err(v_e) => {
                return Err(BCError::from(
                    v_e.first().expect("At least one error").clone(),
                ))
            }
        };
        leaf.install(&config.environment.packages)?;

        info!("Mounting overlay");
        mounts.push(
            mount::mount_overlay(
                &config.get_environment_root_dir(),
                &config.get_overlay_work_dir(),
                &config.get_overlay_upper_dir(),
                &config.get_build_dir(self),
            )
            .err_prepend("When mounting overlay")?,
        );

        info!("Mounting virtual kernel filesystems...");
        mounts.append(
            &mut mount::mount_vkfs(&PathBuf::from("/"), &config.get_build_dir(self))
                .err_prepend("When mounting virtual kernel filesystems")?,
        );

        info!("Ensuring buildroot directories...");
        clean_dir(&config.get_buildroot_target_dir(&self))
            .err_prepend("When creating buildroot target directory")?;
        clean_dir(&config.get_buildroot_build_dir(&self))
            .err_prepend("When creating buildroot build directory")?;

        info!("Mounting target...");
        mounts.push(
            mount::mount_bind(
                &config.get_target_dir(&self),
                &config.get_buildroot_target_dir(&self),
            )
            .err_prepend("When mounting target directory")?,
        );

        info!("Installing build dependencies");
        leaf.config.root = Some(config.get_build_dir(self));
        if let Some(deps) = &self.build_dependencies {
            leaf.install(deps)?;
        }

        Ok(BuildContext {
            pkgbuild: self,
            config: config,
            mounts,
        })
    }
}

impl From<std::io::Error> for BCError {
    fn from(value: std::io::Error) -> Self {
        Self {
            kind: BCErrorKind::IO(value.kind()),
            message: value.to_string(),
        }
    }
}

impl From<LError> for BCError {
    fn from(value: LError) -> Self {
        Self {
            kind: BCErrorKind::Leaf(value.class),
            message: value.message.unwrap_or("Unknown".to_owned()),
        }
    }
}

impl From<zip::result::ZipError> for BCError {
    fn from(value: zip::result::ZipError) -> Self {
        Self {
            message: format!("ZIP error: {}", value),
            kind: BCErrorKind::ZIP(value),
        }
    }
}
