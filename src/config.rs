// Copyright © Shaipe

//! The MIT License (MIT)

use serde_derive::Deserialize;
use std::io::prelude::*;
use std::fs::File;
#[derive(Debug,Clone,Deserialize)]
pub struct MutilThumb{
    pub width:u32,
    pub height:u32
}
#[derive(Debug, Clone, Deserialize)]
pub struct ThumbConfig{
    pub isthumb:bool,
    pub targetdir:String,
    pub extension:String,
    pub mutilthumb:Vec<MutilThumb>

}

impl ThumbConfig{
    pub fn default()->Self{
        ThumbConfig{
            isthumb:false,
            targetdir:String::from("."),
            extension:String::from("gif|jpg|jpeg|png"),
            mutilthumb:Vec::new()
        }
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct WaterConfig{
    pub iswatermark:bool,
    pub watertype:String,
    pub watertext:String,
    pub waterpos:String,
    pub waterfont:String,
    pub watercolor:String
}
impl WaterConfig{
    pub fn default()->Self{
        WaterConfig{
            iswatermark:false,
            watertype:String::from(""),
            watertext:String::from(""),
            waterpos:String::from(""),
            waterfont:String::from(""),
            watercolor:String::from("")
        }
    }
}
// 业务配置信息
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub title: String,
    pub thumb: Option<ThumbConfig>,
    pub water: Option<WaterConfig>
}


impl Config {
    // 加载配置
    pub fn new(config_path: &str) -> Self {
        let file_path =  if config_path.is_empty() {
            "upload.conf"
        }
        else{
            config_path
        };

        // 打开文件
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("no such file {} exception: {}", file_path, e)
        };

        // 读取文件到字符串变量
        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file:{}", e)
        };
        let c: Config = serde_json::from_str(&str_val).unwrap();
        c
    }
}

