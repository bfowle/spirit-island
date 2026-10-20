//! The Spirit Island icon set (© Greater Than Games, fetched from the Spirit
//! Island Wiki for this personal tool; `frontend/src/assets/icons/README.md`),
//! registered with the renderer as glyphs. SVGs are masks tinted by meaning;
//! element PNGs keep their colors.

use si_ui::IconSource;

pub const BLIGHT: u16 = 1;
pub const FEAR: u16 = 2;
pub const EXPLORER: u16 = 3;
pub const TOWN: u16 = 4;
pub const CITY: u16 = 5;
pub const DAHAN: u16 = 6;
pub const SACRED_SITE: u16 = 7;

macro_rules! icon {
    ($f:literal) => {
        include_bytes!(concat!("../../../apps/web/src/assets/icons/", $f)).to_vec()
    };
}

/// An image of the game's art: its id with the renderer and its pixel size.
#[derive(Debug, Clone, Copy)]
pub struct Art {
    pub id: u16,
    pub w: u32,
    pub h: u32,
}

/// The game's own art (the Steam game's sprites, extracted for this personal
/// tool into the art folder): every PNG there, as images for the renderer.
/// Returns the PNGs and the ids with sizes by file stem.
pub fn art(dir: &std::path::Path) -> (Vec<(u16, Vec<u8>)>, std::collections::HashMap<String, Art>) {
    let mut images = Vec::new();
    let mut ids = std::collections::HashMap::new();
    let Ok(rd) = std::fs::read_dir(dir) else { return (images, ids) };
    let mut files: Vec<std::path::PathBuf> = rd.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "png")).collect();
    files.sort();
    for (i, f) in files.iter().enumerate() {
        let (Some(stem), Ok(bytes)) = (f.file_stem().map(|s| s.to_string_lossy().to_string()), std::fs::read(f)) else { continue };
        // The size from the PNG header (IHDR: width and height, big-endian).
        if bytes.len() < 24 || &bytes[..4] != b"\x89PNG" {
            continue;
        }
        let w = u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
        let h = u32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);
        let id = 100 + i as u16;
        images.push((id, bytes));
        ids.insert(stem, Art { id, w, h });
    }
    (images, ids)
}

pub fn all() -> Vec<(u16, IconSource)> {
    vec![
        (BLIGHT, IconSource::Mask(icon!("resource-blight.svg"))),
        (FEAR, IconSource::Mask(icon!("resource-fear.svg"))),
        (EXPLORER, IconSource::Mask(icon!("unit-explorer.svg"))),
        (TOWN, IconSource::Mask(icon!("unit-town.svg"))),
        (CITY, IconSource::Mask(icon!("unit-city.svg"))),
        (DAHAN, IconSource::Mask(icon!("unit-dahan.svg"))),
        (SACRED_SITE, IconSource::Mask(icon!("resource-sacred-site.svg"))),
    ]
}
