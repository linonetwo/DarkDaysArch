use project_root::get_project_root;
use std::fs::write;
use std::path::Path;
use typescript_type_def::write_definition_file;

#[path = "../src/types/furniture.rs"]
mod furniture_json;
#[path = "../src/types/mapgen.rs"]
mod mapgen_json;
#[path = "../src/types/palette.rs"]
mod palette_json;

fn main() {
    // let mut buf = Vec::new();
    // write_definition_file::<_, mapgen_json::CDDAMapgenWithCache>(&mut buf, Default::default()).unwrap();
    // let generated_ts_content = String::from_utf8(buf).unwrap();
    // let ts_file_path = Path::join(&get_project_root().unwrap(), "../src/types/mapgen.ts");
    // // println!("{}", ts_file_path.into_os_string().into_string().unwrap());
    // write(ts_file_path, generated_ts_content).expect("can not write ts file");

    // let mut buf = Vec::new();
    // write_definition_file::<_, palette_json::CDDAPaletteArray>(&mut buf, Default::default()).unwrap();
    // let generated_ts_content = String::from_utf8(buf).unwrap();
    // let ts_file_path = Path::join(&get_project_root().unwrap(), "../src/types/palette.ts");
    // // println!("{}", ts_file_path.into_os_string().into_string().unwrap());
    // write(ts_file_path, generated_ts_content).expect("can not write ts file");

    let mut buf = Vec::new();
    write_definition_file::<_, furniture_json::CDDAFurnArray>(&mut buf, Default::default())
        .unwrap();
    let generated_ts_content = String::from_utf8(buf).unwrap();
    let ts_file_path = Path::join(
        &get_project_root().unwrap(),
        "../src/types/cdda/furniture.ts",
    );
    // println!("{}", ts_file_path.into_os_string().into_string().unwrap());
    write(ts_file_path, generated_ts_content).expect("can not write ts file");
}
