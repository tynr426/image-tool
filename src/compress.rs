
use crate::config::{ThumbConfig,WaterConfig,MutilThumb,Config};
use crate::elapsed::Elapsed;
use image::{FilterType, JPEG, PNG};
use std::fs::{self, File};

use std::time::{Duration, Instant};

use std::path::Path;

pub struct CompressPicture{
    pub cnf:Config
}
impl CompressPicture{
    pub fn new(_cnf:Config)->Self{
        CompressPicture{
            cnf:_cnf
        }
    }
    pub fn compress(&self,source:&Vec<&str>){
       
        let thumb=match self.cnf.thumb.clone(){
            Some(val) => {
                val
            },
            _ => {
                println!("数据库连接没有配置哟!");
                ThumbConfig::default()
            }
        };
        let exts: Vec<&str> = thumb.extension.split("|").collect();
        let target_path = &thumb.targetdir;
        println!("target_path={:?}",target_path);
        fs::create_dir_all(target_path).unwrap();
        for s in source {
            let img_path = Path::new(s);
            if img_path.is_file() {
                let extension = match img_path.extension() {
                    Some(ext) => ext.to_str().unwrap(),
                    _ => break,
                };
           
                if exts.contains(&extension) {//文件后缀判断
                    let file_name = img_path.file_name().unwrap().to_str().unwrap();
                   
                    let timer = Instant::now();
                    println!("target by {} in {}", file_name, Elapsed::from(&timer));
                    let tiny = match image::open(img_path) {
                        Ok(image) => image,
                        _ => {
                            println!(
                                "{} 压缩失败,图片格式有误，可以使用画图工具打开重新保存",
                                file_name
                            );
                            break;
                        }
                    };
                    for m in &thumb.mutilthumb{
                        let new_name=format!("_{}X{}.",m.width,m.height);
                        let new_name=file_name.replace(".",&new_name);
                        let scaled = tiny.resize(m.width, m.height, FilterType::Triangle);//使用这个算法进行压缩
                        let mut output = File::create(format!("{}{}",target_path,new_name)).unwrap();
                        scaled.write_to(&mut output, JPEG).unwrap();//都输出成jpg格式
                    }
                    
                }
            }
        }
    }
}