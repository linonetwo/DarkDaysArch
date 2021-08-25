use project_root::get_project_root;
use std::{
  fs::write,
  path::{Path, PathBuf},
};
use typescript_type_def::{write_definition_file, TypeDef};

#[path = "../src/types/furniture.rs"]
mod furniture_json;
#[path = "../src/types/mapgen.rs"]
mod mapgen_json;
#[path = "../src/types/palette.rs"]
mod palette_json;

fn generate<T>(path: PathBuf)
where
  T: ?Sized + TypeDef, // Sized or ?Sized are both ok, click https://zhuanlan.zhihu.com/p/21820917 to learn why
{
  let mut buf = Vec::new();
  write_definition_file::<_, T>(&mut buf, Default::default()).unwrap();
  let generated_ts_content = String::from_utf8(buf).unwrap();
  write(path, generated_ts_content).expect("can not write ts file");
}

fn main() {
  let project_root = &get_project_root().unwrap();
  generate::<mapgen_json::CDDAMapgenWithCache>(Path::join(project_root, "../src/types/cdda/mapgen.ts"));
  generate::<palette_json::CDDAPaletteArray>(Path::join(project_root, "../src/types/cdda/palette.ts"));
  generate::<furniture_json::CDDAFurnArray>(Path::join(project_root, "../src/types/cdda/furniture.ts"));
}
