use project_root::get_project_root;
use std::{
  fs::write,
  path::{Path, PathBuf},
};
use schemars::{schema_for, JsonSchema};

/**
 * convert serde to jsonschema: https://imfeld.dev/writing/generating_typescript_types_from_rust
 *  with way to optimize
 * convert jsonschema to ts: https://github.com/bcherny/json-schema-to-typescript
*/

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
  generate::<tileset_json::CDDATileSetConfigWithCache>(Path::join(project_root, "../src/types/cdda/tileset.json"));
  generate::<mapgen_json::CDDAMapgenWithCache>(Path::join(project_root, "../src/types/cdda/mapgen.json"));
  generate::<palette_json::CDDAPaletteArray>(Path::join(project_root, "../src/types/cdda/palette.json"));
  generate::<furniture_json::CDDAFurnArray>(Path::join(project_root, "../src/types/cdda/furniture.json"));
}
