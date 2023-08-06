use anyhow::Result;
use asset_catalog::write_asset_catalog;
use clap::{Parser, Subcommand};
use colored::*;
use parser::parse_document_from_file;
use std::path::{Path, PathBuf};
use swift_gen::gen_swift;

#[derive(clap::ValueEnum, Clone, Default)]
enum RenderMode {
  #[default]
  #[value(name = "asset-catalog")]
  ColorSet,
  DynamicColor,
}

#[derive(clap::ValueEnum, Clone, Default)]
enum AccessLevel {
  #[default]
  Internal,
  Public,
}

#[derive(clap::ValueEnum, Clone, Default)]
enum ColorSpace {
  DisplayP3,
  #[default]
  SRGB,
  #[value(name = "extended-srgb")]
  ExtendedRangeSRGB,
  #[value(name = "extended-linear-srgb")]
  ExtendedRangeLinearSRGB,
}

#[derive(Parser)]
#[command(
  author,
  version,
  about,
  arg_required_else_help(true),
  max_term_width(100)
)]
struct Cli {
  #[clap(subcommand)]
  cmd: Command,
}

#[derive(Subcommand)]
enum Command {
  /// generates Swift code
  #[command(name = "gen-swift")]
  GenerateSwiftCode {
    /// Sets the input file
    input_file: PathBuf,
    /// Sets the output filename (e.g. Colors.xcassets)
    #[arg(short)]
    output_path: PathBuf,
    /// Specify if the generated code should reference the asset catalog or create dynamic colors programmatically
    #[arg(name = "mode", value_enum, default_value_t, long, short)]
    render_mode: RenderMode,
    /// The access level for the generated code
    #[arg(name = "access", value_enum, default_value_t, long, short)]
    access_level: AccessLevel,
  },
  /// generates the Asset Catalog
  #[command(name = "gen-assets")]
  GenerateAssetCatalog {
    /// Sets the input file
    input_file: PathBuf,
    /// Sets the output filename (e.g. Colors.xcassets)
    #[arg(short)]
    output_path: PathBuf,
    /// Specify which colorspace to use
    #[arg(value_enum, default_value_t, long, short)]
    color_space: ColorSpace,
    /// Overwrite Asset catalog if it already exists
    #[arg(name = "force", long, short)]
    overwrite_asset_catalog: bool,
  },
}

fn main() {
  let result = match Cli::parse().cmd {
    Command::GenerateSwiftCode {
      input_file,
      output_path,
      render_mode,
      access_level,
    } => generate_swift_code(input_file, output_path, render_mode, access_level),
    Command::GenerateAssetCatalog {
      input_file,
      output_path,
      color_space,
      overwrite_asset_catalog,
    } => generate_asset_catalog(
      input_file,
      output_path,
      color_space,
      overwrite_asset_catalog,
    ),
  };
  match result {
    Ok(_) => (),
    Err(e) => {
      println!("{}", format!("{}", e).red());
      std::process::exit(0x0100);
    }
  }
}

fn generate_swift_code(
  input_file: impl AsRef<Path>,
  output_path: impl AsRef<Path>,
  render_mode: RenderMode,
  access_level: AccessLevel,
) -> Result<()> {
  let doc = parse_document_from_file(input_file)?;
  let output_path = output_path.as_ref();

  match gen_swift(
    &doc,
    output_path,
    render_mode.into(),
    false,
    access_level.into(),
  ) {
    Err(e @ swift_gen::Error::FileIsIdentical { .. }) => {
      println!("{}", format!("{}", e).dimmed())
    }
    Err(e) => return Err(anyhow::Error::new(e)),
    Ok(_) => println!(
      "{}",
      format!("Generated Swift file at {}.", output_path.display()).green()
    ),
  }

  Ok(())
}

fn generate_asset_catalog(
  input_file: impl AsRef<Path>,
  output_path: impl AsRef<Path>,
  color_space: ColorSpace,
  overwrite_asset_catalog: bool,
) -> Result<()> {
  let doc = parse_document_from_file(input_file)?;
  let output_path = output_path.as_ref();

  match write_asset_catalog(
    &doc,
    output_path,
    color_space.into(),
    overwrite_asset_catalog,
  ) {
    Err(asset_catalog::Error::CatalogExists { .. }) => {
      println!(
        "{}",
        format!(
          "Asset catalog at {} already exists. Use -f to overwrite it.",
          output_path.display()
        )
        .yellow()
      );
      std::process::exit(0x0100);
    }
    Err(e) => return Err(anyhow::Error::new(e)),
    Ok(_) => println!(
      "{}",
      format!("Generated Asset catalog at {}.", output_path.display()).green()
    ),
  }

  Ok(())
}

impl From<RenderMode> for swift_gen::RenderMode {
  fn from(value: RenderMode) -> Self {
    match value {
      RenderMode::ColorSet => swift_gen::RenderMode::ColorSet,
      RenderMode::DynamicColor => swift_gen::RenderMode::DynamicColor,
    }
  }
}

impl From<AccessLevel> for swift_gen::AccessLevel {
  fn from(value: AccessLevel) -> Self {
    match value {
      AccessLevel::Internal => swift_gen::AccessLevel::Internal,
      AccessLevel::Public => swift_gen::AccessLevel::Public,
    }
  }
}

impl From<ColorSpace> for asset_catalog::ColorSpace {
  fn from(value: ColorSpace) -> Self {
    match value {
      ColorSpace::DisplayP3 => asset_catalog::ColorSpace::DisplayP3,
      ColorSpace::SRGB => asset_catalog::ColorSpace::SRGB,
      ColorSpace::ExtendedRangeSRGB => asset_catalog::ColorSpace::ExtendedRangeSRGB,
      ColorSpace::ExtendedRangeLinearSRGB => asset_catalog::ColorSpace::ExtendedRangeLinearSRGB,
    }
  }
}
