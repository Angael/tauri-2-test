#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Thumbnail {
    pub res: (u16, u16), // Width, Height

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grid: Option<(u8, u8)>, // Columns, Rows
}

pub fn get_thumbnail_name(thumb: &Thumbnail) -> String {
    let thumb_name = match thumb {
        Thumbnail { res, grid: None } => format!("{}x{}.avif", res.0, res.1),
        Thumbnail {
            res,
            grid: Some(grid),
        } => format!("{}x{}-{}x{}.avif", res.0, res.1, grid.0, grid.1),
    };

    thumb_name
}
