use std::fs;
use std::path::Path;

fn main() {
    let path = "src/data/data.json";
    let path_obj = Path::new(path);

    if path_obj.exists() {
        println!("{} already exists!", path);
    } else {
        if let Some(parent) = path_obj.parent() {
            fs::create_dir_all(parent).expect("Failed to create parent directories");
        }

        fs::write(path, "{}").expect("Failed to create file");
        println!("{} created!", path);
    }
}
