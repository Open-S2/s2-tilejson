import { MetadataBuilder } from '../src';
import { expect, test } from 'bun:test';

import type { BBox, LayerMetaData, Metadata, Shape } from '../src';

test('basic metadata', () => {
  const metaBuilder = new MetadataBuilder();

  // on initial use be sure to update basic metadata:
  metaBuilder.setName('OSM');
  metaBuilder.setDescription('A free editable map of the whole world.');
  metaBuilder.setVersion('1.0.0');
  metaBuilder.setScheme('fzxy'); // 'fzxy' | 'tfzxy' | 'xyz' | 'txyz' | 'tms'
  metaBuilder.setType('vector'); // 'vector' | 'json' | 'raster' | 'raster-dem' | 'sensor' | 'markers'
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

  // as you build tiles, add the tiles metadata:
  const lonLatBoundsForTile: BBox = [-120, -7, 44, 72];
  // WM:
  metaBuilder.addTileWM(0, 0, 0, [-60, -20, 5, 60]);
  // S2:
  metaBuilder.addTileS2(1, 5, 22, 37, lonLatBoundsForTile);

  // finally to get the resulting metadata:
  const resultingMetadata: Metadata = metaBuilder.commit();

  expect(resultingMetadata).toEqual({
    attributions: {
      OpenStreetMap: 'https://www.openstreetmap.org/copyright/',
    },
    bounds: {
      '0': [0, 0, 0, 0],
    },
    center: {
      lat: 26,
      lon: -38,
      zoom: NaN,
    },
    description: 'A free editable map of the whole world.',
    encoding: 'none',
    faces: [0, 1],
    facesbounds: {
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
        drawTypes: [2],
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
    maxzoom: -Infinity,
    minzoom: Infinity,
    name: 'OSM',
    s2tilejson: '1.0.0',
    scheme: 'fzxy',
    tilestats: {
      '0': {
        total: 0,
      },
      '1': {
        total: 1,
      },
      '2': {
        total: 0,
      },
      '3': {
        total: 0,
      },
      '4': {
        total: 0,
      },
      '5': {
        total: 0,
      },
      total: 2,
    },
    type: 'vector',
    vector_layers: [
      {
        description: undefined,
        id: 'water_lines',
        maxzoom: 13,
        minzoom: 0,
      },
    ],
    version: '1.0.0',
  });
});
