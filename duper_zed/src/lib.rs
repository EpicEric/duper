use std::fs;

use serde::Deserialize;
use zed_extension_api::{self as zed, http_client};

const CRATES_IO_DUPER_LSP_URL: &str = "https://crates.io/api/v1/crates/duper_lsp";
const DOWNLOAD_BASE_URL: &str = "https://codeberg.org/api/packages/EpicEric9/generic";

struct DuperExtension {
    cached_binary_path: Option<String>,
}

#[derive(Deserialize)]
struct CrateResponse {
    #[serde(rename = "crate")]
    krate: CrateDataResponse,
}

#[derive(Deserialize)]
struct CrateDataResponse {
    max_version: String,
}

impl DuperExtension {
    fn get_fresh_binary_path(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<String> {
        if let Some(path) = self.cached_binary_path.as_ref() {
            return Ok(path.clone());
        }

        if let Some(path) = worktree.which("duper_lsp") {
            self.cached_binary_path = Some(path.clone());
            return Ok(path);
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        // Figure out executable name from OS + architecture
        let (os, arch) = zed::current_platform();
        let executable = format!(
            "duper_lsp_{}-{}{}",
            match os {
                zed::Os::Mac => "darwin",
                zed::Os::Linux => "linux",
                zed::Os::Windows => "win32",
            },
            match arch {
                zed::Architecture::Aarch64 => "arm64",
                zed::Architecture::X8664 => "x64",
                zed::Architecture::X86 => return Err("The x86 architecture is unsupported.".into()),
            },
            match os {
                zed::Os::Windows => ".exe",
                _ => "",
            },
        );

        // Get latest version from crates.io
        let crates_io_response = http_client::fetch(
            &http_client::HttpRequest::builder()
                .method(http_client::HttpMethod::Get)
                .url(CRATES_IO_DUPER_LSP_URL)
                .redirect_policy(http_client::RedirectPolicy::FollowLimit(5))
                .build()?,
        )?;
        let crates_io_data: CrateResponse =
            serde_json::from_slice(&crates_io_response.body).map_err(|error| error.to_string())?;
        let version = crates_io_data.krate.max_version;

        // Check if binary exists
        let current_version_path = format!("duper_lsp/{}/{}", version, executable);
        if fs::exists(&current_version_path).unwrap_or(false) {
            self.cached_binary_path = Some(current_version_path.clone());
            return Ok(current_version_path);
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        // Remove old versions
        if let Ok(duper_path) = fs::read_dir("duper_lsp") {
            for entry in duper_path.flatten() {
                fs::remove_dir_all(entry.path()).map_err(|error| error.to_string())?;
            }
        }
        fs::create_dir_all(format!("duper_lsp/{}", version)).map_err(|error| error.to_string())?;

        // Download latest version
        zed::download_file(
            &format!("{}/{}", DOWNLOAD_BASE_URL, current_version_path),
            &current_version_path,
            zed::DownloadedFileType::Uncompressed,
        )?;
        zed::make_file_executable(&current_version_path)?;

        self.cached_binary_path = Some(current_version_path.clone());
        Ok(current_version_path)
    }
}

impl zed::Extension for DuperExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        DuperExtension {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        Ok(zed::Command {
            command: self.get_fresh_binary_path(language_server_id, worktree)?,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(DuperExtension);
