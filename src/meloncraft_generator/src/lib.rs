use meloncraft_nbt::NbtValue;
use meloncraft_protocol_types::ProtocolType as _;
use serde_json::{Map, value::Value as JsonValue};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{fs, io};
use zip::ZipArchive;

// Thanks to https://stackoverflow.com/a/65192210
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn generate_data() {
    // 1.21.10

    fs::create_dir_all("generated/jar").unwrap();
    fs::create_dir_all("generated/bundler").unwrap();
    fs::create_dir_all("generated/server").unwrap();
    fs::create_dir_all("generated/server-registries").unwrap();
    fs::create_dir_all("generated/registries").unwrap();

    if !fs::exists("generated/jar/bundler.jar").unwrap() {
        println!("Downloading server jar from mojang.com...");
        download_jar();
    }

    if !fs::exists("generated/jar/server.jar").unwrap() {
        println!("Extracting server jar from downloaded bundle jar...");
        let mut root_zip =
            ZipArchive::new(File::open("generated/jar/bundler.jar").unwrap()).unwrap();
        root_zip.extract("generated/bundler").unwrap();
        fs::copy(
            "generated/bundler/META-INF/versions/1.21.10/server-1.21.10.jar",
            "generated/jar/server.jar",
        )
        .unwrap();
    }

    if !fs::exists("generated/registries/banner_pattern.nbt").unwrap() {
        println!("Generating registries.nbt...:");
        println!("Extracting files from inner server jar...");
        let mut bundler_zip =
            ZipArchive::new(File::open("generated/jar/server.jar").unwrap()).unwrap();
        bundler_zip.extract("generated/server").unwrap();

        println!("Copying registry files to generated/registries...");
        copy_dir_all(
            "generated/server/data/minecraft/",
            "generated/server-registries/",
        )
        .unwrap();

        println!("Concatenating registries...");
        for registry_group in fs::read_dir("generated/server-registries").unwrap() {
            let registry_group = registry_group.unwrap().file_name().into_string().unwrap();
            println!("Deseralizing, converting and serializing registry group {registry_group}...");
            read_registry_group(&registry_group);
        }
    }
}

fn read_registry_group(registry_group: &str) {
    let json: serde_json::value::Value = read_registry(PathBuf::from(format!(
        "generated/server-registries/{}/",
        registry_group
    )));
    let mut json_map = Map::new();
    json_map.insert(registry_group.to_owned(), json);
    let json_root = JsonValue::Object(json_map);
    let nbt: NbtValue = json_root.try_into().unwrap();
    let nbt_bytes = nbt.net_serialize();
    fs::write(
        format!("generated/registries/{registry_group}.nbt"),
        nbt_bytes,
    )
    .unwrap();
}

fn read_registry(path: PathBuf) -> JsonValue {
    let mut registry_map = Map::new();

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let dir_name = entry.file_name().into_string().unwrap();
            registry_map.insert(dir_name.clone(), read_registry(path));
        } else if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let file_name = entry
                .file_name()
                .into_string()
                .unwrap()
                .replace(".json", "");
            let mut file = File::open(&path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let json: JsonValue = serde_json::from_str(&contents).unwrap();
            registry_map.insert(file_name, json);
        }
    }

    JsonValue::Object(registry_map)
}

fn download_jar() {
    let jar = reqwest::blocking::get(
        "http://piston-data.mojang.com/v1/objects/95495a7f485eedd84ce928cef5e223b757d2f764/server.jar",
    ).unwrap().bytes().unwrap();
    fs::write("generated/jar/bundler.jar", jar).unwrap();
}
