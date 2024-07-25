#![no_std]
// #![deny(missing_docs)]
//! The `s2-tilejson` Rust crate... TODO

extern crate alloc;

use core::u64;

use serde::{Serialize, Deserialize};

use alloc::borrow::ToOwned;
use alloc::collections::BTreeSet;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// S2 Face
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Face {
    /// Face 0
    Face0 = 0,
    /// Face 1
    Face1 = 1,
    /// Face 2
    Face2 = 2,
    /// Face 3
    Face3 = 3,
    /// Face 4
    Face4 = 4,
    /// Face 5
    Face5 = 5,
}
impl From<Face> for u8 {
    fn from(face: Face) -> Self {
        face as u8
    }
}
impl From<u8> for Face {
    fn from(face: u8) -> Self {
        match face {
            1 => Face::Face1,
            2 => Face::Face2,
            3 => Face::Face3,
            4 => Face::Face4,
            5 => Face::Face5,
            _ => Face::Face0,
        }
    }
}

/// The Bounding box, whether the tile bounds or lon-lat bounds or whatever.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct BBox<T> {
    /// left most point; Also represents the left-most longitude
    pub left: T,
    /// bottom most point; Also represents the bottom-most latitude
    pub bottom: T,
    /// right most point; Also represents the right-most longitude
    pub right: T,
    /// top most point; Also represents the top-most latitude
    pub top: T,
}

pub type LonLatBounds = BBox<f64>;

pub type TileBounds = BBox<u64>;

/// 1: points, 2: lines, 3: polys, 4: points3D, 5: lines3D, 6: polys3D
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DrawType {
    /// Collection of points
    Points = 1,
    /// Collection of lines
    Lines = 2,
    /// Collection of polygons
    Polygons = 3,
    /// Collection of 3D points
    Points3D = 4,
    /// Collection of 3D lines
    Lines3D = 5,
    /// Collection of 3D polygons
    Polygons3D = 6,
}

// Shapes exist solely to deconstruct and rebuild objects.
//
// Shape limitations:
// - all keys are strings.
// - all values are either:
// - - primitive types: strings, numbers (f32, f64, u64, i64), true, false, or null
// - - sub types: an array of a shape or a nested object which is itself a shape
// - - if the sub type is an array, ensure all elements are of the same type
// The interfaces below help describe how shapes are built by the user.

/// Primitive types that can be found in a shape
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PrimitiveShape {
    /// 'string'
    String,
    /// number 'f32'
    F32,
    /// number 'f64'
    F64,
    /// number 'u64'
    U64,
    /// number 'i64'
    I64,
    /// 'true' or 'false'
    Bool,
    /// 'null'
    Null,
}

/// The Shape Object But the values can only be primitives
pub type ShapePrimitive = BTreeMap<String, PrimitiveShape>;

/// Arrays may contain either a primitive or an object whose values are primitives
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ShapePrimitiveType {
    /// Primitive type
    Primitive(PrimitiveShape),
    /// Nested shape that can only contain primitives
    NestedPrimitive(ShapePrimitive),
}

/// Shape types that can be found in a shapes object.
/// Either a primitive, an array containing any type, or a nested shape.
/// If the type is an array, all elements must be the same type
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ShapeType {
    /// Primitive type
    Primitive(PrimitiveShape),
    /// Nested shape that can only contain primitives
    Array(Vec<ShapePrimitiveType>),
    /// Nested shape
    Nested(Shape),
}

/// The Shape Object
pub type Shape = BTreeMap<String, ShapeType>;

/// Each layer has metadata associated with it. Defined as blueprints pre-construction of vector data.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct LayerMetaData {
    /// The description of the layer
    pub description: Option<String>,
    /// the lowest zoom level at which the layer is available
    pub minzoom: u8,
    /// the highest zoom level at which the layer is available
    pub maxzoom: u8,
    /// The draw types that can be found in this layer
    pub draw_types: Vec<DrawType>,
    /// The shape that can be found in this layer
    pub shape: Shape,
    /// The shape used inside features that can be found in this layer
    pub m_shape: Option<Shape>,
}

/// Each layer has metadata associated with it. Defined as blueprints pre-construction of vector data.
pub type LayersMetaData = BTreeMap<String, LayerMetaData>;

