use crate::PackageBuild;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The core configuration for a builder instance
#[derive(Debug, Deserialize, Serialize)]
pub struct BuilderConfiguration {
    /// The operation root, nothing will escape this root
    pub root: PathBuf,
    /// The environment to use
    pub environment: BuildEnvironment,
}

/// An environment consisting of a name and available / required packages
#[derive(Debug, Deserialize, Serialize)]
pub struct BuildEnvironment {
    /// The name of the environment
    pub name: String,
    /// The packages provided by this environment
    pub packages: Vec<String>,
}

impl BuilderConfiguration {
    /// The path to the environments: `<root>/environments`
    pub fn get_environments_dir(&self) -> PathBuf {
        self.root.join("environments")
    }

    /// The path to the all caches: `<root>/cache`
    pub fn get_cache_dir(&self) -> PathBuf {
        self.root.join("cache")
    }

    /// The path to the build folders: `<root>/build`
    pub fn get_builds_dir(&self) -> PathBuf {
        self.root.join("build")
    }

    /// The path to the target directories (build artifacts): `<root>/target`
    pub fn get_targets_dir(&self) -> PathBuf {
        self.root.join("target")
    }

    /// Get the path to the current environment root
    pub fn get_environment_root_dir(&self) -> PathBuf {
        self.get_environments_dir().join(&self.environment.name)
    }

    /// Get the location for leaf caches
    pub fn get_leaf_cache_dir(&self) -> PathBuf {
        self.get_cache_dir().join("leaf")
    }

    /// The path for the overlayfs `work` dir
    pub fn get_overlay_work_dir(&self) -> PathBuf {
        self.get_cache_dir().join("overlay_work")
    }

    /// The path for the overlayfs `upper` dir
    pub fn get_overlay_upper_dir(&self) -> PathBuf {
        self.get_cache_dir().join("overlay_upper")
    }

    /// The location of the build directory, the runner root
    pub fn get_build_dir(&self, pkgbuild: &PackageBuild) -> PathBuf {
        self.get_builds_dir().join(format!(
            "{}-{}-{}",
            pkgbuild.name, pkgbuild.version, pkgbuild.real_version
        ))
    }

    /// The directory to store the build target (artifact) in
    pub fn get_target_dir(&self, pkgbuild: &PackageBuild) -> PathBuf {
        self.get_targets_dir().join(format!(
            "{}-{}-{}/package",
            pkgbuild.name, pkgbuild.version, pkgbuild.real_version
        ))
    }

    /// The `target` directory location within the build root
    pub fn get_buildroot_target_dir(&self, pkgbuild: &PackageBuild) -> PathBuf {
        self.get_build_dir(pkgbuild).join("target")
    }

    /// The `build` directory location within the build root
    pub fn get_buildroot_build_dir(&self, pkgbuild: &PackageBuild) -> PathBuf {
        self.get_build_dir(pkgbuild).join("build")
    }
}

impl BuildEnvironment {
    /// Create a new build environment from scratch
    /// # Arguments
    /// * `name` - The name for the new environment
    /// * `packages` - The available packages in the environment
    pub fn new(name: &str, packages: Vec<&str>) -> Self {
        let mut n_packages: Vec<String> = Vec::new();
        for package in packages {
            n_packages.push(package.to_owned());
        }
        Self {
            name: name.to_owned(),
            packages: n_packages,
        }
    }
}
