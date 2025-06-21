use std::env;
use walkdir::WalkDir;

fn main() {
    // Get path from command line or use current directory
    let args: Vec<String> = env::args().collect();
    let start_path = args.get(2).map_or(".", String::as_str);
    let extension = args.get(1);

    // Traverse directories
    for entry in WalkDir::new(start_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext.to_str() == Some(extension.unwrap_or(&String::new())) {
                println!("{}", path.display());
            }
        }
    }
}