/// Tilestats is simply a tracker to see where most of the tiles live
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TileStatsMetadata {
    pub total: u64,
    pub total_0: u64,
    pub total_1: u64,
    pub total_2: u64,
    pub total_3: u64,
    pub total_4: u64,
    pub total_5: u64,
}
impl TileStatsMetadata {
    pub fn get(&self, face: Face) -> u64 {
        match face {
            Face::Face0 => self.total_0,
            Face::Face1 => self.total_1,
            Face::Face2 => self.total_2,
            Face::Face3 => self.total_3,
            Face::Face4 => self.total_4,
            Face::Face5 => self.total_5,
        }
    }

    pub fn increment(&mut self, face: Face) {
        match face {
            Face::Face0 => self.total_0 += 1,
            Face::Face1 => self.total_1 += 1,
            Face::Face2 => self.total_2 += 1,
            Face::Face3 => self.total_3 += 1,
            Face::Face4 => self.total_4 += 1,
            Face::Face5 => self.total_5 += 1,
        }
        self.total += 1;
    }
}

/// Attribution data is stored in an object.
/// The key is the name of the attribution, and the value is the link
pub type Attributions = BTreeMap<String, String>;

/// Track the S2 tile bounds of each face and zoom
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FaceBounds {
    // facesbounds[face][zoom] = [...]
    pub face0: BTreeMap<u8, TileBounds>,
    pub face1: BTreeMap<u8, TileBounds>,
    pub face2: BTreeMap<u8, TileBounds>,
    pub face3: BTreeMap<u8, TileBounds>,
    pub face4: BTreeMap<u8, TileBounds>,
    pub face5: BTreeMap<u8, TileBounds>,
}
impl FaceBounds {
    pub fn get(&self, face: Face) -> &BTreeMap<u8, TileBounds> {
        match face {
            Face::Face0 => &self.face0,
            Face::Face1 => &self.face1,
            Face::Face2 => &self.face2,
            Face::Face3 => &self.face3,
            Face::Face4 => &self.face4,
            Face::Face5 => &self.face5,
        }
    }

    pub fn get_mut(&mut self, face: Face) -> &mut BTreeMap<u8, TileBounds> {
        match face {
            Face::Face0 => &mut self.face0,
            Face::Face1 => &mut self.face1,
            Face::Face2 => &mut self.face2,
            Face::Face3 => &mut self.face3,
            Face::Face4 => &mut self.face4,
            Face::Face5 => &mut self.face5,
        }
    }
}

/// Track the WM tile bounds of each zoom
/// `[zoom: number]: BBox`
pub type WMBounds = BTreeMap<u8, TileBounds>;

/// Check the source type of the layer
#[derive(Debug, Default, Clone, PartialEq)]
pub enum SourceType {
    #[default] Vector,
    Json,
    Raster,
    RasterDem,
    Sensor,
}
impl From<&str> for SourceType {
    fn from(source_type: &str) -> Self {
        match source_type {
            "vector" => SourceType::Vector,
            "json" => SourceType::Json,
            "raster" => SourceType::Raster,
            "raster-dem" => SourceType::RasterDem,
            "sensor" => SourceType::Sensor,
            _ => SourceType::Vector,
        }
    }
}

/// Store the encoding of the data
#[derive(Debug, Default, Clone, PartialEq)]
pub enum Encoding {
    Gzip,
    Brotli,
    Zstd,
    #[default] None,
}
impl From<u8> for Encoding {
    fn from(encoding: u8) -> Self {
        match encoding {
            1 => Encoding::Gzip,
            2 => Encoding::Brotli,
            3 => Encoding::Zstd,
            _ => Encoding::None,
        }
    }
}
impl From<Encoding> for u8 {
    fn from(encoding: Encoding) -> Self {
        match encoding {
            Encoding::Gzip => 1,
            Encoding::Brotli => 2,
            Encoding::Zstd => 3,
            Encoding::None => 0,
        }
    }
}
impl From<Encoding> for String {
    fn from(encoding: Encoding) -> Self {
        match encoding {
            Encoding::Gzip => "gzip".into(),
            Encoding::Brotli => "br".into(),
            Encoding::Zstd => "zstd".into(),
            Encoding::None => "none".into(),
        }
    }
}
impl From<&str> for Encoding {
    fn from(encoding: &str) -> Self {
        match encoding {
            "gzip" => Encoding::Gzip,
            "br" => Encoding::Brotli,
            "zstd" => Encoding::Zstd,
            _ => Encoding::None,
        }
    }
}

/// Old spec tracks basic vector data
#[derive(Debug, Default, Clone, PartialEq)]
pub struct VectorLayer {
    pub id: String,
    pub description: Option<String>,
    pub minzoom: Option<u8>,
    pub maxzoom: Option<u8>,
}

