use os_type;
use semver::Version;
use reqwest;
use url::Url;

use std::collections::HashMap;
use std::process::Command;
use std::str::FromStr;
use std::string::ParseError;
use std::env;
use std::fs::File;
use std::fs;

#[derive(Debug)]
pub struct OmnitruckMetadata {
    sha1: String,
    sha256: Option<String>,
    url: Url,
    version: String,
}

impl FromStr for OmnitruckMetadata {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut member_map = s.lines()
            .map(|kv| kv.split('\t'))
            .map(|mut kv| (kv.next().unwrap().into(), kv.next().unwrap().into()))
            .collect::<HashMap<String, String>>();

        Ok(OmnitruckMetadata {
            sha1: member_map.remove("sha1").unwrap(),
            sha256: member_map.remove("sha256"),
            url: Url::parse(&member_map.remove("url").unwrap()).unwrap(),
            version: member_map.remove("version").unwrap(),
        })
    }
}

#[cfg(target_os = "windows")]
pub fn install_chefdk() {
    let filename = fetch_chefdk();
    println!("File: {}", filename);
}

#[cfg(target_os = "macos")]
pub fn install_chefdk() {
    let filename = fetch_chefdk();
    println!("File: {}", filename);

    let tmp_dir = env::home_dir().unwrap().join(".workup");
    fs::create_dir_all(&tmp_dir).unwrap();

    let mountpoint = tmp_dir.clone().join("chefdk_mount");

    Command::new("hdiutil")
            .arg("detach")
            .arg(mountpoint.clone())
            .arg("-force")
            .output()
            .expect("failed to execute process");

    Command::new("hdiutil")
            .arg("attach")
            .arg("-mountpoint")
            .arg(mountpoint.clone())
            .arg(filename.clone())
            .output()
            .expect("failed to execute process");

    let mut pkg_path = mountpoint.join(filename);
    pkg_path.set_extension("pkg");

    let chefdk_pkg_path = tmp_dir.clone().join("chefdk_pkg");
    fs::create_dir_all(&chefdk_pkg_path).unwrap();

    Command::new("xar")
            .arg("-x")
            .arg("-f")
            .arg(pkg_path.clone())
            .arg("-C")
            .arg(chefdk_pkg_path.clone())
            .output()
            .expect("failed to execute process");

    let payload_path = chefdk_pkg_path.clone().join("chefdk-core.pkg").join("Payload");

    let chefdk_path = tmp_dir.clone().join("chefdk");
    fs::create_dir_all(&chefdk_path).unwrap();

    Command::new("tar")
            .arg("-xvf")
            .arg(payload_path.clone())
            .arg("-C")
            .arg(chefdk_path.clone())
            .output()
            .expect("failed to execute process");
}

fn fetch_chefdk() -> String {
    let metadata = fetch_chefdk_metadata("2.4.17");

    println!("Fetching Chef DK from {}", metadata.url);

    let filename = metadata
        .url
        .path_segments()
        .unwrap()
        .last()
        .unwrap()
        .to_owned();

    let mut resp = reqwest::get(metadata.url).unwrap();
    let mut file = File::create(&filename).unwrap();

    let _ = resp.copy_to(&mut file);

    filename
}

pub fn fetch_chefdk_metadata(chefdk_version: &str) -> OmnitruckMetadata {
    let metadata_url = chefdk_metadata_url(chefdk_version);

    println!("Fetching Chef DK metadata from {}", metadata_url);

    let mut resp = reqwest::get(&metadata_url).unwrap();
    resp.text().unwrap().parse().unwrap()
}

#[cfg(target_os = "windows")]
fn chefdk_metadata_url(chefdk_version: &str) -> String {
    String::default()
}

#[cfg(target_os = "macos")]
fn chefdk_metadata_url(chefdk_version: &str) -> String {
    let os = os_type::current_platform();

    let machine_version = Version::parse(&os.version).unwrap();
    let version = format!("{}.{}", machine_version.major, machine_version.minor);

    let arch = if cfg!(target_pointer_width = "64") {
        "x86_64"
    } else if cfg!(target_pointer_width = "32") {
        "i386"
    } else {
        panic!();
    };

    format!(
        "https://omnitruck.chef.io/{}/{}/metadata?v={}&p={}&pv={}&m={}",
        "stable", "chefdk", chefdk_version, "mac_os_x", version, arch
    )
}
