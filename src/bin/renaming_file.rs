use eyre::eyre;
use std::{
    env,
    fs::{self, rename},
    path::Path,
};

fn main() -> eyre::Result<()> {
    let arg = env::args().collect::<Vec<String>>();

    let folder_path = arg.get(1).expect("must specify path");
    println!("folder path : {}", folder_path);

    let folder_path = Path::new(folder_path);

    let file_names = fs::read_dir(folder_path).expect("error reading folder");

    for filename in file_names {
        let namefile = filename.expect("not a file");

        let file_path = namefile.path();

        if file_path.is_file() {
            let slice_name = file_path.file_name().ok_or_else(|| eyre!("error os str"))?;
            let slice_name_str = slice_name.to_str().ok_or_else(|| eyre!("error to str"))?;

            let parted_name = slice_name_str.split('.').collect::<Vec<_>>();
            let mut parted_name = parted_name[0].split('-').collect::<Vec<_>>();
            parted_name.pop();
            let result_name = parted_name.join("-");
            let result_name = format!("{result_name}.png");
            println!(" result_name {}", &result_name);

            let result_filename = folder_path.join(result_name);

            rename(file_path, result_filename)?;
        }
    }

    Ok(())
}

