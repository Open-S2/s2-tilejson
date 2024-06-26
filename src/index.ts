/** S2 Face */
export type Face = 0 | 1 | 2 | 3 | 4 | 5;

/** The Bounding box, whether the tile bounds or lon-lat bounds or whatever. */
export type BBox = [left: number, bottom: number, right: number, top: number];

/** 1: points, 2: lines, 3: polys, 4: points3D, 5: lines3D, 6: polys3D */
export type DrawType = 1 | 2 | 3 | 4;

/** A Shape is a definition of how to unpack properties data for a specific source layer. */
export interface Shape {
  [shapeName: string]: Shape | Array<Shape> | 'null' | 'bool' | 'string' | 'u64' | 'i64' | 'f64';
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
  0: { total: number };
  1: { total: number };
  2: { total: number };
  3: { total: number };
  4: { total: number };
  5: { total: number };
}

/**
 * Attribution data is stored in an object.
 * The key is the name of the attribution, and the value is the link
 */
export type Attributions = Record<string, string>;

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

/** Check the source type of the layer */
export type SourceType = 'vector' | 'json' | 'raster' | 'raster-dem' | 'sensor';

/** Store the encoding of the data */
export type Encoding = 'gz' | 'br' | 'none';

/** Old spec tracks basic vector data */
export interface VectorLayer {
  id: string;
  description?: string;
  minzoom?: number;
  maxzoom?: number;
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
  lon: number;
  lat: number;
  zoom: number;
}

/** Metadata for the tile data */
export interface Metadata {
  /** The version of the s2-tilejson spec */
  s2tilejson: string;
  /** The version of the data */
  version: string;
  /** The name of the data */
  name: string;
  /** The scheme of the data */
  scheme: Scheme;
  /** The description of the data */
  description: string;
  /** The type of the data */
  type: SourceType;
  /** The encoding of the data */
  encoding: Encoding;
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
  center: Center;
  /** { ['human readable string']: 'href' } */
  attributions: { [name: string]: string };
  layers: LayersMetaData;
  /** Track tile stats for each face and total overall */
  tilestats: TileStatsMetadata;
  /** Old spec, track basic layer metadata */
  vector_layers: VectorLayer[];
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
    description: 'Built with s2maps-cli',
    type: 'vector',
    encoding: 'none',
    faces: [],
    bounds: {},
    facesbounds: {
      0: {},
      1: {},
      2: {},
      3: {},
      4: {},
      5: {},
    },
    minzoom: Infinity,
    maxzoom: -Infinity,
    center: { lon: 0, lat: 0, zoom: 0 },
    attributions: {},
    tilestats: {
      total: 0,
      0: { total: 0 },
      1: { total: 0 },
      2: { total: 0 },
      3: { total: 0 },
      4: { total: 0 },
      5: { total: 0 },
    },
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
    this.#metadata.attributions[displayName] = href;
  }

  /**
   * Add the layer metadata
   * @param name - name of the layer
   * @param layer - layer metadata
   */
  addLayer(name: string, layer: LayerMetaData) {
    this.#metadata.layers[name] = layer;
    this.#metadata.vector_layers?.push({
      id: name,
      description: layer.description,
      minzoom: layer.minzoom,
      maxzoom: layer.maxzoom,
    });
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
    metadata.tilestats.total++;
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
    metadata.tilestats.total++;
    metadata.tilestats[face].total++;
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
      lon: (minlon + maxlon) / 2,
      lat: (minlat + maxlat) / 2,
      zoom: (minzoom + maxzoom) / 2,
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
