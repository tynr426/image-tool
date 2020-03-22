//! An example of drawing text. Writes to the user-provided target file.

use image::{open, Rgb, RgbImage, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{FontCollection, Scale};
use std::env;
use std::path::Path;
use std::*;

// #[macro_use] extern crate std;

fn main() {
    // let arg = if env::args().count() == 2 {
    //     env::args().nth(1).unwrap()
    // } else {
    //     panic!("Please enter a target file path")
    // };
    add_water_mark("./source/5_1024.jpg","./source/33.jpg","快马批发",12.4,"BOTTOM_RIGHT");
   
    
}
fn add_water_mark(source_path:&str,target_path:&str,water_text:&str,font_size:f32,position:&str){
    let path = Path::new(target_path);
     //let mut image = image::open(input_path).unwrap();
    let mut image = open(source_path)
        .expect(&format!("Could not load image at {:?}", source_path))
        .to_rgb();
         //let mut image = img.grayscale();
    //let black = Rgba([0u8, 0u8, 0u8, 0u8]);
    let black = Rgb([7u8, 255u8, 255u8]);
        //let mut image = RgbImage::new(200, 200);
    let bytes = include_bytes!("HYXinHaiXingKaiW.ttf");
    let font = Vec::from(bytes as &[u8]);
    let font = FontCollection::from_bytes(font)
        .unwrap()
        .into_font()
        .unwrap();

    let _height = image.height() as f32;
    let _width = image.width() as f32;
    //水印x位置
    let mut xpos: u32 = 0;
    //水印y位置
    let mut ypos: u32 = 0;
    let mut _water_mark_width: f32 = 0.0;
    let mut _water_mark_height: f32 = 0.0;
    //比例
    let mut bl: f32 = 0.0;

    let tx = font_size;
    let scale = Scale { x: tx, y: tx };
    let water_mark_width = tx * water_text.len() as f32;
    let water_mark_height = 50.0;
    //计算水印图片的比率
    //取背景的1/4宽度来比较
    if (_width > water_mark_width) && (_height > water_mark_height) {
        bl = 1.0;
    } else if (_width > water_mark_width) && (_height < water_mark_height) {
        bl = _height / water_mark_height;
    } else if (_width < water_mark_width) && (_height > water_mark_height) {
        bl = _width / water_mark_width;
    } else {
        if (_width * water_mark_height) > (_height * water_mark_width) {
            bl = _height / water_mark_height;
        } else {
            bl = _width / water_mark_width;
        }
    }

    _water_mark_width = water_mark_width*bl;
    _water_mark_height = water_mark_height*bl;
    if _water_mark_width == _width {
        _water_mark_width = _water_mark_width - 10.0;
    }
    if _water_mark_height == _height {
        _water_mark_height = _water_mark_height - 10.0;
    }
    match position {
        "TOP_LEFT" => {
            xpos = 10;
            ypos = 10;
        }
        "TOP_CENTER" => {
            xpos = (_width / 2.0 - _water_mark_width / 2.0) as u32;
            ypos = 10;
        }
        "TOP_RIGHT" => {
            xpos = (_width - _water_mark_width) as u32 - 10;
            ypos = 10;
        }
        "MIDDLE_LEFT" => {
            xpos = 10;
            ypos = (_height / 2.0 - _water_mark_height / 2.0) as u32;
        }
        "MIDDLE_CENTER" => {
            xpos = (_width / 2.0 - _water_mark_width / 2.0) as u32;
            ypos = (_height / 2.0 - _water_mark_height / 2.0) as u32;
        }

        "MIDDLE_RIGHT" => {
            xpos = (_width - _water_mark_width) as u32 - 10;
            ypos = (_height / 2.0 - _water_mark_height / 2.0) as u32;
        }
        "BOTTOM_LEFT" => {
            xpos = 10;
            ypos = (_height - _water_mark_height) as u32 - 10;
        }
        "BOTTOM_CENTER" => {
            xpos = (_width / 2.0 - _water_mark_width / 2.0) as u32;
            ypos = (_height - _water_mark_height) as u32 - 10;
        }
        "BOTTOM_RIGHT" => {
            xpos = (_width - _water_mark_width) as u32 - 10;
            ypos = (_height - _water_mark_height) as u32 - 10;
        }
        _ => {}
    }

    draw_text_mut(&mut image, black, xpos, ypos, scale, &font, water_text);

    //image.blur();//高斯模糊

    let _ = image.save(path).unwrap();
}
