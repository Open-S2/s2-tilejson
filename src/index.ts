/** S2 Face */
export type Face = 0 | 1 | 2 | 3 | 4 | 5;

/** The Bounding box, whether the tile bounds or lon-lat bounds or whatever. */
export type BBox = [left: number, bottom: number, right: number, top: number];

/** 1: points, 2: lines, 3: polys, 4: points3D, 5: lines3D, 6: polys3D, 7: raster */
export const enum DrawType {
  /** Collection of points */
  Points = 1,
  /** Collection of lines */
  Lines = 2,
  /** Collection of polygons */
  Polys = 3,
  /** Collection of points in 3D */
  Points3D = 4,
  /** Collection of lines in 3D */
  Lines3D = 5,
  /** Collection of polygons in 3D */
  Polys3D = 6,
  /** Raster data */
  Raster = 7,
}

//? Shapes exist solely to deconstruct and rebuild objects.
//?
//? Shape limitations:
//? - all keys are strings.
//? - all values are either:
//? - - primitive types: strings, numbers (f32, f64, u64, i64), true, false, or null
//? - - sub types: an array of a shape or a nested object which is itself a shape
//? - - if the sub type is an array, ensure all elements are of the same type
//? The interfaces below help describe how shapes are built by the user.

import ShapeSchema from './shape.schema.json';
export { ShapeSchema };

/** Primitive types that can be found in a shape */
export type PrimitiveShapes = 'string' | 'f32' | 'f64' | 'u64' | 'i64' | 'bool' | 'null';

/** The Shape Object But the values can only be primitives */
export interface ShapePrimitive {
  [key: string]: PrimitiveShapes;
}

/**
 * Arrays may contain either a primitive or an object whose values are primitives
 */
export type ShapePrimitiveType = PrimitiveShapes | ShapePrimitive;

/**
 * Shape types that can be found in a shapes object.
 * Either a primitive, an array containing any type, or a nested shape.
 * If the type is an array, all elements must be the same type
 */
export type ShapeType = PrimitiveShapes | [ShapePrimitiveType] | Shape;

/** The Shape Object */
export interface Shape {
  [key: string]: ShapeType;
}

/** Each layer has metadata associated with it. Defined as blueprints pre-construction of vector data. */
export interface LayerMetaData {
  description?: string;
  minzoom: number;
  maxzoom: number;
  drawTypes: DrawType[];
  shape: Shape;
  mShape?: Shape;
}

/** Each layer has metadata associated with it. Defined as blueprints pre-construction of vector data. */
export interface LayersMetaData {
  [layer: string]: LayerMetaData;
}

/** Tilestats is simply a tracker to see where most of the tiles live */
export interface TileStatsMetadata {
  total: number;
  0: number;
  1: number;
  2: number;
  3: number;
  4: number;
  5: number;
}

/**
 * Attribution data is stored in an object.
 * The key is the name of the attribution, and the value is the link
 */
export type Attribution = Record<string, string>;

/** Track the S2 tile bounds of each face and zoom */
export interface FaceBounds {
  // facesbounds[face][zoom] = [...]
  0: { [zoom: number]: BBox };
  1: { [zoom: number]: BBox };
  2: { [zoom: number]: BBox };
  3: { [zoom: number]: BBox };
  4: { [zoom: number]: BBox };
  5: { [zoom: number]: BBox };
}

/** Track the WM tile bounds of each zoom */
export interface WMBounds {
  [zoom: number]: BBox;
}

/** Types of image extensions */
export type ImageExtensions =
  | 'raw'
  | 'png'
  | 'jpg'
  | 'jpeg'
  | 'jpe'
  | 'webp'
  | 'avif'
  | 'gif'
  | 'svg'
  | 'bmp'
  | 'tiff'
  | 'ico'
  | 'cur';

/** All supported extensions */
export type Extensions = 'geojson' | 'json' | 's2json' | 'pbf' | ImageExtensions | string;

/**
 * Check the source type of the layer.
 * If "overlay" then an old engine was used
 */
export type SourceType =
  | 'vector'
  | 'json'
  | 'raster'
  | 'raster-dem'
  | 'sensor'
  | 'markers'
  | 'overlay';

/** Store the encoding of the data */
export type Encoding = 'gz' | 'br' | 'none' | 'zstd';

