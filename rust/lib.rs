#![no_std]
#![deny(missing_docs)]
//! The `s2-tilejson` Rust crate... TODO

extern crate alloc;

use serde::de::{self, SeqAccess, Visitor};
use serde::ser::SerializeTuple;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use alloc::borrow::ToOwned;
use alloc::collections::BTreeMap;
use alloc::collections::BTreeSet;
use alloc::fmt;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// S2 Face
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
pub struct BBox<T = f64> {
    /// left most point; Also represents the left-most longitude
    pub left: T,
    /// bottom most point; Also represents the bottom-most latitude
    pub bottom: T,
    /// right most point; Also represents the right-most longitude
    pub right: T,
    /// top most point; Also represents the top-most latitude
    pub top: T,
}
impl<T> Serialize for BBox<T>
where
    T: Serialize + Copy,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(4)?;
        seq.serialize_element(&self.left)?;
        seq.serialize_element(&self.bottom)?;
        seq.serialize_element(&self.right)?;
        seq.serialize_element(&self.top)?;
        seq.end()
    }
}

impl<'de, T> Deserialize<'de> for BBox<T>
where
    T: Deserialize<'de> + Copy,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BBoxVisitor<T> {
            marker: core::marker::PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for BBoxVisitor<T>
        where
            T: Deserialize<'de> + Copy,
        {
            type Value = BBox<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence of four numbers")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<BBox<T>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let left = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let bottom = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let right = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let top = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                Ok(BBox {
                    left,
                    bottom,
                    right,
                    top,
                })
            }
        }

        deserializer.deserialize_tuple(
            4,
            BBoxVisitor {
                marker: core::marker::PhantomData,
            },
        )
    }
}

/// Use bounds as floating point numbers for longitude and latitude
pub type LonLatBounds = BBox<f64>;

/// Use bounds as u64 for the tile index range
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
impl From<DrawType> for u8 {
    fn from(draw_type: DrawType) -> Self {
        draw_type as u8
    }
}
impl From<u8> for DrawType {
    fn from(draw_type: u8) -> Self {
        match draw_type {
            1 => DrawType::Points,
            2 => DrawType::Lines,
            3 => DrawType::Polygons,
            4 => DrawType::Points3D,
            5 => DrawType::Lines3D,
            6 => DrawType::Polygons3D,
            _ => DrawType::Points,
        }
    }
}
impl Serialize for DrawType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as u8
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for DrawType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize from u8 or string
        let value: u8 = Deserialize::deserialize(deserializer)?;
        match value {
            1 => Ok(DrawType::Points),
            2 => Ok(DrawType::Lines),
            3 => Ok(DrawType::Polygons),
            4 => Ok(DrawType::Points3D),
            5 => Ok(DrawType::Lines3D),
            6 => Ok(DrawType::Polygons3D),
            _ => Err(serde::de::Error::custom(format!(
                "unknown DrawType variant: {}",
                value
            ))),
        }
    }
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
    /// String type utf8 encoded
    String,
    /// unsigned 64 bit integer
    U64,
    /// signed 64 bit integer
    I64,
    /// floating point number
    F32,
    /// double precision floating point number
    F64,
    /// boolean
    Bool,
    /// null
    Null,
}

/// Arrays may contain either a primitive or an object whose values are primitives
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ShapePrimitiveType {
    /// Primitive type
    Primitive(PrimitiveShape),
    /// Nested shape that can only contain primitives
    NestedPrimitive(BTreeMap<String, PrimitiveShape>),
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
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct LayerMetaData {
    /// The description of the layer
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "mShape")]
    pub m_shape: Option<Shape>,
}

/// Each layer has metadata associated with it. Defined as blueprints pre-construction of vector data.
pub type LayersMetaData = BTreeMap<String, LayerMetaData>;

