mod action;
mod addon;
mod assets;
mod bounds;
mod colors;
mod component_wise;
mod context;
mod elements;
mod id;
mod internal;
mod interval;
mod lockbox;
mod render_util;
mod schema;
mod serde_bitflags;
mod serde_migrate;
mod settings;
mod texture_manager;
mod traits;
mod tree;
mod trigger;
mod util;

use addon::Addon;

use std::{
    io::BufWriter,
    path::{Path, PathBuf},
};

use crate::elements::icon::IconSource as IS;
use elements::Element;
use schema::Schema;
use url::Url;

fn download_to_file(url: &Url, path: impl AsRef<Path>) -> PathBuf {
    let path = path.as_ref();
    // for now let it crash
    let basepath = path.parent().unwrap();
    let packname = path.file_stem().unwrap();
    let filename = url.path_segments().unwrap().last().unwrap();
    std::fs::create_dir_all(basepath.join("icons").join(&packname)).unwrap();
    let path = basepath.join("icons").join(&packname).join(filename);

    println!("Downloading {} to {}", url, path.display());
    let mut bytes = ureq::get(url.as_str()).call().unwrap().into_reader();
    let mut file = BufWriter::new(std::fs::File::create(&path).unwrap());
    std::io::copy(&mut bytes, &mut file).unwrap();
    return PathBuf::from(packname).join(filename);
}

fn download_icon(icon: &mut elements::Icon, path: impl AsRef<Path>) {
    match icon.source {
        IS::Url(ref url) => {
            let url = Url::parse(url).unwrap();
            let path = download_to_file(&url, path);
            icon.source = IS::File(path);
        }
        _ => {}
    }
}

fn iterate_icons(element: &mut Element, path: &Path) {
    use elements::ElementType as ET;
    match &mut element.kind {
        ET::Icon(ref mut iconel) => {
            download_icon(&mut iconel.icon, path);
        }
        ET::IconList(ref mut icon_list) => {
            for icon in icon_list.icons.iter_mut() {
                download_icon(&mut icon.icon, &path);
            }
        }
        ET::Group(ref mut group) => {
            for element in group.members.iter_mut() {
                iterate_icons(element, &path);
            }
        }
        _ => {}
    }
}

fn main() {
    if std::env::args().len() != 3 {
        println!(
            "Usage: {} <input> <output>",
            std::env::args().nth(0).unwrap()
        );
        return;
    }

    let input = std::env::args().nth(1).unwrap();
    let output = PathBuf::from(std::env::args().nth(2).unwrap());

    let Some(mut pack) = Schema::load_from_file(&input).map(|schema| schema.into_pack()) else {
        println!("Could not load pack from file {}", input);
        return;
    };

    for element in pack.elements.iter_mut() {
        iterate_icons(element, &output.as_path());
    }

    println!("Saving updated pack to {}", output.display());
    pack.file = output;
    pack.save_to_file();
}
