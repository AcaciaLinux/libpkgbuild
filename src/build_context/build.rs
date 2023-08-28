use std::io::Write;
use std::process::ExitStatus;
use std::sync::{Arc, Mutex};
use std::{fs::File, process::Command};

use crate::{BCError, StdIOErrorExt};

use super::BuildContext;

impl<'a> BuildContext<'a> {
    /// Build the package using the context
    pub fn build_package(&mut self) -> Result<(), BCError> {
        match &self.pkgbuild.prepare {
            Some(script) => {
                info!("PREPARE script exists, running...");
                let status = self.run_script(script, "prepare.sh")?;
                info!("PREPARE script is done: SUCCESS: {}", status.success())
            }
            None => {
                info!("PREPARE script does not exist, skipping");
            }
        }

        match &self.pkgbuild.build {
            Some(script) => {
                info!("BUILD script exists, running...");
                let status = self.run_script(script, "build.sh")?;
                info!("BUILD script is done: SUCCESS: {}", status.success())
            }
            None => {
                info!("BUILD script does not exist, skipping");
            }
        }

        match &self.pkgbuild.check {
            Some(script) => {
                info!("CHECK script exists, running...");
                let status = self.run_script(script, "check.sh")?;
                info!("CHECK script is done: SUCCESS: {}", status.success())
            }
            None => {
                info!("CHECK script does not exist, skipping");
            }
        }

        match &self.pkgbuild.package {
            Some(script) => {
                info!("PACKAGE script exists, running...");
                let status = self.run_script(script, "package.sh")?;
                info!("PACKAGE script is done: SUCCESS: {}", status.success())
            }
            None => {
                info!("PACKAGE script does not exist, skipping");
            }
        }

        Ok(())
    }

    /// Run a script using the environment
    /// # Arguments
    /// * `script` - The lines of the script
    /// * `script_name` - The name the script file should have
    fn run_script(
        &mut self,
        script: &Vec<String>,
        script_name: &str,
    ) -> Result<ExitStatus, BCError> {
        let path = self
            .config
            .get_buildroot_build_dir(self.pkgbuild)
            .join(script_name);

        let mut output = File::create(&path).err_prepend("When creating build script")?;

        for line in script {
            writeln!(output, "{}", line).err_prepend("When populating build script")?;
        }

        let command_string = format!(
            "
            set -e &&
            export PKG_NAME={} &&
            export PKG_VERSION={} &&
            export PKG_ROOT=/target &&
            export PKG_INSTALL_DIR=$PKG_ROOT/data &&
            cd build &&
            /bin/sh /build/{}
        ",
            self.pkgbuild.name, self.pkgbuild.version, script_name
        );

        let mut command = Command::new("/usr/bin/chroot");
        let child = command
            .arg(self.config.get_build_dir(self.pkgbuild))
            .args(["/bin/sh", "-c", &command_string])
            .spawn()
            .unwrap();

        let sond = Arc::new(Mutex::new(child));

        let sond2 = sond.clone();
        ctrlc::set_handler(move || match sond2.lock().expect("Lock mutex").kill() {
            Ok(_) => {}
            Err(_) => {}
        })
        .unwrap();

        let mut output = sond.lock().expect("Lock mutex");

        Ok(output.wait().unwrap())
    }
}
