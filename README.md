# s2-tilejson ![GitHub Actions Workflow Status][test-workflow] [![npm][npm-image]][npm-url] [![crate][crate-image]][crate-url] [![downloads][downloads-image]][downloads-url] [![bundle][bundle-image]][bundle-url] [![docs-ts][docs-ts-image]][docs-ts-url] [![docs-rust][docs-rust-image]][docs-rust-url] ![doc-coverage][doc-coverage-image] ![code-coverage][code-coverage-image] [![Discord][discord-image]][discord-url]

[test-workflow]: https://img.shields.io/github/actions/workflow/status/Open-S2/s2-tilejson/test.yml?logo=github
[npm-image]: https://img.shields.io/npm/v/s2-tilejson.svg?logo=npm&logoColor=white
[npm-url]: https://npmjs.org/package/s2-tilejson
[crate-image]: https://img.shields.io/crates/v/s2-tilejson.svg?logo=rust&logoColor=white
[crate-url]: https://crates.io/crates/s2-tilejson
[bundle-image]: https://img.shields.io/bundlejs/size/s2-tilejson
[bundle-url]: https://bundlejs.com/?q=s2-tilejson&treeshake=%5B*%5D
[downloads-image]: https://img.shields.io/npm/dm/s2-tilejson.svg
[downloads-url]: https://www.npmjs.com/package/s2-tilejson
[docs-ts-image]: https://img.shields.io/badge/docs-typescript-yellow.svg
[docs-ts-url]: https://open-s2.github.io/s2-tilejson/
[docs-rust-image]: https://img.shields.io/badge/docs-rust-yellow.svg
[docs-rust-url]: https://docs.rs/s2-tilejson
[doc-coverage-image]: https://raw.githubusercontent.com/Open-S2/s2-tilejson/master/assets/doc-coverage.svg
[code-coverage-image]: https://raw.githubusercontent.com/Open-S2/s2-tilejson/master/assets/code-coverage.svg
[discord-image]: https://img.shields.io/discord/953563031701426206?logo=discord&logoColor=white
[discord-url]: https://discord.opens2.com

## About

TileJSON is a mostly [backwards-compatible](https://github.com/mapbox/tilejson-spec) open standard for representing map metadata.

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
metaBuilder.setType('vector') // 'vector' | 'json' | 'raster' | 'raster-dem' | 'sensor' | 'markers'
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
# bacon
bacon coverage # or type `l` inside the tool
```
