use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use service_manager::{
    ServiceInstallCtx, ServiceLabel, ServiceManager, ServiceStartCtx, ServiceStopCtx,
    ServiceUninstallCtx,
};
use std::env::current_exe;

fn label() -> ServiceLabel {
    "dev.agents-mcp".parse().expect("valid service label")
}

fn manager() -> Result<Box<dyn ServiceManager>> {
    service_manager::native().context("Failed to create service manager")
}

fn state_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| std::path::Path::new("/tmp").to_path_buf())
        .join("agents-mcp")
}

fn installed_flag() -> PathBuf {
    state_dir().join(".installed")
}

pub fn install() -> Result<()> {
    let manager = manager()?;
    let exe = current_exe().context("Failed to get current exe path")?;

    manager
        .install(ServiceInstallCtx {
            label: label(),
            program: exe,
            args: vec!["--daemon".into()],
            autostart: true,
            contents: None,
        })
        .context("Failed to install service")?;

    manager
        .start(ServiceStartCtx {
            label: label(),
        })
        .context("Failed to start service")?;

    fs::write(installed_flag(), "")?;

    Ok(())
}

pub fn start() -> Result<()> {
    let manager = manager()?;
    manager
        .start(ServiceStartCtx {
            label: label(),
        })
        .context("Failed to start service")
}

pub fn stop() -> Result<()> {
    let manager = manager()?;
    manager
        .stop(ServiceStopCtx {
            label: label(),
        })
        .context("Failed to stop service")
}

pub fn uninstall() -> Result<()> {
    let manager = manager()?;
    let _ = manager.stop(ServiceStopCtx {
        label: label(),
    });
    manager
        .uninstall(ServiceUninstallCtx {
            label: label(),
        })
        .context("Failed to uninstall service")?;

    let _ = fs::remove_file(installed_flag());
    let _ = fs::remove_file(crate::daemon::pid_file_path());

    Ok(())
}

pub fn is_installed() -> bool {
    installed_flag().exists()
}
