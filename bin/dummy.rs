use s2_tilejson::{DrawType, LayerMetaData, LonLatBounds, Metadata, MetadataBuilder, Shape};
use s2json::Face;

fn main() {
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
    meta_builder.add_tile_wm(
        0,
        0,
        0,
        &LonLatBounds { left: -60.0, bottom: -20.0, right: 5.0, top: 60.0 },
    );
    // S2:
    meta_builder.add_tile_s2(
        Face::Face1,
        5,
        22,
        37,
        &LonLatBounds { left: -120.0, bottom: -7.0, right: 44.0, top: 72.0 },
    );

    // finally to get the resulting metadata:
    let _resulting_metadata: Metadata = meta_builder.commit();
}