/** Old spec tracks basic vector data */
export interface VectorLayer {
  /** Unique identifier of the layer */
  id: string;
  /** Description of the layer */
  description?: string;
  /** Minimum zoom level for the layer */
  minzoom?: number;
  /** Maximum zoom level for the layer */
  maxzoom?: number;
  /** Field metadata for the layer. */
  fields: Record<string, string>;
  /** Allow additional properties */
  [key: string]: unknown;
}

/**
 * Default S2 tile scheme is `fzxy`
 * Default Web Mercator tile scheme is `xyz`
 * Adding a t prefix to the scheme will change the request to be time sensitive
 * TMS is an oudated version that is not supported by s2maps-gpu
 */
export type Scheme = 'fzxy' | 'tfzxy' | 'xyz' | 'txyz' | 'tms';

/** Store where the center of the data lives */
export interface Center {
  /** the center longitude */
  lon: number;
  /** the center latitude */
  lat: number;
  /** the zoom level */
  zoom: number;
}

/**
 * # S2 TileJSON Metadata
 * Metadata describing a collection of S2 or WM tiles and how to access them.
 */
export interface Metadata {
  /** The version of the s2-tilejson spec */
  s2tilejson: string;
  /** The version of the data */
  version: string;
  /** The name of the data */
  name: string;
  /** The extension when requesting a tile */
  extension: Extensions;
  /** The scheme of the data */
  scheme: Scheme;
  /** The description of the data */
  description: string;
  /** The type of the data */
  type: SourceType;
  /** The encoding of the data */
  encoding?: Encoding;
  /** List of faces that have data */
  faces: Face[];
  /** WM Tile fetching bounds. Helpful to not make unecessary requests for tiles we know don't exist */
  bounds: WMBounds;
  /** S2 Tile fetching bounds. Helpful to not make unecessary requests for tiles we know don't exist */
  facesbounds: FaceBounds;
  /** minzoom at which to request tiles. [default=0] */
  minzoom: number;
  /** maxzoom at which to request tiles. [default=27] */
  maxzoom: number;
  /** The center of the data */
  center: Center;
  /** { ['human readable string']: 'href' } */
  attribution: Attribution;
  /** Track layer metadata */
  layers: LayersMetaData;
  /** Track tile stats for each face and total overall */
  tilestats?: TileStatsMetadata;
  /** Old spec, track basic layer metadata */
  vector_layers: VectorLayer[];
  /** Allow additional properties */
  [key: string]: unknown;
}

