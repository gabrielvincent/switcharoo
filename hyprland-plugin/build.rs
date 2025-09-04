use std::fs::read_dir;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::{env, fs, fs::File, io, path::Path};
use zip::ZipWriter;
use zip::write::FileOptions;

fn include_plugin() {
    let prepare_dir = combine();

    let out_dir = env::var("OUT_DIR").unwrap();
    let zip_path = Path::new(&out_dir).join("plugin.zip");

    let file = File::create(&zip_path).expect("Failed to create zip file");
    let mut zip = ZipWriter::new(&file);
    let options: FileOptions<()> = FileOptions::default()
        .compression_method(zip::CompressionMethod::Zstd)
        .compression_level(None)
        .unix_permissions(0o755);
    let mut buffer = Vec::new();
    for file in read_dir(prepare_dir)
        .expect("Failed to read prepare dir")
        .flatten()
    {
        // we can use the name as we dont allow for folders here
        zip.start_file(file.file_name().to_string_lossy(), options)
            .expect("Failed to start file in zip");
        let mut f = File::open(file.path()).expect("Failed to open file");
        f.read_to_end(&mut buffer).expect("Failed to read file");
        zip.write_all(&buffer).expect("Failed to write file to zip");
        buffer.clear();
    }
    zip.finish().expect("Failed to finish zip");
}

fn combine() -> PathBuf {
    let out_dir = env::var("OUT_DIR").unwrap();
    let srcs_dir = Path::new("plugin/src");
    let prepare_dir = Path::new(&out_dir).join("prepare");

    fs::create_dir_all(&prepare_dir).expect("Failed to create prepare dir");
    // Combine all source files into one for easier compilation
    let all_cpp_path = prepare_dir.join("all.cpp");
    let mut all_cpp = File::create(&all_cpp_path).expect("Failed to create all.cpp");

    let cpp_files = read_dir(srcs_dir)
        .expect("Failed to read srcs dir")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "cpp"));
    let header_files = read_dir(srcs_dir)
        .expect("Failed to read srcs dir")
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().extension().map_or(false, |ext| ext == "h")
                && entry.path().file_name() != Some("defs-test.h".as_ref())
        });

    for entry in cpp_files {
        let src_path = entry.path();
        let file_name = src_path.file_name().unwrap();
        all_cpp
            .write_fmt(format_args!("\n\n// {} \n", file_name.to_string_lossy()))
            .expect("Failed to write to all.cpp");
        let mut src_file = File::open(&src_path).expect("Failed to open source file");
        io::copy(&mut src_file, &mut all_cpp).expect("Failed to copy source file to all.cpp");
    }
    for header in header_files {
        let dest_path = prepare_dir.join(header.file_name());
        fs::copy(header.path(), dest_path).expect("Failed to copy header file");
    }
    all_cpp.flush().expect("Failed to flush all.cpp");
    all_cpp.sync_all().expect("Failed to sync all.cpp");
    prepare_dir
}

fn main() {
    include_plugin();
    println!("cargo:rerun-if-changed=plugin/src/*");
}
