import fs from 'fs';
import path from 'path';
// TODO: use https://github.com/quicktype/quicktype#installation instead
import { compileFromFile } from 'json-schema-to-typescript';

const cddaTypingsFolder = path.join(__dirname, '..', 'src', 'types', 'cdda');
const fileNames = fs.readdirSync(cddaTypingsFolder);
const jsonSchemaFilePaths = fileNames.filter((fileName) => fileName.endsWith('.json')).map((fileName) => path.join(cddaTypingsFolder, fileName));

void jsonSchemaFilePaths.map(
  async (fileName) => await compileFromFile(fileName).then((tsContent) => fs.writeFileSync(fileName.replace('.json', '.ts'), tsContent)),
);
