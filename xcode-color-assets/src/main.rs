use asset_catalog::{write_asset_catalog, ColorSpace};
use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand};
use colored::*;
use parser::parse_document_from_file;
use std::path::Path;
use std::str::FromStr;
use swift_gen::{gen_swift, AccessLevel, RenderMode};

fn main() {
  let matches = App::new(crate_name!())
    .version(crate_version!())
    .about(crate_description!())
    .global_setting(AppSettings::ColorAuto)
    .global_setting(AppSettings::ColoredHelp)
    .global_setting(AppSettings::DeriveDisplayOrder)
    .global_setting(AppSettings::UnifiedHelpMessage)
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .subcommand(
      SubCommand::with_name("gen-assets")
        .about("generates the Asset Catalog")
        .arg(
          Arg::with_name("output")
            .short("o")
            .help("Sets the output filename (e.g. Colors.xcassets)")
            .value_name("OUTPUT_FILE")
            .required(true),
        )
        .arg(
          Arg::with_name("input")
            .help("Sets the input file")
            .value_name("INPUT_FILE")
            .required(true)
            .index(1),
        )
        .arg(
          Arg::with_name("force-overwrite")
            .short("f")
            .long("force")
            .help("Overwrite Asset catalog if it already exists"),
        )
        .arg(
          Arg::with_name("color-space")
            .short("cs")
            .long("color-space")
            .takes_value(true)
            .possible_value("display-p3")
            .possible_value("srgb")
            .possible_value("extended-srgb")
            .possible_value("extended-linear-srgb")
            .default_value("srgb")
            .help("Specify which colorspace to use")
            .value_name("COLOR-SPACE"),
        ),
    )
    .subcommand(
      SubCommand::with_name("gen-swift")
        .about("generates Swift code")
        .arg(
          Arg::with_name("output")
            .short("o")
            .help("Sets the output filename (e.g. Colors.swift)")
            .value_name("OUTPUT_FILE")
            .required(true),
        )
        .arg(
          Arg::with_name("input")
            .help("Sets the input file")
            .value_name("INPUT_FILE")
            .required(true)
            .index(1),
        )
        .arg(
          Arg::with_name("mode")
            .short("m")
            .long("mode")
            .takes_value(true)
            .possible_value("asset-catalog")
            .possible_value("dynamic-color")
            .default_value("asset-catalog")
            .help("Specify if the generated code should reference the asset catalog or create dynamic colors programmatically")
            .value_name("MODE"),
        )
        .arg(
          Arg::with_name("access-level")
            .short("a")
            .long("access")
            .takes_value(true)
            .possible_value("internal")
            .possible_value("public")
            .default_value("internal")
            .help("The access level for the generated code")
            .value_name("ACCESS-LEVEL"),
        ),
    )
    .get_matches();

  match matches.subcommand() {
    ("gen-assets", Some(m)) => {
      let input_file = m.value_of("input").unwrap();
      let output_path = m.value_of("output").unwrap();
      let color_space =
        ColorSpace::from_str(m.value_of("color-space").unwrap()).expect("Unknown colorspace");

      let doc = match parse_document_from_file(&input_file) {
        Ok(doc) => doc,
        Err(e) => {
          println!("{}", format!("{}", e).red());
          std::process::exit(0x0100);
        }
      };

      let overwrite_asset_catalog = m.is_present("force-overwrite");

      match write_asset_catalog(
        &doc,
        &Path::new(output_path),
        color_space,
        overwrite_asset_catalog,
      ) {
        Err(asset_catalog::Error::CatalogExists(_)) => {
          println!(
            "{}",
            format!(
              "Asset catalog at {} already exists. Use -f to overwrite it.",
              output_path
            )
            .yellow()
          );
          std::process::exit(0x0100);
        }
        Err(e) => {
          println!("{}", format!("{}", e).red());
          std::process::exit(0x0100);
        }
        Ok(_) => println!(
          "{}",
          format!("Generated Asset catalog at {}.", output_path).green()
        ),
      }
    }
    ("gen-swift", Some(m)) => {
      let input_file = m.value_of("input").unwrap();
      let output_path = m.value_of("output").unwrap();
      let render_mode =
        RenderMode::from_str(m.value_of("mode").unwrap()).expect("Unknown render mode");
      let access_level =
        AccessLevel::from_str(m.value_of("access-level").unwrap()).expect("Unknown access level");

      let doc = match parse_document_from_file(&input_file) {
        Ok(doc) => doc,
        Err(e) => {
          println!("{}", format!("{}", e).red());
          std::process::exit(0x0100);
        }
      };

      match gen_swift(
        &doc,
        &Path::new(output_path),
        render_mode,
        false,
        access_level,
      ) {
        Err(e @ swift_gen::Error::FileIsIdentical(_)) => println!("{}", format!("{}", e).dimmed()),
        Err(e) => {
          println!("{}", format!("{}", e).red());
          std::process::exit(0x0100);
        }
        Ok(_) => println!(
          "{}",
          format!("Generated Swift file at {}.", output_path).green()
        ),
      }
    }
    (&_, _) => {}
  }
}