/// Tilestats is simply a tracker to see where most of the tiles live
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct TileStatsMetadata {
    /// total number of tiles
    #[serde(default)]
    pub total: u64,
    /// number of tiles for face 0
    #[serde(rename = "0", default)]
    pub total_0: u64,
    /// number of tiles for face 1
    #[serde(rename = "1", default)]
    pub total_1: u64,
    /// number of tiles for face 2
    #[serde(rename = "2", default)]
    pub total_2: u64,
    /// number of tiles for face 3
    #[serde(rename = "3", default)]
    pub total_3: u64,
    /// number of tiles for face 4
    #[serde(rename = "4", default)]
    pub total_4: u64,
    /// number of tiles for face 5
    #[serde(rename = "5", default)]
    pub total_5: u64,
}
impl TileStatsMetadata {
    /// Access the total number of tiles for a given face
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

    /// Increment the total number of tiles for a given face and also the grand total
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
pub type Attribution = BTreeMap<String, String>;

/// Track the S2 tile bounds of each face and zoom
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct FaceBounds {
    // facesbounds[face][zoom] = [...]
    /// Tile bounds for face 0 at each zoom
    #[serde(rename = "0")]
    pub face0: BTreeMap<u8, TileBounds>,
    /// Tile bounds for face 1 at each zoom
    #[serde(rename = "1")]
    pub face1: BTreeMap<u8, TileBounds>,
    /// Tile bounds for face 2 at each zoom
    #[serde(rename = "2")]
    pub face2: BTreeMap<u8, TileBounds>,
    /// Tile bounds for face 3 at each zoom
    #[serde(rename = "3")]
    pub face3: BTreeMap<u8, TileBounds>,
    /// Tile bounds for face 4 at each zoom
    #[serde(rename = "4")]
    pub face4: BTreeMap<u8, TileBounds>,
    /// Tile bounds for face 5 at each zoom
    #[serde(rename = "5")]
    pub face5: BTreeMap<u8, TileBounds>,
}
impl FaceBounds {
    /// Access the tile bounds for a given face and zoom
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

    /// Access the mutable tile bounds for a given face and zoom
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
#[derive(Serialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SourceType {
    /// Vector data
    #[default]
    Vector,
    /// Json data
    Json,
    /// Raster data
    Raster,
    /// Raster DEM data
    #[serde(rename = "raster-dem")]
    RasterDem,
    /// Sensor data
    Sensor,
    /// Marker data
    Markers,
    /// Unknown source type
    Unknown,
}
impl From<&str> for SourceType {
    fn from(source_type: &str) -> Self {
        match source_type {
            "vector" => SourceType::Vector,
            "json" => SourceType::Json,
            "raster" => SourceType::Raster,
            "raster-dem" => SourceType::RasterDem,
            "sensor" => SourceType::Sensor,
            "markers" => SourceType::Markers,
            _ => SourceType::Unknown,
        }
    }
}
impl<'de> Deserialize<'de> for SourceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize from a string
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(SourceType::from(s.as_str()))
    }
}

