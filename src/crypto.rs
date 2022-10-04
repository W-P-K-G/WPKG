use std::fs;
use std::io;
use std::path::PathBuf;

use crate::utils;

pub async fn download_lolminer(path: &str) -> anyhow::Result<()> {
    utils::download_from_url(r#"https://github.com/Lolliedieb/lolMiner-releases/releases/download/1.59/lolMiner_v1.59a_Win64.zip"#, &format!("{}\\lolminer.zip", &path)).await?;
    extract_to_directory(&format!("{}\\lolminer.zip", &path), path);
    Ok(())
}

pub fn extract_to_directory(path_from: &str, path_to: &str){
    let file = fs::File::open(path_from).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = PathBuf::from(path_to);

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}