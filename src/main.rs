use self_update::{cargo_crate_version, self_replace};

fn main() {
    println!("This version: {}", cargo_crate_version!());
    println!("{:?}", update());
}

// fn update() -> Result<(), Box<dyn std::error::Error>> {
//     let status = self_update::backends::github::Update::configure()
//         .repo_owner("MaryEaston")
//         .repo_name("update-test")
//         .bin_name("update-test")
//         .show_download_progress(true)
//         .current_version(cargo_crate_version!())
//         .build()?
//         .fetch()?;
//     println!("Update status: `{}`!", status.version());
//     Ok(())
// }

fn update() -> Result<(), Box<dyn std::error::Error>> {
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("MaryEaston")
        .repo_name("update-test")
        .build()?
        .fetch()?;
    println!("found releases:");
    println!("{:#?}\n", releases);

    // get the first available release
    let asset = releases[0]
        .asset_for(&self_update::get_target(), None)
        .unwrap();
    println!("asset:");
    println!("{:#?}\n", asset);

    let tmp_dir = tempfile::Builder::new()
        .prefix("self_update")
        .tempdir_in(::std::env::current_dir()?)?;
    let tmp_tarball_path = tmp_dir.path().join(&"new_bin");
    let tmp_tarball = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&tmp_tarball_path)?;

    self_update::Download::from_url(&asset.download_url)
        .set_header(reqwest::header::ACCEPT, "application/octet-stream".parse()?)
        .download_to(&tmp_tarball)?;

    let new_exe = tmp_dir.path().join(tmp_tarball_path);
    self_replace::self_replace(new_exe)?;
    tmp_dir.close()?;

    Ok(())
}
