use serde_json::value::ToJson;
use serde_json::{Map, Value as JsonValue};
use std::fs::File;
use std::io::Read;
use std::path::Path;
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

fn main() {
    // 1.21.10

    fs::create_dir_all("generated/bundler-jar").unwrap();
    fs::create_dir_all("generated/jar").unwrap();
    fs::create_dir_all("generated/registries").unwrap();

    println!("Downloading server jar from mojang.com...");
    download_jar();

    println!("Extracting server jar from downloaded bundle jar...");
    let mut root_zip =
        ZipArchive::new(File::open("generated/jar/bundled-server.jar").unwrap()).unwrap();
    root_zip.extract("generated/bundler-jar/").unwrap();

    println!("Extracting files from inner server jar...");
    let mut bundler_zip = ZipArchive::new(
        File::open("generated/bundler-jar/META-INF/versions/1.21.10/server-1.21.10.jar").unwrap(),
    )
    .unwrap();
    bundler_zip.extract("generated/jar").unwrap();

    println!("Copying registry files to generated/registries...");
    copy_dir_all("generated/jar/data/minecraft/", "generated/registries/").unwrap();

    println!("Concatenating registries...");
    let mut map = Map::new();
    map.insert(
        "".to_owned(),
        read_registry(String::from("generated/registries/")).unwrap(),
    );
    let json = map.to_json().unwrap();
    fs::write("generated/registries.json", json.to_string()).unwrap();
}

fn read_registry(path: String) -> io::Result<JsonValue> {
    let mut registry_map = Map::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let dir_name = entry.file_name().into_string().unwrap();
            registry_map.insert(
                dir_name.clone(),
                read_registry(path.to_str().unwrap().to_owned())?,
            );
        } else if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let file_name = entry.file_name().into_string().unwrap();
            let mut file = File::open(&path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let json: JsonValue = serde_json::from_str(&contents).unwrap();
            registry_map.insert(file_name, json);
        }
    }

    Ok(JsonValue::Object(registry_map))
}

fn download_jar() {
    let jar = reqwest::blocking::get(
        "http://piston-data.mojang.com/v1/objects/95495a7f485eedd84ce928cef5e223b757d2f764/server.jar",
    ).unwrap().bytes().unwrap();
    fs::write("generated/jar/bundled-server.jar", jar.clone()).unwrap();
}
