import { Texture } from 'pixi.js';

import { Direction } from 'src/types/direction';
import { TileLayers } from 'src/types/tileset';

export interface ITileTextures {
  bg?: Texture;
  fg?: Texture;
  tileHeight: number;
  tileWidth: number;
}

class TextureManager {
  private textures: Record<string, Texture> = {};

  add(tileName: string, texture: Texture): void {
    this.textures[tileName] = texture;
  }

  getOrCreateTexture(tileName: string, textureGenerationFunction: (tileName: string) => Texture): Texture {
    if (this.textures[tileName] === undefined) {
      this.textures[tileName] = textureGenerationFunction(tileName);
    }
    return this.textures[tileName];
  }

  public getTileCacheID(tileName: string, layer: TileLayers, direction?: Direction): string {
    return `${tileName}_${layer}${direction ?? ''}`;
  }
}

export const textureManager = new TextureManager();
