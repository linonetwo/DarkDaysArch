use project_root::get_project_root;
use std::{
  fs::write,
  path::{Path, PathBuf},
};
use schemars::{schema_for, JsonSchema};

#[path = "../src/types/tileset.rs"]
mod tileset_json;
#[path = "../src/types/furniture.rs"]
mod furniture_json;
#[path = "../src/types/mapgen.rs"]
mod mapgen_json;
#[path = "../src/types/palette.rs"]
mod palette_json;

fn generate<T>(path: PathBuf)
where
  T: ?Sized + JsonSchema, // Sized or ?Sized are both ok, click https://zhuanlan.zhihu.com/p/21820917 to learn why
{
  let schema = schema_for!(T);
  let output = serde_json::to_string_pretty(&schema).unwrap();
  write(path, output).expect("can not write json-schema file")
}

fn main() {
  let project_root = &get_project_root().unwrap();
  generate::<tileset_json::CDDATileSetConfigWithCache>(Path::join(project_root, "../src/types/tileset.json"));
  generate::<mapgen_json::CDDAMapgenWithCache>(Path::join(project_root, "../src/types/mapgen.json"));
  generate::<palette_json::CDDAPaletteArray>(Path::join(project_root, "../src/types/palette.json"));
  generate::<furniture_json::CDDAFurnArray>(Path::join(project_root, "../src/types/furniture.json"));
}