/** Builder class to help build the metadata */
export class MetadataBuilder {
  #lonLatBounds: BBox = [Infinity, Infinity, -Infinity, -Infinity];
  #faces: Set<Face> = new Set();
  #metadata: Metadata = {
    s2tilejson: '1.0.0',
    version: '1.0.0',
    name: 'default',
    scheme: 'fzxy',
    extension: 'pbf',
    description: 'Built with s2maps-cli',
    type: 'vector',
    encoding: 'none',
    faces: [],
    bounds: {},
    facesbounds: { 0: {}, 1: {}, 2: {}, 3: {}, 4: {}, 5: {} },
    minzoom: Infinity,
    maxzoom: -Infinity,
    center: { lon: 0, lat: 0, zoom: 0 },
    attribution: {},
    tilestats: { total: 0, 0: 0, 1: 0, 2: 0, 3: 0, 4: 0, 5: 0 },
    layers: {},
    vector_layers: [],
  };

  /** @returns - resultant metadata */
  commit(): Metadata {
    // set the center
    this.#updateCenter();
    // set the faces
    this.#metadata.faces = [...this.#faces];
    // return the result
    return this.#metadata;
  }

  /**
   * Set the name
   * @param name - name of the data
   */
  setName(name: string) {
    this.#metadata.name = name;
  }

  /**
   * Set the extension
   * @param extension - extension of the data
   */
  setExtension(extension: Extensions) {
    this.#metadata.extension = extension;
  }

  /**
   * Set the scheme of the data. [default=fzxy]
   * @param scheme - scheme of the data
   */
  setScheme(scheme: Scheme) {
    this.#metadata.scheme = scheme;
  }

  /**
   * Set the type of the data. [default=vector]
   * @param type - type of the data
   */
  setType(type: SourceType) {
    this.#metadata.type = type;
  }

  /**
   * Set the version of the data
   * @param version - version of the data
   */
  setVersion(version: string) {
    this.#metadata.version = version;
  }

  /**
   * Describe the data
   * @param description - description of the data
   */
  setDescription(description: string) {
    this.#metadata.description = description;
  }

  /**
   * Set the encoding of each vector tile. [default=none]
   * @param encoding - encoding of each tile
   */
  setEncoding(encoding: Encoding) {
    this.#metadata.encoding = encoding;
  }

  /**
   * Add an attribution
   * @param displayName - name of the attribution
   * @param href - link to the attribution
   */
  addAttribution(displayName: string, href: string) {
    this.#metadata.attribution[displayName] = href;
  }

  /**
   * Add the layer metadata
   * @param name - name of the layer
   * @param layer - layer metadata
   */
  addLayer(name: string, layer: LayerMetaData) {
    // add layer
    this.#metadata.layers[name] = layer;
    // add vector layer
    this.#metadata.vector_layers?.push({
      id: name,
      description: layer.description,
      minzoom: layer.minzoom,
      maxzoom: layer.maxzoom,
      fields: {},
    });
    // update minzoom and maxzoom
    if (layer.minzoom < this.#metadata.minzoom) this.#metadata.minzoom = layer.minzoom;
    if (layer.maxzoom > this.#metadata.maxzoom) this.#metadata.maxzoom = layer.maxzoom;
  }

  /**
   * Add the WM tile metadata
   * @param zoom - zoom of the tile
   * @param x - x position of the tile
   * @param y - y position of the tile
   * @param llBounds - the lon-lat bounds of the tile
   */
  addTileWM(zoom: number, x: number, y: number, llBounds: BBox) {
    const metadata = this.#metadata;
    // update tile stats
    if (metadata.tilestats !== undefined) metadata.tilestats.total++;
    this.#faces.add(0);
    this.#addBoundsWM(zoom, x, y);
    // update lon lat
    this.#updateLonLatBounds(llBounds);
  }

  /**
   * Add the S2 tile metadata
   * @param face - face of the tile
   * @param zoom - zoom of the tile
   * @param x - x position of the tile
   * @param y - y position of the tile
   * @param llBounds - the lon-lat bounds of the tile
   */
  addTileS2(face: Face, zoom: number, x: number, y: number, llBounds: BBox): void {
    const metadata = this.#metadata;
    // update tile stats
    if (metadata.tilestats !== undefined) {
      metadata.tilestats.total++;
      metadata.tilestats[face]++;
    }
    this.#faces.add(face);
    this.#addBoundsS2(face, zoom, x, y);
    // update lon lat
    this.#updateLonLatBounds(llBounds);
  }

  /**
   * Update the center now that all tiles have been added
   */
  #updateCenter() {
    const { minzoom, maxzoom } = this.#metadata;
    const [minlon, minlat, maxlon, maxlat] = this.#lonLatBounds;
    this.#metadata.center = {
      lon: (minlon + maxlon) >> 1,
      lat: (minlat + maxlat) >> 1,
      zoom: (minzoom + maxzoom) >> 1,
    };
  }

  /**
   * Add the bounds of the tile for WM data
   * @param zoom - zoom of the tile
   * @param x - x position of the tile
   * @param y - y position of the tile
   */
  #addBoundsWM(zoom: number, x: number, y: number): void {
    if (this.#metadata.bounds[zoom] === undefined) {
      this.#metadata.bounds[zoom] = [Infinity, Infinity, -Infinity, -Infinity];
    }

    const bbox = this.#metadata.bounds[zoom];
    bbox[0] = Math.min(bbox[0], x);
    bbox[1] = Math.min(bbox[1], y);
    bbox[2] = Math.max(bbox[2], x);
    bbox[3] = Math.max(bbox[3], y);
  }

  /**
   * Add the bounds of the tile for S2 data
   * @param face - face of the tile
   * @param zoom - zoom of the tile
   * @param x - x position of the tile
   * @param y - y position of the tile
   */
  #addBoundsS2(face: Face, zoom: number, x: number, y: number): void {
    if (this.#metadata.facesbounds[face][zoom] === undefined) {
      this.#metadata.facesbounds[face][zoom] = [Infinity, Infinity, -Infinity, -Infinity];
    }

    const bbox = this.#metadata.facesbounds[face][zoom];
    bbox[0] = Math.min(bbox[0], x);
    bbox[1] = Math.min(bbox[1], y);
    bbox[2] = Math.max(bbox[2], x);
    bbox[3] = Math.max(bbox[3], y);
  }

  /**
   * Update the lon-lat bounds so eventually we can find the center point of the data
   * @param llBounds - the lon-lat bounds of the tile
   */
  #updateLonLatBounds(llBounds: BBox) {
    const [minlon, minlat, maxlon, maxlat] = llBounds;
    this.#lonLatBounds[0] = Math.min(this.#lonLatBounds[0], minlon);
    this.#lonLatBounds[1] = Math.min(this.#lonLatBounds[1], minlat);
    this.#lonLatBounds[2] = Math.max(this.#lonLatBounds[2], maxlon);
    this.#lonLatBounds[3] = Math.max(this.#lonLatBounds[3], maxlat);
  }
}