/// Store the encoding of the data
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Encoding {
    /// No encoding
    #[default]
    None = 0,
    /// Gzip encoding
    Gzip = 1,
    /// Brotli encoding
    #[serde(rename = "br")]
    Brotli = 2,
    /// Zstd encoding
    Zstd = 3,
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
impl From<Encoding> for &str {
    fn from(encoding: Encoding) -> Self {
        match encoding {
            Encoding::Gzip => "gzip",
            Encoding::Brotli => "br",
            Encoding::Zstd => "zstd",
            Encoding::None => "none",
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
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct VectorLayer {
    /// The id of the layer
    pub id: String,
    /// The description of the layer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The min zoom of the layer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minzoom: Option<u8>,
    /// The max zoom of the layer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxzoom: Option<u8>,
    /// Information about each field property
    pub fields: BTreeMap<String, String>,
}

/// Default S2 tile scheme is `fzxy`
/// Default Web Mercator tile scheme is `xyz`
/// Adding a t prefix to the scheme will change the request to be time sensitive
/// TMS is an oudated version that is not supported by s2maps-gpu
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Scheme {
    /// The default scheme with faces (S2)
    #[default]
    Fzxy,
    /// The time sensitive scheme with faces (S2)
    Tfzxy,
    /// The basic scheme (Web Mercator)
    Xyz,
    /// The time sensitive basic scheme (Web Mercator)
    Txyz,
    /// The TMS scheme
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
impl From<Scheme> for &str {
    fn from(scheme: Scheme) -> Self {
        match scheme {
            Scheme::Fzxy => "fzxy",
            Scheme::Tfzxy => "tfzxy",
            Scheme::Xyz => "xyz",
            Scheme::Txyz => "txyz",
            Scheme::Tms => "tms",
        }
    }
}

/// Store where the center of the data lives
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Center {
    /// The longitude of the center
    pub lon: f64,
    /// The latitude of the center
    pub lat: f64,
    /// The zoom of the center
    pub zoom: u8,
}

/// Metadata for the tile data
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Metadata {
    /// The version of the s2-tilejson spec
    #[serde(default)]
    pub s2tilejson: String,
    /// The version of the data
    #[serde(default)]
    pub version: String,
    /// The name of the data
    #[serde(default)]
    pub name: String,
    /// The scheme of the data
    #[serde(default)]
    pub scheme: Scheme,
    /// The description of the data
    #[serde(default)]
    pub description: String,
    /// The type of the data
    #[serde(rename = "type", default)]
    pub type_: SourceType,
    /// The extension to use when requesting a tile
    #[serde(default)]
    pub extension: String,
    /// The encoding of the data
    #[serde(default)]
    pub encoding: Encoding,
    /// List of faces that have data
    #[serde(default)]
    pub faces: Vec<Face>,
    /// WM Tile fetching bounds. Helpful to not make unecessary requests for tiles we know don't exist
    #[serde(default)]
    pub bounds: WMBounds,
    /// S2 Tile fetching bounds. Helpful to not make unecessary requests for tiles we know don't exist
    #[serde(default)]
    pub facesbounds: FaceBounds,
    /// minzoom at which to request tiles. [default=0]
    #[serde(default)]
    pub minzoom: u8,
    /// maxzoom at which to request tiles. [default=27]
    #[serde(default)]
    pub maxzoom: u8,
    /// The center of the data
    #[serde(default)]
    pub center: Center,
    /// { ['human readable string']: 'href' }
    #[serde(default)]
    pub attribution: Attribution,
    /// Track layer metadata
    #[serde(default)]
    pub layers: LayersMetaData,
    /// Track tile stats for each face and total overall
    #[serde(default)]
    pub tilestats: TileStatsMetadata,
    /// Old spec, track basic layer metadata
    #[serde(default)]
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
            extension: "pbf".into(),
            encoding: Encoding::default(),
            faces: Vec::new(),
            bounds: WMBounds::default(),
            facesbounds: FaceBounds::default(),
            minzoom: 0,
            maxzoom: 27,
            center: Center::default(),
            attribution: BTreeMap::new(),
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
            lon_lat_bounds: BBox {
                left: f64::INFINITY,
                bottom: f64::INFINITY,
                right: -f64::INFINITY,
                top: -f64::INFINITY,
            },
            faces: BTreeSet::new(),
            metadata: Metadata {
                minzoom: 30,
                maxzoom: 0,
                ..Metadata::default()
            },
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

    /// Set the extension of the data. [default=pbf]
    pub fn set_extension(&mut self, extension: String) {
        self.metadata.extension = extension;
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

    /// add an attribution
    pub fn add_attribution(&mut self, display_name: &str, href: &str) {
        self.metadata
            .attribution
            .insert(display_name.into(), href.into());
    }

    /// Add the layer metadata
    pub fn add_layer(&mut self, name: &str, layer: &LayerMetaData) {
        // Only insert if the key does not exist
        if self
            .metadata
            .layers
            .entry(name.into())
            .or_insert(layer.clone())
            .eq(&layer)
        {
            // Also add to vector_layers only if the key was not present and the insert was successful
            self.metadata.vector_layers.push(VectorLayer {
                id: name.into(), // No need to clone again; we use the moved value
                description: layer.description.clone(),
                minzoom: Some(layer.minzoom),
                maxzoom: Some(layer.maxzoom),
                fields: BTreeMap::new(),
            });
        }
        // update minzoom and maxzoom
        if layer.minzoom < self.metadata.minzoom {
            self.metadata.minzoom = layer.minzoom;
        }
        if layer.maxzoom > self.metadata.maxzoom {
            self.metadata.maxzoom = layer.maxzoom;
        }
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
        let Metadata {
            minzoom, maxzoom, ..
        } = self.metadata;
        let BBox {
            left,
            bottom,
            right,
            top,
        } = self.lon_lat_bounds;
        self.metadata.center.lon = (left + right) / 2.0;
        self.metadata.center.lat = (bottom + top) / 2.0;
        self.metadata.center.zoom = (minzoom + maxzoom) >> 1;
    }

    /// Add the bounds of the tile for WM data
    fn add_bounds_wm(&mut self, zoom: u8, x: u32, y: u32) {
        let x = x as u64;
        let y = y as u64;
        let bbox = self.metadata.bounds.entry(zoom).or_insert(BBox {
            left: u64::MAX,
            bottom: u64::MAX,
            right: 0,
            top: 0,
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
        let bbox = self
            .metadata
            .facesbounds
            .get_mut(face)
            .entry(zoom)
            .or_insert(BBox {
                left: u64::MAX,
                bottom: u64::MAX,
                right: 0,
                top: 0,
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
        meta_builder.set_extension("pbf".into());
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
        let shape: Shape =
            serde_json::from_str(shape_str).unwrap_or_else(|e| panic!("ERROR: {}", e));
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
        meta_builder.add_tile_wm(
            0,
            0,
            0,
            &LonLatBounds {
                left: -60.0,
                bottom: -20.0,
                right: 5.0,
                top: 60.0,
            },
        );
        // S2:
        meta_builder.add_tile_s2(
            Face::Face1,
            5,
            22,
            37,
            &LonLatBounds {
                left: -120.0,
                bottom: -7.0,
                right: 44.0,
                top: 72.0,
            },
        );

        // finally to get the resulting metadata:
        let resulting_metadata: Metadata = meta_builder.commit();

        assert_eq!(
            resulting_metadata,
            Metadata {
                name: "OSM".into(),
                description: "A free editable map of the whole world.".into(),
                version: "1.0.0".into(),
                scheme: "fzxy".into(),
                type_: "vector".into(),
                encoding: "none".into(),
                extension: "pbf".into(),
                attribution: BTreeMap::from([(
                    "OpenStreetMap".into(),
                    "https://www.openstreetmap.org/copyright/".into()
                ),]),
                bounds: BTreeMap::from([(
                    0,
                    TileBounds {
                        left: 0,
                        bottom: 0,
                        right: 0,
                        top: 0
                    }
                ),]),
                faces: Vec::from(&[Face::Face0, Face::Face1]),
                facesbounds: FaceBounds {
                    face0: BTreeMap::new(),
                    face1: BTreeMap::from([(
                        5,
                        TileBounds {
                            left: 22,
                            bottom: 37,
                            right: 22,
                            top: 37
                        }
                    ),]),
                    face2: BTreeMap::new(),
                    face3: BTreeMap::new(),
                    face4: BTreeMap::new(),
                    face5: BTreeMap::new(),
                },
                minzoom: 0,
                maxzoom: 13,
                center: Center {
                    lon: -38.0,
                    lat: 26.0,
                    zoom: 6
                },
                tilestats: TileStatsMetadata {
                    total: 2,
                    total_0: 0,
                    total_1: 1,
                    total_2: 0,
                    total_3: 0,
                    total_4: 0,
                    total_5: 0,
                },
                layers: BTreeMap::from([(
                    "water_lines".into(),
                    LayerMetaData {
                        description: Some("water_lines".into()),
                        minzoom: 0,
                        maxzoom: 13,
                        draw_types: Vec::from(&[DrawType::Lines]),
                        shape: BTreeMap::from([
                            ("class".into(), ShapeType::Primitive(PrimitiveShape::String)),
                            ("offset".into(), ShapeType::Primitive(PrimitiveShape::F64)),
                            (
                                "info".into(),
                                ShapeType::Nested(BTreeMap::from([
                                    ("name".into(), ShapeType::Primitive(PrimitiveShape::String)),
                                    ("value".into(), ShapeType::Primitive(PrimitiveShape::I64)),
                                ]))
                            ),
                        ]),
                        m_shape: None,
                    }
                )]),
                s2tilejson: "1.0.0".into(),
                vector_layers: Vec::from([VectorLayer {
                    id: "water_lines".into(),
                    description: Some("water_lines".into()),
                    minzoom: Some(0),
                    maxzoom: Some(13),
                    fields: BTreeMap::new()
                }]),
            }
        );
    }

    #[test]
    fn test_face() {
        assert_eq!(Face::Face0, Face::from(0));
        assert_eq!(Face::Face1, Face::from(1));
        assert_eq!(Face::Face2, Face::from(2));
        assert_eq!(Face::Face3, Face::from(3));
        assert_eq!(Face::Face4, Face::from(4));
        assert_eq!(Face::Face5, Face::from(5));

        assert_eq!(0, u8::from(Face::Face0));
        assert_eq!(1, u8::from(Face::Face1));
        assert_eq!(2, u8::from(Face::Face2));
        assert_eq!(3, u8::from(Face::Face3));
        assert_eq!(4, u8::from(Face::Face4));
        assert_eq!(5, u8::from(Face::Face5));
    }

    #[test]
    fn test_bbox() {
        let bbox: BBox = BBox {
            left: 0.0,
            bottom: 0.0,
            right: 0.0,
            top: 0.0,
        };
        // serialize to JSON and back
        let json = serde_json::to_string(&bbox).unwrap();
        assert_eq!(json, r#"[0.0,0.0,0.0,0.0]"#);
        let bbox2: BBox = serde_json::from_str(&json).unwrap();
        assert_eq!(bbox, bbox2);
    }

    // TileStatsMetadata
    #[test]
    fn test_tilestats() {
        let mut tilestats = TileStatsMetadata {
            total: 2,
            total_0: 0,
            total_1: 1,
            total_2: 0,
            total_3: 0,
            total_4: 0,
            total_5: 0,
        };
        // serialize to JSON and back
        let json = serde_json::to_string(&tilestats).unwrap();
        assert_eq!(json, r#"{"total":2,"0":0,"1":1,"2":0,"3":0,"4":0,"5":0}"#);
        let tilestats2: TileStatsMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(tilestats, tilestats2);

        // get0
        assert_eq!(tilestats.get(0.into()), 0);
        // increment0
        tilestats.increment(0.into());
        assert_eq!(tilestats.get(0.into()), 1);

        // get 1
        assert_eq!(tilestats.get(1.into()), 1);
        // increment 1
        tilestats.increment(1.into());
        assert_eq!(tilestats.get(1.into()), 2);

        // get 2
        assert_eq!(tilestats.get(2.into()), 0);
        // increment 2
        tilestats.increment(2.into());
        assert_eq!(tilestats.get(2.into()), 1);

        // get 3
        assert_eq!(tilestats.get(3.into()), 0);
        // increment 3
        tilestats.increment(3.into());
        assert_eq!(tilestats.get(3.into()), 1);

        // get 4
        assert_eq!(tilestats.get(4.into()), 0);
        // increment 4
        tilestats.increment(4.into());
        assert_eq!(tilestats.get(4.into()), 1);

        // get 5
        assert_eq!(tilestats.get(5.into()), 0);
        // increment 5
        tilestats.increment(5.into());
        assert_eq!(tilestats.get(5.into()), 1);
    }

    // FaceBounds
    #[test]
    fn test_facebounds() {
        let mut facebounds = FaceBounds::default();
        // get mut
        let face0 = facebounds.get_mut(0.into());
        face0.insert(
            0,
            TileBounds {
                left: 0,
                bottom: 0,
                right: 0,
                top: 0,
            },
        );
        // get mut 1
        let face1 = facebounds.get_mut(1.into());
        face1.insert(
            0,
            TileBounds {
                left: 0,
                bottom: 0,
                right: 1,
                top: 1,
            },
        );
        // get mut 2
        let face2 = facebounds.get_mut(2.into());
        face2.insert(
            0,
            TileBounds {
                left: 0,
                bottom: 0,
                right: 2,
                top: 2,
            },
        );
        // get mut 3
        let face3 = facebounds.get_mut(3.into());
        face3.insert(
            0,
            TileBounds {
                left: 0,
                bottom: 0,
                right: 3,
                top: 3,
            },
        );
        // get mut 4
        let face4 = facebounds.get_mut(4.into());
        face4.insert(
            0,
            TileBounds {
                left: 0,
                bottom: 0,
                right: 4,
                top: 4,
            },
        );
        // get mut 5
        let face5 = facebounds.get_mut(5.into());
        face5.insert(
            0,
            TileBounds {
                left: 0,
                bottom: 0,
                right: 5,
                top: 5,
            },
        );

        // now get for all 5:
        // get 0
        assert_eq!(
            facebounds.get(0.into()).get(&0).unwrap(),
            &TileBounds {
                left: 0,
                bottom: 0,
                right: 0,
                top: 0
            }
        );
        // get 1
        assert_eq!(
            facebounds.get(1.into()).get(&0).unwrap(),
            &TileBounds {
                left: 0,
                bottom: 0,
                right: 1,
                top: 1
            }
        );
        // get 2
        assert_eq!(
            facebounds.get(2.into()).get(&0).unwrap(),
            &TileBounds {
                left: 0,
                bottom: 0,
                right: 2,
                top: 2
            }
        );
        // get 3
        assert_eq!(
            facebounds.get(3.into()).get(&0).unwrap(),
            &TileBounds {
                left: 0,
                bottom: 0,
                right: 3,
                top: 3
            }
        );
        // get 4
        assert_eq!(
            facebounds.get(4.into()).get(&0).unwrap(),
            &TileBounds {
                left: 0,
                bottom: 0,
                right: 4,
                top: 4
            }
        );
        // get 5
        assert_eq!(
            facebounds.get(5.into()).get(&0).unwrap(),
            &TileBounds {
                left: 0,
                bottom: 0,
                right: 5,
                top: 5
            }
        );

        // serialize to JSON and back
        let json = serde_json::to_string(&facebounds).unwrap();
        assert_eq!(json, "{\"0\":{\"0\":[0,0,0,0]},\"1\":{\"0\":[0,0,1,1]},\"2\":{\"0\":[0,0,2,2]},\"3\":{\"0\":[0,0,3,3]},\"4\":{\"0\":[0,0,4,4]},\"5\":{\"0\":[0,0,5,5]}}");
        let facebounds2 = serde_json::from_str(&json).unwrap();
        assert_eq!(facebounds, facebounds2);
    }

    // DrawType
    #[test]
    fn test_drawtype() {
        assert_eq!(DrawType::from(1), DrawType::Points);
        assert_eq!(DrawType::from(2), DrawType::Lines);
        assert_eq!(DrawType::from(3), DrawType::Polygons);
        assert_eq!(DrawType::from(4), DrawType::Points3D);
        assert_eq!(DrawType::from(5), DrawType::Lines3D);
        assert_eq!(DrawType::from(6), DrawType::Polygons3D);
        assert_eq!(DrawType::from(7), DrawType::Points);

        assert_eq!(1, u8::from(DrawType::Points));
        assert_eq!(2, u8::from(DrawType::Lines));
        assert_eq!(3, u8::from(DrawType::Polygons));
        assert_eq!(4, u8::from(DrawType::Points3D));
        assert_eq!(5, u8::from(DrawType::Lines3D));
        assert_eq!(6, u8::from(DrawType::Polygons3D));

        // check json is the number value
        let json = serde_json::to_string(&DrawType::Points).unwrap();
        assert_eq!(json, "1");
        let drawtype: DrawType = serde_json::from_str(&json).unwrap();
        assert_eq!(drawtype, DrawType::Points);

        let drawtype: DrawType = serde_json::from_str("2").unwrap();
        assert_eq!(drawtype, DrawType::Lines);

        let drawtype: DrawType = serde_json::from_str("3").unwrap();
        assert_eq!(drawtype, DrawType::Polygons);

        let drawtype: DrawType = serde_json::from_str("4").unwrap();
        assert_eq!(drawtype, DrawType::Points3D);

        let drawtype: DrawType = serde_json::from_str("5").unwrap();
        assert_eq!(drawtype, DrawType::Lines3D);

        let drawtype: DrawType = serde_json::from_str("6").unwrap();
        assert_eq!(drawtype, DrawType::Polygons3D);

        assert!(serde_json::from_str::<DrawType>("7").is_err());
    }

    // SourceType
    #[test]
    fn test_sourcetype() {
        // from string
        assert_eq!(SourceType::from("vector"), SourceType::Vector);
        assert_eq!(SourceType::from("json"), SourceType::Json);
        assert_eq!(SourceType::from("raster"), SourceType::Raster);
        assert_eq!(SourceType::from("raster-dem"), SourceType::RasterDem);
        assert_eq!(SourceType::from("sensor"), SourceType::Sensor);
        assert_eq!(SourceType::from("markers"), SourceType::Markers);
        assert_eq!(SourceType::from("overlay"), SourceType::Unknown);

        // json vector
        let json = serde_json::to_string(&SourceType::Vector).unwrap();
        assert_eq!(json, "\"vector\"");
        let sourcetype: SourceType = serde_json::from_str(&json).unwrap();
        assert_eq!(sourcetype, SourceType::Vector);

        // json json
        let json = serde_json::to_string(&SourceType::Json).unwrap();
        assert_eq!(json, "\"json\"");
        let sourcetype: SourceType = serde_json::from_str(&json).unwrap();
        assert_eq!(sourcetype, SourceType::Json);

        // json raster
        let json = serde_json::to_string(&SourceType::Raster).unwrap();
        assert_eq!(json, "\"raster\"");
        let sourcetype: SourceType = serde_json::from_str(&json).unwrap();
        assert_eq!(sourcetype, SourceType::Raster);

        // json raster-dem
        let json = serde_json::to_string(&SourceType::RasterDem).unwrap();
        assert_eq!(json, "\"raster-dem\"");
        let sourcetype: SourceType = serde_json::from_str(&json).unwrap();
        assert_eq!(sourcetype, SourceType::RasterDem);

        // json sensor
        let json = serde_json::to_string(&SourceType::Sensor).unwrap();
        assert_eq!(json, "\"sensor\"");
        let sourcetype: SourceType = serde_json::from_str(&json).unwrap();
        assert_eq!(sourcetype, SourceType::Sensor);

        // json markers
        let json = serde_json::to_string(&SourceType::Markers).unwrap();
        assert_eq!(json, "\"markers\"");
        let sourcetype: SourceType = serde_json::from_str(&json).unwrap();
        assert_eq!(sourcetype, SourceType::Markers);

        // json unknown
        let json = serde_json::to_string(&SourceType::Unknown).unwrap();
        assert_eq!(json, "\"unknown\"");
        let sourcetype: SourceType = serde_json::from_str(r#""overlay""#).unwrap();
        assert_eq!(sourcetype, SourceType::Unknown);
    }

    // Encoding
    #[test]
    fn test_encoding() {
        // from string
        assert_eq!(Encoding::from("none"), Encoding::None);
        assert_eq!(Encoding::from("gzip"), Encoding::Gzip);
        assert_eq!(Encoding::from("br"), Encoding::Brotli);
        assert_eq!(Encoding::from("zstd"), Encoding::Zstd);

        // to string
        assert_eq!(core::convert::Into::<&str>::into(Encoding::None), "none");
        assert_eq!(core::convert::Into::<&str>::into(Encoding::Gzip), "gzip");
        assert_eq!(core::convert::Into::<&str>::into(Encoding::Brotli), "br");
        assert_eq!(core::convert::Into::<&str>::into(Encoding::Zstd), "zstd");

        // from u8
        assert_eq!(Encoding::from(0), Encoding::None);
        assert_eq!(Encoding::from(1), Encoding::Gzip);
        assert_eq!(Encoding::from(2), Encoding::Brotli);
        assert_eq!(Encoding::from(3), Encoding::Zstd);

        // to u8
        assert_eq!(u8::from(Encoding::None), 0);
        assert_eq!(u8::from(Encoding::Gzip), 1);
        assert_eq!(u8::from(Encoding::Brotli), 2);
        assert_eq!(u8::from(Encoding::Zstd), 3);

        // json gzip
        let json = serde_json::to_string(&Encoding::Gzip).unwrap();
        assert_eq!(json, "\"gzip\"");
        let encoding: Encoding = serde_json::from_str(&json).unwrap();
        assert_eq!(encoding, Encoding::Gzip);

        // json br
        let json = serde_json::to_string(&Encoding::Brotli).unwrap();
        assert_eq!(json, "\"br\"");
        let encoding: Encoding = serde_json::from_str(&json).unwrap();
        assert_eq!(encoding, Encoding::Brotli);

        // json none
        let json = serde_json::to_string(&Encoding::None).unwrap();
        assert_eq!(json, "\"none\"");
        let encoding: Encoding = serde_json::from_str(&json).unwrap();
        assert_eq!(encoding, Encoding::None);

        // json zstd
        let json = serde_json::to_string(&Encoding::Zstd).unwrap();
        assert_eq!(json, "\"zstd\"");
        let encoding: Encoding = serde_json::from_str(&json).unwrap();
        assert_eq!(encoding, Encoding::Zstd);
    }

    // Scheme
    #[test]
    fn test_scheme() {
        // from string
        assert_eq!(Scheme::from("fzxy"), Scheme::Fzxy);
        assert_eq!(Scheme::from("tfzxy"), Scheme::Tfzxy);
        assert_eq!(Scheme::from("xyz"), Scheme::Xyz);
        assert_eq!(Scheme::from("txyz"), Scheme::Txyz);
        assert_eq!(Scheme::from("tms"), Scheme::Tms);

        // to string
        assert_eq!(core::convert::Into::<&str>::into(Scheme::Fzxy), "fzxy");
        assert_eq!(core::convert::Into::<&str>::into(Scheme::Tfzxy), "tfzxy");
        assert_eq!(core::convert::Into::<&str>::into(Scheme::Xyz), "xyz");
        assert_eq!(core::convert::Into::<&str>::into(Scheme::Txyz), "txyz");
        assert_eq!(core::convert::Into::<&str>::into(Scheme::Tms), "tms");
    }

    #[test]
    fn test_tippecanoe_metadata() {
        let meta_str = r#"{
            "name": "test_fixture_1.pmtiles",
            "description": "test_fixture_1.pmtiles",
            "version": "2",
            "type": "overlay",
            "generator": "tippecanoe v2.5.0",
            "generator_options": "./tippecanoe -zg -o test_fixture_1.pmtiles --force",
            "vector_layers": [
                {
                    "id": "test_fixture_1pmtiles",
                    "description": "",
                    "minzoom": 0,
                    "maxzoom": 0,
                    "fields": {}
                }
            ],
            "tilestats": {
                "layerCount": 1,
                "layers": [
                    {
                        "layer": "test_fixture_1pmtiles",
                        "count": 1,
                        "geometry": "Polygon",
                        "attributeCount": 0,
                        "attributes": []
                    }
                ]
            }
        }"#;

        let _meta: Metadata =
            serde_json::from_str(meta_str).unwrap_or_else(|e| panic!("ERROR: {}", e));
    }
}
