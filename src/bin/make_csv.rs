mod utils;

use eyre::eyre;
use serde::Serialize;
use std::rc::Rc;
use utils::load_json::*;

#[derive(Debug, Serialize)]
struct RecordCSV {
    key_id: String,
    original_filename: String,
    main_image: String,
    main_webp: String,
}

fn main() -> eyre::Result<()> {
    let file_data = load("mainfile.json", JsonEnum::FromFile)?;
    let webp_data = load("webp_image.json", JsonEnum::FromJsonImage)?;
    let thumb_data = load("thumb.json", JsonEnum::FromJsonImage)?;
    let main_data = load("main_image.json", JsonEnum::FromJsonImage)?;

    let file_data = file_data.get_arr_file().ok_or_else(|| eyre!("error"))?;
    let webp_data = webp_data.get_arr_json().ok_or_else(|| eyre!("error"))?;
    let thumb_data = thumb_data.get_arr_json().ok_or_else(|| eyre!("error"))?;
    let main_data = main_data.get_arr_json().ok_or_else(|| eyre!("error"))?;

    let count_webp = Rc::new(webp_data);
    let count_thumb = Rc::new(thumb_data);
    let count_main = Rc::new(main_data);

    let len = file_data.len();

    let mut data_arr = Vec::with_capacity(len);

    for data in &file_data {
        let data_webp = count_webp
            .iter()
            .find(|webp| webp.original_filename == data.hash_filename.to_string())
            .ok_or_else(|| eyre!("webp_data not found"))?;

        println!("data_webp : {:#?}", &data_webp);

        let data_main = count_main
            .iter()
            .find(|mainjson| mainjson.original_filename == data.hash_filename.to_string())
            .ok_or_else(|| eyre!("main_data not found"))?;

        let data_thumb = count_thumb
            .iter()
            .find(|thumb| thumb.original_filename == data.original_filename)

        let record_csv = RecordCSV {
            key_id: data_thumb.asset_id.clone(),
            original_filename: data_thumb.original_filename.clone(),
            main_image: data_main.url.clone(),
            main_webp: data_webp.url.clone(),
        };

        data_arr.push(record_csv);
    }

    println!("result : {:#?}", data_arr);

    Ok(())
}

