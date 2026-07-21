use std::{fs, io::Cursor, path::Path};

use scraper::{Html, Selector};
use snafu::ResultExt;

use crate::core::ArchiveExtractionSnafu;

use super::{DirectoryCreationSnafu, Error, Game, SourceEngineVersion};

pub struct MetamodDownloader;

const METAMOD_VERSIONS_URL: &str = "https://mms.alliedmods.net/mmsdrop";

#[cfg(target_os = "windows")]
const TARGET_OS: &str = "windows";

#[cfg(target_os = "linux")]
const TARGET_OS: &str = "linux";

#[derive(Debug, Clone)]
pub enum MetamodBranch {
    Stable,
    Dev,
}

impl MetamodDownloader {
    pub async fn download(
        path: impl AsRef<Path>,
        game: &Game,
        branch: &MetamodBranch,
        source_version: &SourceEngineVersion,
    ) -> Result<(), Error> {
        let version = get_latest_metamod_version(branch, source_version).await?;

        let path = path.as_ref();

        fs::create_dir_all(path).context(DirectoryCreationSnafu)?;

        let latest_metamod_archive_name_url =
            format!("{METAMOD_VERSIONS_URL}/{version}/mmsource-latest-{TARGET_OS}");

        let metamod_version_name = reqwest::get(latest_metamod_archive_name_url)
            .await
            .map_err(|_| Error::UnableToFindLatestVersionError)?
            .text()
            .await
            .map_err(|_| Error::UnableToFindLatestVersionError)?;

        let metamod_download_url =
            format!("{}/{version}/{metamod_version_name}", METAMOD_VERSIONS_URL);

        let metamod_archive_contents = reqwest::get(metamod_download_url)
            .await
            .map_err(|_| Error::UnableToFindLatestVersionError)?
            .bytes()
            .await
            .map_err(|_| Error::UnableToFindLatestVersionError)?;

        let cursor = Cursor::new(metamod_archive_contents);

        #[cfg(target_os = "linux")]
        {
            use crate::core::TarSnafu;
            use flate2::read::GzDecoder;

            let tar = GzDecoder::new(cursor);

            let mut archive = tar::Archive::new(tar);

            archive
                .unpack(path.to_path_buf().join(format!("{}/", game.arg_name())))
                .context(TarSnafu)
                .context(ArchiveExtractionSnafu)?;
        }

        #[cfg(target_os = "windows")]
        {
            use crate::core::ZipSnafu;

            let mut zip = zip::ZipArchive::new(cursor)
                .context(ZipSnafu)
                .context(ArchiveExtractionSnafu)?;

            zip.extract(path.to_path_buf().join(format!("{}/", game.arg_name())))
                .context(ZipSnafu)
                .context(ArchiveExtractionSnafu)?;
        }

        Ok(())
    }
}

/// Oh God, this is so annoying.
async fn get_latest_metamod_version(
    branch: &MetamodBranch,
    source_version: &SourceEngineVersion,
) -> Result<String, Error> {
    let page_contents = reqwest::get(METAMOD_VERSIONS_URL)
        .await
        .map_err(|_| Error::UnableToFindLatestVersionError)?
        .text()
        .await
        .map_err(|_| Error::UnableToFindLatestVersionError)?;

    let html = Html::parse_fragment(&page_contents);

    let a_selector = Selector::parse("a").map_err(|_| Error::UnableToFindLatestVersionError)?;

    let mut stable = (0u32, 0u32);
    let mut dev = (0u32, 0u32);

    for element in html.select(&a_selector).skip(5) {
        let string = element.inner_html();

        let mut split = string.trim_end_matches('/').trim().split('.');

        let Some(Ok(major)) = split.next().map(|s| s.parse::<u32>()) else {
            continue;
        };

        let Some(Ok(minor)) = split.next().map(|s| s.parse::<u32>()) else {
            continue;
        };

        let version = (major, minor);

        if version > dev {
            stable = dev;
            dev = version;
        } else if version > stable {
            stable = version;
        }
    }

    let (major, minor) = match source_version {
        SourceEngineVersion::Source2 => {
            if stable.0 >= 2 {
                stable
            } else {
                dev
            }
        }
        SourceEngineVersion::Source1 => match branch {
            MetamodBranch::Stable => stable,
            MetamodBranch::Dev => dev,
        },
    };

    Ok(format!("{major}.{minor}"))
}
