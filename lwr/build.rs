extern crate byteorder;
use byteorder::{ReadBytesExt, LE};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

fn main() {
    let regions = [
        ("US915", "../serialized/US915.res7.h3idx"),
        ("EU868", "../serialized/EU868.res7.h3idx"),
        ("AS923_1", "../serialized/AS923-1.res7.h3idx"),
        ("AS923_2", "../serialized/AS923-2.res7.h3idx"),
        ("AS923_3", "../serialized/AS923-3.res7.h3idx"),
        ("AU915", "../serialized/AU915.res7.h3idx"),
        ("CN779", "../serialized/CN779.res7.h3idx"),
        ("EU433", "../serialized/EU433.res7.h3idx"),
        ("IN865", "../serialized/IN865.res7.h3idx"),
        ("KR920", "../serialized/KR920.res7.h3idx"),
        ("RU864", "../serialized/RU864.res7.h3idx"),
    ];

    let out_path: PathBuf = [std::env::var("OUT_DIR").unwrap().as_str(), "regions.rs"]
        .iter()
        .collect();
    let mut out = File::create(out_path).unwrap();

    gen_region_header(&regions, &mut out);

    for (region_name, h3_file_path) in &regions {
        let h3_file_path = Path::new(h3_file_path).canonicalize().unwrap();
        println!("cargo:rerun-if-changed={}", h3_file_path.to_str().unwrap());
        let h3_file = File::open(h3_file_path).unwrap();
        gen_region_array(region_name, h3_file, &mut out);
    }
}

fn gen_region_header(regions: &[(&str, &str)], out: &mut File) {
    write!(out, "pub static REGIONS: &[(&str, &[u64])] = &[\n").unwrap();
    for (region_name, _) in regions {
        write!(out, "    (\"{}\", {}),\n", region_name, region_name).unwrap()
    }
    write!(out, "];\n\n").unwrap();
}

fn gen_region_array(region_name: &str, mut h3_indices: File, out: &mut File) {
    write!(out, "pub static {}: &[u64] = &[\n", region_name).unwrap();
    while let Ok(h3_index) = h3_indices.read_u64::<LE>() {
        write!(out, "    {:#016x},\n", h3_index).unwrap()
    }
    write!(out, "];\n\n").unwrap();
}
