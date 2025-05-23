import Ajv from 'ajv';
import {
  DrawType,
  MetadataBuilder,
  S2TileJSONSchema,
  ShapeSchema,
  TileJSONSchema,
  toMetadata,
} from '../src';
import { expect, test } from 'bun:test';

import type { LayerMetaData, MapboxTileJSONMetadata, Metadata, Shape } from '../src';

test('basic metadata', () => {
  const metaBuilder = new MetadataBuilder();

  // on initial use be sure to update basic metadata:
  metaBuilder.setName('OSM');
  metaBuilder.setDescription('A free editable map of the whole world.');
  metaBuilder.setVersion('1.0.0');
  metaBuilder.setScheme('fzxy'); // 'fzxy' | 'tfzxy' | 'xyz' | 'txyz' | 'tms'
  metaBuilder.setType('vector'); // 'vector' | 'json' | 'raster' | 'raster-dem' | 'sensor' | 'markers'
  metaBuilder.setExtension('pbf');
  metaBuilder.setEncoding('none'); // 'gz' | 'br' | 'none'
  metaBuilder.addAttribution('OpenStreetMap', 'https://www.openstreetmap.org/copyright/');

  // Vector Specific: add layers based on how you want to parse data from a source:

  const layer: LayerMetaData = {
    minzoom: 0,
    maxzoom: 13,
    drawTypes: [2],
    shape: {
      class: 'string',
      offset: 'f64',
      info: {
        name: 'string',
        value: 'i64',
      },
    } as Shape,
    mShape: undefined,
  };
  metaBuilder.addLayer('water_lines', layer);

  // WM:
  metaBuilder.addTileWM(0, 0, 0, [-60, -20, 5, 60]);
  // S2:
  metaBuilder.addTileS2(1, 5, 22, 37, [-120, -7, 44, 72]);

  // finally to get the resulting metadata:
  const resultingMetadata: Metadata = metaBuilder.commit();

  expect(resultingMetadata).toEqual({
    attributions: {
      OpenStreetMap: 'https://www.openstreetmap.org/copyright/',
    },
    bounds: [-120, -20, 44, 72],
    wmbounds: {
      '0': [0, 0, 0, 0],
    },
    centerpoint: {
      lat: 26,
      lon: -38,
      zoom: 6,
    },
    description: 'A free editable map of the whole world.',
    encoding: 'none',
    extension: 'pbf',
    faces: [0, 1],
    s2bounds: {
      '0': {},
      '1': {
        '5': [22, 37, 22, 37],
      },
      '2': {},
      '3': {},
      '4': {},
      '5': {},
    },
    layers: {
      water_lines: {
        drawTypes: [DrawType.Lines],
        mShape: undefined,
        maxzoom: 13,
        minzoom: 0,
        shape: {
          class: 'string',
          info: {
            name: 'string',
            value: 'i64',
          },
          offset: 'f64',
        },
      },
    },
    maxzoom: 13,
    minzoom: 0,
    name: 'OSM',
    s2tilejson: '1.0.0',
    scheme: 'fzxy',
    tilestats: {
      '0': 0,
      '1': 1,
      '2': 0,
      '3': 0,
      '4': 0,
      '5': 0,
      total: 2,
    },
    type: 'vector',
    vector_layers: [
      {
        description: undefined,
        id: 'water_lines',
        maxzoom: 13,
        minzoom: 0,
        fields: {},
      },
    ],
    version: '1.0.0',
  });
});

