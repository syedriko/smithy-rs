/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use anyhow::Result;
use clap::Parser;
use render::subcommand_render;
use split::subcommand_split;

mod entry;
mod render;
mod split;

#[derive(Parser, Debug, Eq, PartialEq)]
#[clap(name = "changelogger", author, version, about)]
pub enum Args {
    /// Split SDK changelog entries into a separate file
    Split(split::SplitArgs),
    /// Render a TOML/JSON changelog into GitHub-flavored Markdown
    Render(render::RenderArgs),
}

fn main() -> Result<()> {
    match Args::parse() {
        Args::Split(split) => subcommand_split(&split),
        Args::Render(render) => subcommand_render(&render),
    }
}

#[cfg(test)]
mod tests {
    use super::Args;
    use crate::entry::ChangeSet;
    use crate::render::RenderArgs;
    use crate::split::SplitArgs;
    use clap::Parser;
    use std::path::PathBuf;

    #[test]
    fn args_parsing() {
        assert_eq!(
            Args::Split(SplitArgs {
                source: PathBuf::from("fromplace"),
                destination: PathBuf::from("someplace"),
                since_commit: None,
                smithy_rs_location: None,
            }),
            Args::try_parse_from([
                "./changelogger",
                "split",
                "--source",
                "fromplace",
                "--destination",
                "someplace"
            ])
            .unwrap()
        );

        assert_eq!(
            Args::Render(RenderArgs {
                change_set: ChangeSet::SmithyRs,
                independent_versioning: false,
                source: vec![PathBuf::from("fromplace")],
                source_to_truncate: PathBuf::from("fromplace"),
                changelog_output: PathBuf::from("some-changelog"),
                release_manifest_output: Some(PathBuf::from("some-manifest")),
                previous_release_versions_manifest: None,
                date_override: None,
                smithy_rs_location: None,
                smithy_rs_sdk: None,
            }),
            Args::try_parse_from([
                "./changelogger",
                "render",
                "--change-set",
                "smithy-rs",
                "--source",
                "fromplace",
                "--source-to-truncate",
                "fromplace",
                "--changelog-output",
                "some-changelog",
                "--release-manifest-output",
                "some-manifest"
            ])
            .unwrap()
        );

        assert_eq!(
            Args::Render(RenderArgs {
                change_set: ChangeSet::AwsSdk,
                independent_versioning: true,
                source: vec![
                    PathBuf::from("fromplace"),
                    PathBuf::from("fromanotherplace")
                ],
                source_to_truncate: PathBuf::from("fromplace"),
                changelog_output: PathBuf::from("some-changelog"),
                release_manifest_output: None,
                previous_release_versions_manifest: None,
                date_override: None,
                smithy_rs_location: None,
                smithy_rs_sdk: None,
            }),
            Args::try_parse_from([
                "./changelogger",
                "render",
                "--change-set",
                "aws-sdk",
                "--independent-versioning",
                "--source",
                "fromplace",
                "--source",
                "fromanotherplace",
                "--source-to-truncate",
                "fromplace",
                "--changelog-output",
                "some-changelog",
            ])
            .unwrap()
        );

        assert_eq!(
            Args::Render(RenderArgs {
                change_set: ChangeSet::AwsSdk,
                independent_versioning: true,
                source: vec![PathBuf::from("fromplace")],
                source_to_truncate: PathBuf::from("fromplace"),
                changelog_output: PathBuf::from("some-changelog"),
                release_manifest_output: None,
                previous_release_versions_manifest: Some(PathBuf::from("path/to/versions.toml")),
                date_override: None,
                smithy_rs_location: None,
                smithy_rs_sdk: None,
            }),
            Args::try_parse_from([
                "./changelogger",
                "render",
                "--change-set",
                "aws-sdk",
                "--independent-versioning",
                "--source",
                "fromplace",
                "--source-to-truncate",
                "fromplace",
                "--changelog-output",
                "some-changelog",
                "--previous-release-versions-manifest",
                "path/to/versions.toml"
            ])
            .unwrap()
        );
    }
}
