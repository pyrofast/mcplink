use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use service_manager::{
    native_service_manager, ServiceInstallCtx, ServiceLabel, ServiceManager, ServiceStartCtx,
    ServiceStopCtx, ServiceUninstallCtx,
};

fn label() -> ServiceLabel {
    "dev.agents-mcp".parse().expect("valid service label")
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
    let manager = native_service_manager()?;

    let exe = std::env::current_exe().context("Failed to get current exe path")?;

    manager
        .install(ServiceInstallCtx {
            label: label(),
            program: exe,
            args: vec!["--daemon".into()],
            contents: None,
            username: None,
            working_directory: None,
            environment: None,
            autostart: true,
        })
        .context("Failed to install service")?;

    manager
        .start(ServiceStartCtx {
            label: label(),
        })
        .context("Failed to start service")?;

    if let Some(parent) = installed_flag().parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(installed_flag(), "")?;

    Ok(())
}

pub fn start() -> Result<()> {
    let manager = native_service_manager()?;
    manager
        .start(ServiceStartCtx {
            label: label(),
        })
        .context("Failed to start service")
}

pub fn stop() -> Result<()> {
    let manager = native_service_manager()?;
    manager
        .stop(ServiceStopCtx {
            label: label(),
        })
        .context("Failed to stop service")
}

pub fn uninstall() -> Result<()> {
    let manager = native_service_manager()?;
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
