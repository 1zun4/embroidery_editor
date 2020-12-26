use std::path::{Path, PathBuf};
use std::fs;
use nom_derive::Nom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct PesFile {
    pub file_path: PathBuf,
    pub pes_data: PesData
}

#[derive(Nom, Debug, Clone)]
pub struct PesData {
    #[nom(Count="4")]
    pub identification: Vec<u8>,
    #[nom(Count="4")]
    pub version: Vec<u8>,

    pub pec_section: u32,
     // #[nom(MoveAbs="pec_section")]
     // pub pec_data: PecData,

    pub hoop_size_indicator: u16,
    #[nom(Count="2")]
    pub subversion: Vec<u8>,
    pub description: Description,
    pub optimize_hoop_change: u16,
    pub design_page_custom: u16,
    pub hoop_size: HoopSize,
    pub hoop_rotation: u16,
    pub design_width: u16,
    pub design_height: u16,
    pub design_page_width: u16,
    pub design_page_height: u16,
    unk1: u16,
    pub design_page_background_color: u16,
    pub design_page_foreground_color: u16,
    pub show_grid: u16,
    pub with_axes: u16,
    pub snap_to_grid: u16,
    pub grid_interval: u16,
    unk2: u16,
    pub optimize_entry_exit_points: u16,

}

#[derive(Nom, Debug, Clone)]
pub struct HoopSize {
    pub width: u16,
    pub height: u16
}

#[derive(Nom, Debug, Clone)]
pub struct Description {
    design_len: u8,
    #[nom(Count="design_len")]
    pub design: Vec<u8>,

    category_len: u8,
    #[nom(Count="category_len")]
    pub category: Vec<u8>,

    author_len: u8,
    #[nom(Count="author_len")]
    pub author: Vec<u8>,

    keywords_len: u8,
    #[nom(Count="keywords_len")]
    pub keywords: Vec<u8>,

    comments_len: u8,
    #[nom(Count="comments_len")]
    pub comments: Vec<u8>,

}

#[derive(Nom, Debug, Clone)]
pub struct PecData {
    #[nom(Count="19")]
    pub label: Vec<u8>,
    #[nom(Count="1")]
    pub carriage: Vec<u8>,
}