pub mod furniture;
pub mod mapgen;
pub mod overmap_terrain;
pub mod palette;
pub mod terrain;
pub mod tileset;
pub mod trap;

pub enum CDDA_JSON {
  Furniture(furniture::CDDAFurniture),
  Mapgen(mapgen::CDDAMapgen),
  OvermapTerrain(overmap_terrain::CDDAOvermapTerrain),
  Palette(palette::CDDAPalette),
  Terrain(terrain::CDDATerrain),
  Tileset(tileset::CDDATileSetConfig),
}
