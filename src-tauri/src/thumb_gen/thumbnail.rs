#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum Thumbnail {
    Single { res: (u16, u16) },
    Grid { res: (u16, u16), grid: (u8, u8) },
}

pub fn get_thumbnail_name(thumb: &Thumbnail) -> String {
    let thumb_name = match thumb {
        Thumbnail::Single { res } => format!("{}x{}.avif", res.0, res.1),
        Thumbnail::Grid { res, grid } => format!("{}x{}-{}x{}.avif", res.0, res.1, grid.0, grid.1),
    };

    thumb_name
}
