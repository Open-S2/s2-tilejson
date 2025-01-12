# S2-TileJSON 1.0.0

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

## Table of contents

1. [Purpose](#1-purpose)
1. [File format](#2-file-format)
1. [Structure](#3-structure)
   1. [s2-tilejson](#31-s2-tilejson)
   1. [interval](#32-interval)
   1. [vector_layers](#33-vector_layers)
   1. [attribution](#34-attribution)
   1. [bounds](#35-bounds)
   1. [center](#36-center)
   1. [faces](#37-faces)
   1. [description](#38-description)
   1. [fillzoom](#39-fillzoom)
   1. [facesbounds](#310-facesbounds)
   1. [legend](#311-legend)
   1. [maxzoom](#312-maxzoom)
   1. [minzoom](#313-minzoom)
   1. [name](#314-name)
   1. [scheme](#315-scheme)
   1. [layers](#316-layers)
   1. [version](#317-version)
   1. [tilestats](#318-tilestats)
   1. [type](#319-type)
   1. [extension](#320-extension)
   1. [encoding](#321-encoding)
1. [Examples](#4-examples)
1. [Caching](#5-caching)

## 1. Purpose

This specification attempts to create a standard for representing metadata about multiple types of web-based map layers, to aid clients in configuration and browsing.

## 2. File Format

S2-TileJSON manifest files use the JSON format as described in [RFC 8259](https://tools.ietf.org/html/rfc8259).

## 3. Structure

The following describes the structure of a S2-TileJSON object. Implementations MUST treat unknown keys as if they weren't present. However, implementations MUST expose unknown key value pairs so users can optionally handle these keys. Implementations MUST treat invalid values for keys as if they were not present. If the key is optional and the value is invalid, the default value MAY be applied. If the key is required, implementations MUST treat the entire S2-TileJSON manifest file as invalid and refuse operation.

*The word "implementation" in the following sections refers to services or tools that serve, generate, or validate S2-TileJSON objects.*

### 3.1 `s2-tilejson`

REQUIRED. String.

A semver.org style version number as a string. Describes the version of the S2-TileJSON spec that is implemented by this JSON object.

```JSON
{
  "s2tilejson": "1.0.0"
}
```

### 3.2 `interval`

Optional. Number

The time interval in milliseconds each frame is.

```JSON
{
  "interval": 1500
}
```

### 3.3 `vector_layers`

REQUIRED (deprecated - see `layers`). Array<Object>.

An array of objects. Each object describes one layer of vector tile data. A `vector_layer` object MUST contain the `id` and `fields` keys, and MAY contain the `description`, `minzoom`, or `maxzoom` keys. An implementation MAY include arbitrary keys in the object outside those defined in this specification.

Note: When describing a set of raster tiles or other tile format that does not have a "layers" concept (i.e. `"format": "jpeg"`), the `vector_layers` key is not required.

#### 3.3.1 `id`

REQUIRED. String.

A string value representing the layer id. For added context, this is referred to as the `name` of the layer in the [Mapbox Vector Tile spec](https://github.com/mapbox/vector-tile-spec/tree/master/2.1#41-layers).

#### 3.3.2 `fields`

REQUIRED. Object.

An object whose keys and values are the names and descriptions of attributes available in this layer. Each value (description) MUST be a string that describes the underlying data. If no fields are present, the `fields` key MUST be an empty object.

#### 3.3.3 `description`

OPTIONAL. String.

A string representing a human-readable description of the entire layer's contents.

#### 3.3.4 `minzoom` and `maxzoom`

OPTIONAL. Integer.

An integer representing the lowest/highest zoom level whose tiles this layer appears in. `minzoom` MUST be greater than or equal to the set of tiles' `minzoom`. `maxzoom` MUST be less than or equal to the set of tiles' `maxzoom`.

These keys are used to describe the situation where different sets of vector layers appear in different zoom levels of the same set of tiles, for example in a case where a "minor roads" layer is only present at high zoom levels.

```JSON
{
  "vector_layers": [
    {
      "id": "roads",
      "description": "Roads and their attributes",
      "minzoom": 2,
      "maxzoom": 16,
      "fields": {
        "type": "One of: trunk, primary, secondary",
        "lanes": "Number",
        "name": "String",
        "sidewalks": "Boolean"
      }
    },
    {
      "id": "countries",
      "description": "Admin 0 (country) boundaries",
      "minzoom": 0,
      "maxzoom": 16,
      "fields": {
        "iso": "ISO 3166-1 Alpha-2 code",
        "name": "English name of the country",
        "name_ar": "Arabic name of the country"
      }
    },
    {
      "id": "buildings",
      "description": "A layer with an empty fields object",
      "fields": {}
    }
  ]
}
```

### 3.4 `attribution`

OPTIONAL. Object. Default: `null`.

NOTICE: This is a breaking change from the [older tilejson specification](https://github.com/mapbox/tilejson-spec/blob/master/3.0.0/README.md#34-attribution).

Contains a map of attributions to be displayed when the map is shown to a user. The key is the display name and the value is the link to the attribution.

```JSON
{
  "attribution": {
    "OpenStreetMap": "https://www.openstreetmap.org/copyright/",
    "Open S2": "https://opens2.com/legal/attribution"
  }
}
```

### 3.5 `bounds`

OPTIONAL. Object. Default: `{}`.

NOTICE: This is a breaking change from the [older tilejson specification](https://github.com/mapbox/tilejson-spec/blob/master/3.0.0/README.md#34-attribution). Instead of tracking lon-lat bounds, it tracks tile bounds to help reduce the number of requests.

The maximum extent of available map tiles relative to the zoom. Each zoom has it's own bounding box defined by `[minX, minY, maxX, maxY]`.

```JSON
"bounds": { // bounds[zoom] = [...]
  "0": [0, 0, 0, 0],
  "1": [0, 1, 0, 0],
  "2": [0, 3, 1, 3]
},
```

### 3.6 `center`

OPTIONAL. Array<Number>. Default: `null`.

The first value is the longitude, the second is latitude (both in WGS:84 values), the third value is the zoom level as an integer. Longitude and latitude MUST be within the specified bounds. The zoom level MUST be between minzoom and maxzoom. Implementations MAY use this center value to set the default location. If the value is null, implementations MAY use their own algorithm for determining a default location.

```JSON
{
  "center": {
    "lon": -76.275329586789,
    "lat": 39.153492567373,
    "zoom": 8 
  }
}
```

### 3.7 `faces`

OPTIONAL. Array<Number>. Default: `[]`.

This is an S2 specific key. An array of faces that the data interacts with. Face vaules MUST be within `[0-6)`.

```JSON
{
  "faces": [0, 2, 5]
}
```

### 3.8 `description`

OPTIONAL. String. Default: `null`.

A text description of the set of tiles. The description can contain any valid unicode character as described by the JSON specification [RFC 8259](https://tools.ietf.org/html/rfc8259).

```JSON
{
  "description": "Highways, roads, and vehicular tracks derived from OpenStreetMap."
}
```

### 3.9 `fillzoom`

OPTIONAL. Integer. Default: `null`.

An integer specifying the zoom level from which to generate overzoomed tiles. Implementations MAY generate overzoomed tiles from parent tiles if the requested zoom level does not exist. In most cases, overzoomed tiles are generated from the maximum zoom level of the set of tiles. If fillzoom is specified, the overzoomed tile MAY be generated from the fillzoom level.

For example, in a set of tiles with maxzoom 10 and *no* fillzoom specified, a request for a z11 tile will use the z10 parent tiles to generate the new, overzoomed z11 tile. If the same TileJSON object had fillzoom specified at z7, a request for a z11 tile would use the z7 tile instead of z10.

While TileJSON may specify rules for overzooming tiles, it is ultimately up to the tile serving client or renderer to implement overzooming.

```JSON
{
  "fillzoom": 7
}
```

### 3.10 `facesbounds`

OPTIONAL. Object. Default: `null`.

This is an S2 specific key. An object that maps each face to its bounds. The bounds are specified in the same format as the `bounds` property except that the starting key is a face number.

Much like how bounds are tracked, facebounds utilize the same format as bounds but map to

```JSON
{
  "facesbounds": { // facesbounds[face][zoom] = [...]
    "0": { "0": [0, 0, 0, 0], "1": [0, 1, 0, 0], "2": [0, 3, 1, 3] },
  }
}
```

### 3.11 `legend`

OPTIONAL. String. Default: `null`.

Contains a legend to be displayed with the map. Implementations MAY decide to treat this as HTML or literal text. For security reasons, make absolutely sure that this field can't be abused as a vector for XSS or beacon tracking.

```JSON
{
  "legend": "Dangerous zones are red, safe zones are green"
}
```

### 3.12 `maxzoom`

REQUIRED. Integer. Default: `30`.

An integer specifying the maximum zoom level. MUST be in range: 0 <= minzoom <= maxzoom <= 30. A client or server MAY request tiles outside the zoom range, but the availability of these tiles is dependent on how the tile server or renderer handles the request (such as overzooming tiles).

```JSON
{
  "maxzoom": 11
}
```

### 3.13 `minzoom`

REQUIRED. Integer. Default: `0`.

An integer specifying the minimum zoom level. MUST be in range: 0 <= minzoom <= maxzoom <= 30.

```JSON
{
  "minzoom": 0
}
```

### 3.14 `name`

OPTIONAL. String. Default: `null`.

A name describing the set of tiles. The name can contain any legal character. Implementations SHOULD NOT interpret the name as HTML.

```JSON
{
  "name": "Earthsea v2"
}
```

### 3.15 `scheme`

OPTIONAL. String. Default: `"fzxy"`.

Mercator: Either `"xyz"` or `"txyz"`. The global-mercator (aka Spherical Mercator) profile is assumed. if a `t` is attached to the beginning of the xyz spec, the tiles are time based.

S2: May be `"fzxy"` or `"tfzxy"`. This stands for `face`-`zoom`-`x`-`y`. If a `t` is attached to the beginning of the xyz spec, the tiles are time based.

```JSON
{
  "scheme": "xyz"
}
```

### 3.16 `layers`

REQUIRED. Object. Default: `{}`.

Designed to better support [open-vector-tile](https://github.com/Open-S2/open-vector-tile) data with `shape` types and the more complex nested objects.

#### 3.16.1 `key`

The object key is the `name` of the layer.

#### 3.16.2 `value`

##### 3.16.2.1 `description`

OPTIONAL. String. Default: `null`.

The description is used for the layer name in the legend.

##### 3.16.2.2 `minzoom`

REQUIRED. Unsigned Integer. Default: `null`.

The minimum zoom level of the layer.

##### 3.16.2.3 `maxzoom`

REQUIRED. Unsigned Integer. Default: `null`.

The maximum zoom level of the layer.

##### 3.16.2.4 `drawTypes`

REQUIRED. Array<Number>. Default: `[]`.

The draw type of the layer.

One of:

- 1 Points
- 2 Lines
- 3 Polygons
- 4 Points3D
- 5 Lines3D
- 6 Polygons3D
- 7 Raster
- 8 Grid

##### 3.16.2.5 `shape`

REQUIRED. Object. Default: `{}`.

The shape of the properties data for the layer. Each object key is the `name` of the property. Each object value defines the type of data in the shape and may itself be an object. Values are string definitions that may be one of:

- `string`
- `u64`
- `i64`
- `f64`
- `bool`
- `Array<TYPE>`
- `{ ... }`

In the case of an array, the values are all of the same type. While the number types may seem expensive, the writer's will encode them (e.g. varint) to reduce the number of bytes, this is just for the reader's sake. Always assume any value MAY be null in actuality. Also, if the array isn't a primitive type, then it MUST be an object but that object may only have values that are primitives.

Example Shape:

```JSON
{
  "class": "string",
  "offset": "f64",
  "info": {
    "name": "string",
    "value": "i64",
  },
  "arr": ["u64"]
}
```

##### 3.16.2.6 `mShape`

OPTIONAL. Object. Default: `undefined`.

The shape of the geometries mValues. Each object key is the `name` of the property. Each object value defines the type of data in the shape and may itself be an object. Values may be one of:

- `null`
- `string`
- `u64`
- `i64`
- `f64`
- `bool`
- `Array<TYPE>`
- `{ ... }`

In the case of an array, the values are all of the same type. While the number types may seem expensive, the writer's will encode them (e.g. varint) to reduce the number of bytes, this is just for the reader's sake.

Example Shape:

```JSON
{
  "class": "string",
  "offset": "f64",
  "info": {
    "name": "string",
    "value": "i64",
  }
}
```

### 3.17 `version`

OPTIONAL. String. Default: `"1.0.0"`.

A [semver.org](https://semver.org) style version number of the tiles. When changes across tiles are introduced the minor version MUST change. This may lead to cut off labels. Therefore, implementors can decide to clean their cache when the minor version changes. Changes to the patch level MUST only have changes to tiles that are contained within one tile. When tiles change significantly, such as updating a vector tile layer name, the major version MUST be increased. Implementations MUST NOT use tiles with different major versions.

```JSON
{
  "version": "1.0.0"
}
```

### 3.18 `tilestats`

OPTIONAL. Object. Default: `{}`.

Tile count statistics. Includes an all encompassing count called `total`. If using the S2 TileJSON format, create totals for each face.

```JSON
{
  "tilestats": {
    "total": 7,
    "0": 0,
    "1": 0,
    "2": 5,
    "3": 2,
    "4": 0,
    "5": 0
  }
}
```

### 3.19 `type`

REQUIRED. String. Default: `"vector"`.

The type of the tiles being used. May be one of `"vector"`, `"json"`, `"raster"`, `"raster-dem"`, `"markers"`, or `"grid"`. Because this value is often misused, you should support an "unknown" type but more often then not the string value is `"overlay"`.

```JSON
{
  "type": "vector"
}
```

### 3.20 `extension`

REQUIRED. String. Default: `pbf`.

Explains the extension name attached to the file So if the source url is `https://tile.example.com/data-example`, then the metadata url is `https://tile.example.com/data-example.json` and the metadata's `extension` property (for example may be `pbf`) will then ensure the following request could be something like `https://tile.example.com/data-example/{z}/{x}/{y}.pbf`

```JSON
{
  "extension": "pbf"
}
```

### 3.21 `encoding`

OPTIONAL. String. Default: `"none"`.

The encoding used to store the tile data. May be one of `none`, `gzip`, `br` or `zstd`.

## 4. Examples

Examples can be found in the [examples directory](./example).

## 5. Caching

Clients MAY cache files retrieved from a remote server. When implementations decide to perform caching, they MUST honor valid cache control HTTP headers as defined in the HTTP specification for both tile images and the TileJSON manifest file.
