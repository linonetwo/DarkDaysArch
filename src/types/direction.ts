export enum Direction {
  down = '↓',
  left = '←',
  right = '→',
  up = '↑',
}

/**
 * "bg": ["t_wall_n", "t_wall_e", "t_wall_s", "t_wall_w"]
 * @docs https://github.com/CleverRaven/Cataclysm-DDA/blob/master/doc/TILESET.md
 */
export const directionToIndex = { [Direction.up]: 0, [Direction.right]: 1, [Direction.down]: 2, [Direction.left]: 3 };
