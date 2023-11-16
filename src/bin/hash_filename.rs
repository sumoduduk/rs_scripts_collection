use eyre::eyre;
use serde::Serialize;
use std::collections::hash_map::DefaultHasher;
use std::env::args;
use std::fs::{self, rename, File};
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, Serialize)]
struct Record {
    original_filename: String,
    hash_filename: u64,
}

impl Record {
    fn new(original_filename: String, hash_filename: u64) -> Self {
        Self {
            original_filename,
            hash_filename,
        }
    }
}

fn make_json(arr: Vec<Record>) -> eyre::Result<()> {
    let now = SystemTime::now();
    let time_now = now.duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
    let jsonfile_name = format!("main-image-{}.json", time_now);

    let json_data = serde_json::to_string_pretty(&arr)?;
    let mut json_file = File::create(jsonfile_name)?;
    json_file.write_all(json_data.as_bytes())?;

    println!("save to json complete");
    Ok(())
}

fn main() -> eyre::Result<()> {
    let arg = args().collect::<Vec<String>>();

    let folder_path = arg.get(1).expect("must specify path");
    println!("folder path : {}", folder_path);

    let folder_path = Path::new(folder_path);

    let file_names = fs::read_dir(folder_path).expect("error reading folder");

    let mut arr_record = Vec::new();

    for filename in file_names {
        let namefile = filename.expect("not a file");

        let file_path = namefile.path();

        if file_path.is_file() {
            let slice_name = file_path.file_name().ok_or_else(|| eyre!("error os str"))?;

            let slice_name_str = slice_name.to_str().ok_or_else(|| eyre!("error to str"))?;

            let extension = file_path.extension().and_then(std::ffi::OsStr::to_str);

            match extension {
                Some("jpg") | Some("jpeg") | Some("png") | Some("webp") => {
                    let mut hasher = DefaultHasher::new();
                    slice_name_str.hash(&mut hasher);

                    let result_hash = hasher.finish();

                    let result_name = folder_path.join(format!("{}.png", result_hash));

                    let sliced = slice_name_str.split('.').collect::<Vec<_>>();
                    let record = Record::new(sliced[0].to_string(), result_hash);

                    arr_record.push(record);

                    match rename(&file_path, &result_name) {
                        Ok(_) => println!("success rename from {slice_name_str} to {result_hash}"),
                        Err(_) => println!("error rename"),
                    }
                }
                _ => (),
            }
        }
    }
    make_json(arr_record)?;

    Ok(())
}