/// Default S2 tile scheme is `fzxy`
/// Default Web Mercator tile scheme is `xyz`
/// Adding a t prefix to the scheme will change the request to be time sensitive
/// TMS is an oudated version that is not supported by s2maps-gpu
#[derive(Debug, Default, Clone, PartialEq)]
pub enum Scheme {
    #[default] Fzxy,
    Tfzxy,
    Xyz,
    Txyz,
    Tms,
}
impl From<&str> for Scheme {
    fn from(scheme: &str) -> Self {
        match scheme {
            "fzxy" => Scheme::Fzxy,
            "tfzxy" => Scheme::Tfzxy,
            "xyz" => Scheme::Xyz,
            "txyz" => Scheme::Txyz,
            _ => Scheme::Tms,
        }
    }
}
impl From<Scheme> for String {
    fn from(scheme: Scheme) -> Self {
        match scheme {
            Scheme::Fzxy => "fzxy".into(),
            Scheme::Tfzxy => "tfzxy".into(),
            Scheme::Xyz => "xyz".into(),
            Scheme::Txyz => "txyz".into(),
            Scheme::Tms => "tms".into(),
        }
    }
}

/// Store where the center of the data lives
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Center {
    pub lon: f64,
    pub lat: f64,
    pub zoom: u8,
}

/// Metadata for the tile data
#[derive(Debug, Clone, PartialEq)]
pub struct Metadata {
    /// The version of the s2-tilejson spec
    pub s2tilejson: String,
    /// The version of the data
    pub version: String,
    /// The name of the data
    pub name: String,
    /// The scheme of the data
    pub scheme: Scheme,
    /// The description of the data
    pub description: String,
    /// The type of the data
    pub type_: SourceType,
    /// The encoding of the data
    pub encoding: Encoding,
    /// List of faces that have data
    pub faces: Vec<Face>,
    /// WM Tile fetching bounds. Helpful to not make unecessary requests for tiles we know don't exist
    pub bounds: WMBounds,
    /// S2 Tile fetching bounds. Helpful to not make unecessary requests for tiles we know don't exist
    pub facesbounds: FaceBounds,
    /// minzoom at which to request tiles. [default=0]
    pub minzoom: u8,
    /// maxzoom at which to request tiles. [default=27]
    pub maxzoom: u8,
    /// The center of the data
    pub center: Center,
    /// { ['human readable string']: 'href' }
    pub attributions: BTreeMap<String, String>,
    /// Track layer metadata
    pub layers: LayersMetaData,
    /// Track tile stats for each face and total overall
    pub tilestats: TileStatsMetadata,
    /// Old spec, track basic layer metadata
    pub vector_layers: Vec<VectorLayer>,
}
impl Default for Metadata {
    fn default() -> Self {
        Self {
            s2tilejson: "1.0.0".into(),
            version: "1.0.0".into(),
            name: "default".into(),
            scheme: Scheme::default(),
            description: "Built with s2maps-cli".into(),
            type_: SourceType::default(),
            encoding: Encoding::default(),
            faces: Vec::new(),
            bounds: WMBounds::default(),
            facesbounds: FaceBounds::default(),
            minzoom: 0,
            maxzoom: 27,
            center: Center::default(),
            attributions: BTreeMap::new(),
            layers: LayersMetaData::default(),
            tilestats: TileStatsMetadata::default(),
            vector_layers: Vec::new(),
        }
    }
}

/// Builder for the metadata
#[derive(Debug, Clone)]
pub struct MetadataBuilder {
    lon_lat_bounds: LonLatBounds,
    faces: BTreeSet<Face>,
    metadata: Metadata,
}
impl Default for MetadataBuilder {
    fn default() -> Self {
        MetadataBuilder {
            lon_lat_bounds: BBox { left: f64::INFINITY, bottom: f64::INFINITY, right: -f64::INFINITY, top: -f64::INFINITY },
            faces: BTreeSet::new(),
            metadata: Metadata { minzoom: 30, maxzoom: 0, ..Metadata::default() },
        }
    }
}
impl MetadataBuilder {
    /// Commit the metadata and take ownership
    pub fn commit(&mut self) -> Metadata {
        // set the center
        self.update_center();
        // set the faces
        for face in &self.faces {
            self.metadata.faces.push(*face);
        }
        // return the result
        self.metadata.to_owned()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.metadata.name = name;
    }

