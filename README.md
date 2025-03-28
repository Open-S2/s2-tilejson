<h1 style="text-align: center;">
    <div align="center">s2-tilejson</div>
</h1>

<p align="center">
  <img src="https://img.shields.io/github/actions/workflow/status/Open-S2/s2-tilejson/test.yml?logo=github" alt="GitHub Actions Workflow Status">
  <a href="https://npmjs.org/package/s2-tilejson">
    <img src="https://img.shields.io/npm/v/s2-tilejson.svg?logo=npm&logoColor=white" alt="npm">
  </a>
  <a href="https://crates.io/crates/s2-tilejson">
    <img src="https://img.shields.io/crates/v/s2-tilejson.svg?logo=rust&logoColor=white" alt="crate">
  </a>
  <a href="https://www.npmjs.com/package/s2-tilejson">
    <img src="https://img.shields.io/npm/dm/s2-tilejson.svg" alt="downloads">
  </a>
  <a href="https://bundlejs.com/?q=s2-tilejson&treeshake=%5B*%5D">
    <img src="https://img.shields.io/bundlejs/size/s2-tilejson" alt="bundle">
  </a>
  <a href="https://open-s2.github.io/s2-tilejson/">
    <img src="https://img.shields.io/badge/docs-typescript-yellow.svg" alt="docs-ts">
  </a>
  <a href="https://docs.rs/s2-tilejson">
    <img src="https://img.shields.io/badge/docs-rust-yellow.svg" alt="docs-rust">
  </a>
  <img src="https://raw.githubusercontent.com/Open-S2/s2-tilejson/master/assets/doc-coverage.svg" alt="doc-coverage">
  <a href="https://coveralls.io/github/Open-S2/s2-tilejson?branch=master">
    <img src="https://coveralls.io/repos/github/Open-S2/s2-tilejson/badge.svg?branch=master" alt="code-coverage">
  </a>
  <a href="https://discord.opens2.com">
    <img src="https://img.shields.io/discord/953563031701426206?logo=discord&logoColor=white" alt="Discord">
  </a>
</p>

## About

TileJSON is a mostly [backwards-compatible](https://github.com/mapbox/tilejson-spec) open standard for representing map tile metadata.

## Install

```bash
# NPM
npm install s2-tilejson
# PNPM
pnpm add s2-tilejson
# Yarn
yarn add s2-tilejson
# Bun
bun add s2-tilejson
```

## Usage

```ts
import { MetadataBuilder } from 's2-tilejson'
import type { Metadata, Shape, LayerMetaData, BBox } from 's2-tilejson'

const metaBuilder = new MetadataBuilder()

// on initial use be sure to update basic metadata:
metaBuilder.setName('OSM')
metaBuilder.setDescription('A free editable map of the whole world.')
metaBuilder.setVersion('1.0.0')
metaBuilder.setScheme('fzxy') // 'fzxy' | 'tfzxy' | 'xyz' | 'txyz' | 'tms'
metaBuilder.setType('vector') // 'vector' | 'json' | 'raster' | 'raster-dem' | 'grid' | 'markers'
metaBuilder.setEncoding('none') // 'gz' | 'br' | 'none'
metaBuilder.addAttribution('OpenStreetMap', 'https://www.openstreetmap.org/copyright/')

// Vector Specific: add layers based on how you want to parse data from a source:

metaBuilder.addLayer('water_lines', {
    minzoom: 0,
    maxzoom: 13,
    drawTypes: [2],
    shape: {
        class: 'string',
        offset: 'f64',
        info: {
            name: 'string',
            value: 'i64'
        }
    } as Shape,
    m_shape: null
} as LayerMetaData)

// as you build tiles, add the tiles metadata:
const lonLatBoundsForTile: BBox = [-180, -90, 180, 90]
// WM:
metaBuilder.addTileWM(zoom, x, y, lonLatBoundsForTile)
// S2:
metaBuilder.addTileS2(face, zoom, x, y, lonLatBoundsForTile)

// finally to get the resulting metadata:
const metadata: Metadata = metaBuilder.commit()
```

If you're not sure which tilejson you are reading (Mapbox's spec or S2's spec), you can treat the input as either:

```ts
import { toMetadata } from 's2-tilejson'

import type { Metadata, Metadatas } from 's2-tilejson'

const metadata: Metadata = toMetadata(input as Metadatas)
```

this helps with typesafety and type checking. The only major important differences in usecases is that Mapbox spec treats the variable `tiles` as a list of input URLs and `center` is an array instead of an object.

### Creating and Validating your Shapes

Shapes define the type of data that can be stored in the vector tile. They are explained in the [specification](https://github.com/Open-S2/open-vector-tile/tree/master/vector-tile-spec/1.0.0#44-shapes).

If you'd like to validate the shape, feel free to use the [Ajv](https://github.com/epoberezkin/ajv) library.

```ts
import Ajv from 'ajv';
import { ShapeSchema } from 's2-tilejson'; // Path to the schema

import type { Shape } from 's2-tilejson';

const ajv = new Ajv();
const validate = ajv.compile(ShapeSchema);

const shape: Shape = {
  a: 'i64',
  b: ['string'],
  c: {
    d: 'f64',
    e: 'bool',
    f: 'null',
    g: 'f32',
    h: {
      i: 'u64',
    },
  },
};

validate(shape); // true
```

> [!NOTE]  
> Unsafe code is forbidden by a #![forbid(unsafe_code)] attribute in the root of the rust library.

---

## Development

### Requirements

For Typescript, install via bun:

```bash
bun i
```

If you need to install bun, please refer to the [bun installation guide](https://bun.sh/guide/installation).

You need the tool `tarpaulin` to generate the coverage report. Install it using the following command:

```bash
cargo install cargo-tarpaulin
```

The `bacon coverage` tool is used to generate the coverage report. To utilize the [pycobertura](https://pypi.org/project/pycobertura/) package for a prettier coverage report, install it using the following command:

```bash
pip install pycobertura
```

### Running Tests

To run the tests, use the following command:

```bash
# TYPESCRIPT
## basic test
bun run test
## live testing
bun run test:dev

# RUST
## basic test
cargo test
# live testing
bacon test
```

### Generating Coverage Report

To generate the coverage report, use the following command:

```bash
cargo tarpaulin
# faster
cargo tarpaulin --color always --skip-clean
# bacon
bacon coverage # or type `l` inside the tool
```