/**
 * # TileJSON V3.0.0
 *
 * Represents a TileJSON metadata object.
 * ## Links
 * [TileJSON Spec](https://github.com/mapbox/tilejson-spec/blob/master/3.0.0/schema.json)
 */
export interface MapboxTileJSONMetadata {
  /**
   * Version of the TileJSON spec used.
   * Matches the pattern: `\d+\.\d+\.\d+\w?[\w\d]*`.
   */
  tilejson: string;
  /** Array of tile URL templates. */
  tiles: string[];
  /** Array of vector layer metadata. */
  vector_layers: VectorLayer[];
  /** Attribution string. */
  attribution?: string;
  /** Bounding box array [west, south, east, north]. */
  bounds?: BBox;
  /** Center coordinate array [longitude, latitude, zoom]. */
  center?: [lon: number, lat: number, zoom: number];
  /** Array of data source URLs. */
  data?: string[];
  /** Description of the tileset. */
  description?: string;
  /** Fill zoom level. Must be between 0 and 30. */
  fillzoom?: number;
  /** Array of UTFGrid URL templates. */
  grids?: string[];
  /** Legend of the tileset. */
  legend?: string;
  /** Maximum zoom level. Must be between 0 and 30. */
  maxzoom?: number;
  /** Minimum zoom level. Must be between 0 and 30. */
  minzoom?: number;
  /** Name of the tileset. */
  name?: string;
  /** Tile scheme, e.g., `xyz` or `tms`. */
  scheme?: Scheme;
  /** Template for interactivity. */
  template?: string;
  /** Version of the tileset. Matches the pattern: `\d+\.\d+\.\d+\w?[\w\d]*`. */
  version?: string;
  /** Allow additional properties */
  [key: string]: unknown;
}

/** When the input is unknown, it can be either an S2 TileJSON or a Mapbox TileJSON */
export type Metadatas = Metadata | MapboxTileJSONMetadata;

/**
 * If you're not sure which tilejson you are reading (Mapbox's spec or S2's spec), you can treat
 * the input as either and ensure the output is the same
 * @param metadatas - the S2 TileJSON or Mapbox TileJSON
 * @returns - S2 TileJSON
 */
export function toMetadata(metadatas: Metadatas): Metadata {
  if ('s2tilejson' in metadatas) {
    return metadatas as Metadata;
  } else {
    const [lon, lat, zoom] = metadatas.center ?? [0, 0, 0];
    return {
      s2tilejson: '1.0.0',
      version: metadatas.version ?? '1.0.0',
      name: metadatas.name ?? 'Converted from Mapbox TileJSON to S2 TileJSON',
      extension: metadatas.tiles?.[0].split('.')[1] ?? 'pbf',
      scheme: metadatas.scheme ?? 'xyz',
      description: metadatas.description ?? '',
      /** The type of the data */
      type: 'vector',
      /** The encoding of the data */
      encoding: 'none',
      /** List of faces that have data */
      faces: [0],
      /** WM Tile fetching bounds. Helpful to not make unecessary requests for tiles we know don't exist */
      bounds: {},
      /** S2 Tile fetching bounds. Helpful to not make unecessary requests for tiles we know don't exist */
      facesbounds: { 0: {}, 1: {}, 2: {}, 3: {}, 4: {}, 5: {} },
      /** minzoom at which to request tiles. [default=0] */
      minzoom: metadatas.minzoom ?? 0,
      /** maxzoom at which to request tiles. [default=27] */
      maxzoom: metadatas.maxzoom ?? 27,
      /** The center of the data */
      center: { lon, lat, zoom },
      /** { ['human readable string']: 'href' } */
      attribution: {},
      /** Track layer metadata */
      layers: {},
      /** Old spec, track basic layer metadata */
      vector_layers: metadatas.vector_layers ?? [],
    };
  }
}