    /// Set the scheme of the data. [default=fzxy]
    pub fn set_scheme(&mut self, scheme: Scheme) {
        self.metadata.scheme = scheme;
    }

    /// Set the type of the data. [default=vector]
    pub fn set_type(&mut self, type_: SourceType) {
        self.metadata.type_ = type_;
    }

    /// Set the version of the data
    pub fn set_version(&mut self, version: String) {
        self.metadata.version = version;
    }

    /// Set the description of the data
    pub fn set_description(&mut self, description: String) {
        self.metadata.description = description;
    }

    /// Set the encoding of the data. [default=none]
    pub fn set_encoding(&mut self, encoding: Encoding) {
        self.metadata.encoding = encoding;
    }

    /// Set the minzoom
    pub fn set_minzoom(&mut self, minzoom: u8) {
        self.metadata.minzoom = minzoom;
    }

    /// add an attribution
    pub fn add_attribution(&mut self, display_name: &str, href: &str) {
        self.metadata.attributions.insert(display_name.into(), href.into());
    }

    /// Add the layer metadata
    pub fn add_layer(&mut self, name: &str, layer: &LayerMetaData) {
        // Only insert if the key does not exist
        if self.metadata.layers.entry(name.into()).or_insert(layer.clone()).eq(&layer) {
            // Also add to vector_layers only if the key was not present and the insert was successful
            self.metadata.vector_layers.push(VectorLayer {
                id: name.into(),  // No need to clone again; we use the moved value
                description: layer.description.clone(),
                minzoom: Some(layer.minzoom),
                maxzoom: Some(layer.maxzoom),
            });
        }
        // update minzoom and maxzoom
        if layer.minzoom < self.metadata.minzoom { self.metadata.minzoom = layer.minzoom; }
        if layer.maxzoom > self.metadata.maxzoom { self.metadata.maxzoom = layer.maxzoom; }
    }

    /// Add the WM tile metadata
    pub fn add_tile_wm(&mut self, zoom: u8, x: u32, y: u32, ll_bounds: &LonLatBounds) {
        self.metadata.tilestats.total += 1;
        self.faces.insert(Face::Face0);
        self.add_bounds_wm(zoom, x, y);
        self.update_lon_lat_bounds(ll_bounds);
    }

    /// Add the S2 tile metadata
    pub fn add_tile_s2(&mut self, face: Face, zoom: u8, x: u32, y: u32, ll_bounds: &LonLatBounds) {
        self.metadata.tilestats.increment(face);
        self.faces.insert(face);
        self.add_bounds_s2(face, zoom, x, y);
        self.update_lon_lat_bounds(ll_bounds);
    }

    /// Update the center now that all tiles have been added
    fn update_center(&mut self) {
        let Metadata { minzoom, maxzoom, .. } = self.metadata;
        let BBox { left, bottom, right, top } = self.lon_lat_bounds;
        self.metadata.center.lon = (left + right) / 2.0;
        self.metadata.center.lat = (bottom + top) / 2.0;
        self.metadata.center.zoom = (minzoom + maxzoom) >> 1;
    }

    /// Add the bounds of the tile for WM data
    fn add_bounds_wm(&mut self, zoom: u8, x: u32, y: u32) {
        let x = x as u64;
        let y = y as u64;
        let bbox = self.metadata.bounds.entry(zoom).or_insert(BBox{ 
            left: u64::MAX, bottom: u64::MAX, right: 0, top: 0
        });
        
        bbox.left = bbox.left.min(x);
        bbox.bottom = bbox.bottom.min(y);
        bbox.right = bbox.right.max(x);
        bbox.top = bbox.top.max(y);
    }

    /// Add the bounds of the tile for S2 data
    fn add_bounds_s2(&mut self, face: Face, zoom: u8, x: u32, y: u32) {
        let x = x as u64;
        let y = y as u64;
        let bbox = self.metadata.facesbounds.get_mut(face).entry(zoom).or_insert(BBox{ 
            left: u64::MAX, bottom: u64::MAX, right: 0, top: 0
        });
        
        bbox.left = bbox.left.min(x);
        bbox.bottom = bbox.bottom.min(y);
        bbox.right = bbox.right.max(x);
        bbox.top = bbox.top.max(y);
    }

