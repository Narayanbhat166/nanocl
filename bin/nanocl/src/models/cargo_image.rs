use chrono::NaiveDateTime;
use nanocld_client::stubs::cargo_image::ListCargoImagesOptions;
use tabled::Tabled;
use clap::{Parser, Subcommand};
use bollard_next::models::ImageSummary;

#[derive(Debug, Parser)]
pub struct CargoImageRemoveOpts {
  #[clap(short = 'y')]
  pub skip_confirm: bool,
  /// List of image names to delete
  pub(crate) names: Vec<String>,
}

#[derive(Debug, Parser)]
pub struct CargoImagePullOpts {
  /// Name of the image to pull
  pub(crate) name: String,
}

#[derive(Debug, Parser)]
pub struct CargoImageInspectOpts {
  /// Name of the image to inspect
  pub(crate) name: String,
}

#[derive(Debug, Subcommand)]
pub enum CargoImageCommands {
  /// List cargo images
  #[clap(alias("ls"))]
  List(CargoImageListOpts),
  /// Pull a new cargo image
  Pull(CargoImagePullOpts),
  /// Remove an existing cargo image
  #[clap(alias("rm"))]
  Remove(CargoImageRemoveOpts),
  /// Inspect a cargo image
  Inspect(CargoImageInspectOpts),
  /// Import a cargo image from a tarball
  Import(CargoImageImportOpts),
}

#[derive(Clone, Debug, Parser)]
pub struct CargoImageListOpts {
  /// Show all images. Only images from a final layer (no children) are shown by default.
  #[clap(long, short)]
  pub all: bool,
  // TODO: implement filters
  // pub filters: Option<HashMap<String, Vec<String>>>,
  /// Show digest information as a RepoDigests field on each image.
  #[clap(long)]
  pub digests: bool,
  /// Compute and show shared size as a SharedSize field on each image.
  #[clap(long)]
  pub shared_size: bool,
  /// Show only the numeric IDs of images.
  #[clap(long, short)]
  pub quiet: bool,
}

impl From<CargoImageListOpts> for ListCargoImagesOptions {
  fn from(options: CargoImageListOpts) -> Self {
    Self {
      all: Some(options.all),
      digests: Some(options.digests),
      shared_size: Some(options.shared_size),
      ..Default::default()
    }
  }
}

#[derive(Debug, Parser)]
pub struct CargoImageImportOpts {
  /// path to tar archive
  #[clap(short = 'f')]
  pub(crate) file_path: String,
}

/// Manage container images
#[derive(Debug, Parser)]
#[clap(name = "nanocl cargo image")]
pub struct CargoImageOpts {
  #[clap(subcommand)]
  pub(crate) commands: CargoImageCommands,
}

#[derive(Tabled)]
pub struct CargoImageRow {
  pub(crate) id: String,
  pub(crate) repositories: String,
  pub(crate) tag: String,
  pub(crate) size: String,
  pub(crate) created: String,
}

fn convert_size(size: i64) -> String {
  if size >= 1_000_000_000 {
    format!("{} GB", size / 1024 / 1024 / 1024)
  } else {
    format!("{} MB", size / 1024 / 1024)
  }
}

impl From<ImageSummary> for CargoImageRow {
  fn from(value: ImageSummary) -> Self {
    let binding = value
      .repo_tags
      .get(0)
      .unwrap_or(&String::from("<none>"))
      .to_owned();
    let vals: Vec<_> = binding.split(':').collect();
    let id = value.id.replace("sha256:", "");
    let id = id[0..12].to_owned();
    let created = NaiveDateTime::from_timestamp_opt(value.created, 0).unwrap();
    let created = created.format("%Y-%m-%d %H:%M:%S").to_string();

    Self {
      id,
      repositories: vals.first().unwrap_or(&"<none>").to_string(),
      tag: vals.get(1).unwrap_or(&"<none>").to_string(),
      size: convert_size(value.size),
      created,
    }
  }
}
