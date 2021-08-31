use project_root::get_project_root;
use schemars::{schema_for, JsonSchema};
use std::{
  fs::write,
  path::{Path, PathBuf},
};

/**
 * convert serde to jsonschema: https://imfeld.dev/writing/generating_typescript_types_from_rust
 *  with way to optimize
 * convert jsonschema to ts: https://github.com/bcherny/json-schema-to-typescript
*/
use data::types::{furniture, mapgen, palette, terrain, tileset, trap};

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
  generate::<tileset::CDDATileSetConfigWithCache>(Path::join(project_root, "../src/types/cdda/tileset.json"));
  generate::<mapgen::CDDAMapgenWithCache>(Path::join(project_root, "../src/types/cdda/mapgen.json"));
  generate::<palette::CDDAPaletteArray>(Path::join(project_root, "../src/types/cdda/palette.json"));
  generate::<furniture::CDDAFurnArray>(Path::join(project_root, "../src/types/cdda/furniture.json"));
  generate::<terrain::CDDATerrainArray>(Path::join(project_root, "../src/types/cdda/terrain.json"));
}