    /// Update the lon-lat bounds so eventually we can find the center point of the data
    fn update_lon_lat_bounds(&mut self, ll_bounds: &LonLatBounds) {
        self.lon_lat_bounds.left = ll_bounds.left.min(self.lon_lat_bounds.left);
        self.lon_lat_bounds.bottom = ll_bounds.bottom.min(self.lon_lat_bounds.bottom);
        self.lon_lat_bounds.right = ll_bounds.right.max(self.lon_lat_bounds.right);
        self.lon_lat_bounds.top = ll_bounds.top.max(self.lon_lat_bounds.top);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut meta_builder = MetadataBuilder::default();


        // on initial use be sure to update basic metadata:
        meta_builder.set_name("OSM".into());
        meta_builder.set_description("A free editable map of the whole world.".into());
        meta_builder.set_version("1.0.0".into());
        meta_builder.set_scheme("fzxy".into()); // 'fzxy' | 'tfzxy' | 'xyz' | 'txyz' | 'tms'
        meta_builder.set_type("vector".into()); // 'vector' | 'json' | 'raster' | 'raster-dem' | 'sensor' | 'markers'
        meta_builder.set_encoding("none".into()); // 'gz' | 'br' | 'none'
        meta_builder.add_attribution("OpenStreetMap", "https://www.openstreetmap.org/copyright/");

        // Vector Specific: add layers based on how you want to parse data from a source:
        let shape_str = r#"
        {
            "class": "string",
            "offset": "f64",
            "info": {
                "name": "string",
                "value": "i64"
            }
        }
        "#;
        let shape: Shape = serde_json::from_str(shape_str).unwrap_or_else(|e| panic!("ERROR: {}", e));
        let layer = LayerMetaData {
            minzoom: 0,
            maxzoom: 13,
            description: Some("water_lines".into()),
            draw_types: Vec::from(&[DrawType::Lines]),
            shape: shape.clone(),
            m_shape: None,
        };
        meta_builder.add_layer("water_lines", &layer);

        // as you build tiles, add the tiles metadata:
        // WM:
        meta_builder.add_tile_wm(0, 0, 0, &BBox{ left: -60.0, bottom: -20.0, right: 5.0, top: 60.0 });
        // S2:
        meta_builder.add_tile_s2(Face::Face1, 5, 22, 37, &BBox { left: -120.0, bottom: -7.0, right: 44.0, top: 72.0 });

        // finally to get the resulting metadata:
        let resulting_metadata: Metadata = meta_builder.commit();

        assert_eq!(resulting_metadata, Metadata {
            name: "OSM".into(),
            description: "A free editable map of the whole world.".into(),
            version: "1.0.0".into(),
            scheme: "fzxy".into(),
            type_: "vector".into(),
            encoding: "none".into(),
            attributions: BTreeMap::from([
                ("OpenStreetMap".into(), "https://www.openstreetmap.org/copyright/".into()),
            ]),
            bounds: BTreeMap::from([
                (0, BBox { left: 0, bottom: 0, right: 0, top: 0 }),
            ]),
            faces: Vec::from(&[Face::Face0, Face::Face1]),
            facesbounds: FaceBounds {
                face0: BTreeMap::new(),
                face1: BTreeMap::from([
                    (5, BBox { left: 22, bottom: 37, right: 22, top: 37 }),
                ]),
                face2: BTreeMap::new(),
                face3: BTreeMap::new(),
                face4: BTreeMap::new(),
                face5: BTreeMap::new(),
            },
            minzoom: 0,
            maxzoom: 13,
            center: Center { lon: -38.0, lat: 26.0, zoom: 6 },
            tilestats: TileStatsMetadata {
                total: 2,
                total_0: 0,
                total_1: 1,
                total_2: 0,
                total_3: 0,
                total_4: 0,
                total_5: 0,
            },
            layers: BTreeMap::from([("water_lines".into(), LayerMetaData{
                description: Some("water_lines".into()),
                minzoom: 0,
                maxzoom: 13,
                draw_types: Vec::from(&[DrawType::Lines]),
                shape: BTreeMap::from([
                    ("class".into(), ShapeType::Primitive(PrimitiveShape::String)),
                    ("offset".into(), ShapeType::Primitive(PrimitiveShape::F64)),
                    ("info".into(), ShapeType::Nested(BTreeMap::from([
                        ("name".into(), ShapeType::Primitive(PrimitiveShape::String)),
                        ("value".into(), ShapeType::Primitive(PrimitiveShape::I64)),
                    ]))),
                ]),
                m_shape: None,
            })]),
            s2tilejson: "1.0.0".into(),
            vector_layers: Vec::from([VectorLayer { id: "water_lines".into(), description: Some("water_lines".into()), minzoom: Some(0), maxzoom: Some(13) }]),
        });
    }
}