test('Mapbox Metadata', () => {
  const mapboxSpec: MapboxTileJSONMetadata = {
    tilejson: '3.0.0',
    name: 'OpenStreetMap',
    description: 'A free editable map of the whole world.',
    version: '1.0.0',
    attribution: "<a href='https://openstreetmap.org'>OSM contributors</a>",
    scheme: 'xyz',
    tiles: [
      'https://a.tile.custom-osm-tiles.org/{z}/{x}/{y}.mvt',
      'https://b.tile.custom-osm-tiles.org/{z}/{x}/{y}.mvt',
      'https://c.tile.custom-osm-tiles.org/{z}/{x}/{y}.mvt',
    ],
    minzoom: 0,
    maxzoom: 18,
    bounds: [-180, -85, 180, 85],
    fillzoom: 6,
    something_custom: 'this is my unique field',
    vector_layers: [
      {
        id: 'telephone',
        fields: {
          phone_number: 'the phone number',
          payment: 'how to pay',
        },
      },
      {
        id: 'bicycle_parking',
        fields: {
          type: 'the type of bike parking',
          year_installed: 'the year the bike parking was installed',
        },
      },
      {
        id: 'showers',
        fields: {
          water_temperature: 'the maximum water temperature',
          wear_sandles: 'whether you should wear sandles or not',
          wheelchair: 'is the shower wheelchair friendly?',
        },
      },
    ],
  };

  const s2Spec = toMetadata(mapboxSpec);
  expect(s2Spec).toEqual({
    attributions: { 'OSM contributors': 'https://openstreetmap.org' },
    wmbounds: {},
    centerpoint: {
      lat: 0,
      lon: 0,
      zoom: 0,
    },
    description: 'A free editable map of the whole world.',
    encoding: 'none',
    extension: 'tile',
    faces: [0],
    s2bounds: {
      '0': {},
      '1': {},
      '2': {},
      '3': {},
      '4': {},
      '5': {},
    },
    layers: {},
    maxzoom: 18,
    minzoom: 0,
    name: 'OpenStreetMap',
    s2tilejson: '1.0.0',
    scheme: 'xyz',
    type: 'vector',
    vector_layers: [
      {
        fields: {
          payment: 'how to pay',
          phone_number: 'the phone number',
        },
        id: 'telephone',
      },
      {
        fields: {
          type: 'the type of bike parking',
          year_installed: 'the year the bike parking was installed',
        },
        id: 'bicycle_parking',
      },
      {
        fields: {
          water_temperature: 'the maximum water temperature',
          wear_sandles: 'whether you should wear sandles or not',
          wheelchair: 'is the shower wheelchair friendly?',
        },
        id: 'showers',
      },
    ],
    version: '1.0.0',
    // old spec
    bounds: [-180, -85, 180, 85],
    attribution: "<a href='https://openstreetmap.org'>OSM contributors</a>",
    fillzoom: 6,
    tilejson: '3.0.0',
    tiles: [
      'https://a.tile.custom-osm-tiles.org/{z}/{x}/{y}.mvt',
      'https://b.tile.custom-osm-tiles.org/{z}/{x}/{y}.mvt',
      'https://c.tile.custom-osm-tiles.org/{z}/{x}/{y}.mvt',
    ],
    // random values
    something_custom: 'this is my unique field',
  });
});

test('Minimal metadata', () => {
  const mini = {
    bounds: [-180, -85, 180, 85],
    name: 'Mapbox Satellite',
    scheme: 'xyz',
    format: 'zxy',
    type: 'raster',
    extension: 'webp',
    encoding: 'none',
    minzoom: 0,
    maxzoom: 3,
  };
  const s2Spec = toMetadata(mini as unknown as MapboxTileJSONMetadata);
  expect(s2Spec).toEqual({
    attributions: {},
    wmbounds: {},
    centerpoint: {
      lat: 0,
      lon: 0,
      zoom: 0,
    },
    description: '',
    encoding: 'none',
    extension: 'pbf',
    faces: [0],
    s2bounds: {
      '0': {},
      '1': {},
      '2': {},
      '3': {},
      '4': {},
      '5': {},
    },
    layers: {},
    maxzoom: 3,
    minzoom: 0,
    name: 'Mapbox Satellite',
    s2tilejson: '1.0.0',
    scheme: 'xyz',
    type: 'vector',
    vector_layers: [],
    version: '1.0.0',
    // old spec stuff
    bounds: [-180, -85, 180, 85],
    // random dead code
    format: 'zxy',
  });
});

// test('validate the s2tilejson example', async () => {
//   const ajv = new Ajv();
//   const validate = ajv.compile(S2TileJSONSchema);

//   const exampleJSON = await Bun.file(
//     `${__dirname}/../s2-tilejson-spec/1.0.0/examples/osm-s2.json`,
//   ).json();

//   expect(validate(exampleJSON)).toBeTrue();
// });

test('validate the tilejson example', async () => {
  const ajv = new Ajv();
  const validate = ajv.compile(TileJSONSchema);

  const exampleJSON = await Bun.file(
    `${__dirname}/../s2-tilejson-spec/1.0.0/examples/backwards.osm.json`,
  ).json();

  expect(validate(exampleJSON)).toBeTrue();

  // s2
  const exampleJSON_S2 = toMetadata(exampleJSON);

  const s2Validate = ajv.compile(S2TileJSONSchema);
  expect(s2Validate(exampleJSON_S2)).toBeTrue();
});

test('validate a shape', () => {
  const ajv = new Ajv();
  const validate = ajv.compile(ShapeSchema);

  const shape = {
    class: 'string',
    offset: 'f64',
    info: {
      name: 'string',
      value: 'i64',
    },
  };

  expect(validate(shape)).toBeTrue();
});
