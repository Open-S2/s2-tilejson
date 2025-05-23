# Changelog

## Version 1.0.0

- naming convention is `s2-tilejson` now
- Initial release
- support for S2 components "faces" and "s2bounds" with "wmbounds" for WM.
- Attributions are now objects instead of strings. Also an `s` is added to the end instead of `attribution` its now `attributions`
- bounds are now an object instead of an array, where we track zoom tile bounds instead of lon-lat bounds
- `vector_layers` are now **deprectated** in favor of `layers` but should still be parsable from readers for backwards compatibility
- added `layers` to better parse input data
- `center` is now an object called `centerpoint` of keys `lon`, `lat`, and `zoom` rather then the array `[lon, lat, zoom]`
- added a `type` which was misused by old engines
- `tiles` is deprecated in favor of forcing use of the same url and adding an `extension` property to the metata.
- added optional `encoding` property for more transparency in the future
- `minzoom` and `maxzoom` are now required
