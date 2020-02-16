use std::fs;

use crate::lib::Sphere;
use crate::lib::Sun;
use crate::lib::Vector3;
use crate::lib::Color;
use crate::lib::ImageConfig;
use crate::lib::Bulb;

pub struct Image {
    pub suns: Vec<Sun>,
    pub bulbs: Vec<Bulb>,
    pub spheres: Vec<Sphere>,
    pub cfg: ImageConfig,
    pub bounces: i32,
}

pub fn parse(filename: &String) -> Image {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    let mut color = Color::white();

    let mut bulbs = Vec::<Bulb>::new();
    let mut suns = Vec::<Sun>::new();
    let mut spheres = Vec::<Sphere>::new();
    let mut cfg = ImageConfig { width: 0, height: 0, filename: "test.png".to_string() };

    let mut shininess = Color::black();
    let mut transparency = Color::black();
    let mut bounces = 4;

    for line in contents.lines() {
        if line != "" {
            let words: Vec<&str> = line.split(" ").collect();
            //println!("{:?}", words);
            let word = words[0];
            if word == "png" {
                let width = words[1].parse::<u32>().unwrap();
                let height = words[2].parse::<u32>().unwrap();
                let fname = words[3];
                cfg = ImageConfig { width: width, height: height, filename: fname.to_string() };
            }
            else if word == "sphere" {
                let x = words[1].parse::<f64>().unwrap();
                let y = words[2].parse::<f64>().unwrap();
                let z = words[3].parse::<f64>().unwrap();
                let center = Vector3 {x: x, y: y, z: z};
                let radius = words[4].parse::<f64>().unwrap();
                let sphere_color = color.clone();
                spheres.push(Sphere::new(center, radius, sphere_color, shininess.clone(), transparency.clone()));
            }
            else if word == "sun" {
                let x = words[1].parse::<f64>().unwrap();
                let y = words[2].parse::<f64>().unwrap();
                let z = words[3].parse::<f64>().unwrap();
                let direction = Vector3 {x: x, y: y, z: z};
                let sun_color = color.clone();
                suns.push(Sun::new(direction, sun_color));
            }
            else if word == "color" {
                let r = words[1].parse::<f64>().unwrap();
                let g = words[2].parse::<f64>().unwrap();
                let b = words[3].parse::<f64>().unwrap();
                color.r = r;
                color.g = g;
                color.b = b;
            }
            else if word == "bulb" {
                let x = words[1].parse::<f64>().unwrap();
                let y = words[2].parse::<f64>().unwrap();
                let z = words[3].parse::<f64>().unwrap();
                let position = Vector3 {x: x, y: y, z: z};
                bulbs.push(Bulb {position, color: color.clone()});
            }
            else if word == "shininess" {
                if words.len() == 2 {
                    let c = words[1].parse::<f64>().unwrap();
                    shininess.r = c;
                    shininess.g = c;
                    shininess.b = c;
                } else {
                    let r = words[1].parse::<f64>().unwrap();
                    let g = words[2].parse::<f64>().unwrap();
                    let b = words[3].parse::<f64>().unwrap();
                    shininess.r = r;
                    shininess.g = g;
                    shininess.b = b;
                }
            }
            else if word == "transparency" {
                if words.len() == 2 {
                    let c = words[1].parse::<f64>().unwrap();
                    transparency.r = c;
                    transparency.g = c;
                    transparency.b = c;
                } else {
                    let r = words[1].parse::<f64>().unwrap();
                    let g = words[2].parse::<f64>().unwrap();
                    let b = words[3].parse::<f64>().unwrap();
                    transparency.r = r;
                    transparency.g = g;
                    transparency.b = b;
                }
            }
            else if word == "bounces" {
                bounces = words[1].parse::<i32>().unwrap();
            }
        }
    }
    Image {suns, bulbs, spheres, cfg, bounces}
}
