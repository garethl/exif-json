use std::env;
use std::process;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufReader;
use exif::Exif;
use std::collections::BTreeMap;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} image-file", args[0]);
        process::exit(1);
    }

    let path = PathBuf::from(&args[1]);
    match load_exif(&path) {
        Ok(exif) => {
            let hash = convert_to_hash(&exif);
            let json = serde_json::to_string_pretty(&hash);
            match json {
                Ok(j) => println!("{}", j),
                Err(err) => {
                    eprintln!("Error parsing {}: {}", path.display(), err);
                    process::exit(1);
                }
            }
        }
        Err(err) => {
            eprintln!("Error parsing {}: {}", path.display(), err);
            process::exit(1);
        }
    }
}

fn convert_to_hash(exif: &Exif) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for field in exif.fields() {
        let key = format!("{}", field.tag);
        let value = format!("{}", field.display_value().with_unit(exif));
        map.insert(key, value);
    }
    return map
}

fn load_exif(path: &Path) -> Result<Exif, exif::Error> {
    let file = File::open(path)?;
    let exif = exif::Reader::new().read_from_container(
        &mut BufReader::new(&file))?;
    Ok(exif)
}