mod utils;
extern crate base64;
extern crate image;
use base64::encode;
use image::DynamicImage;
use image::ImageFormat;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::panic;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn load_image_from_array(_array: &[u8]) -> DynamicImage {
    let image = match image::load_from_memory_with_format(_array, ImageFormat::Png) {
        Ok(img) => img,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };
    image
}

fn get_image_as_base64(_img: DynamicImage) -> String {
    let mut c = Cursor::new(Vec::new());
    match _img.write_to(&mut c, ImageFormat::Png) {
        Ok(c) => c,
        Err(error) => {
            panic!("There was a problem writing the result buffer: {:?}", error)
        }
    }

    c.seek(SeekFrom::Start(0)).unwrap();
    let mut out = Vec::new();
    c.read_to_end(&mut out).unwrap();
    let stt = encode(&mut out);
    let together = format!("{}{}", "data:image/png;base64", stt);
    together
}

#[wasm_bindgen]
pub fn grayscale(_array: &[u8]) -> Result<(), JsValue> {
    let mut img = load_image_from_array(_array);
    img = img.grayscale();
    let base64_str = get_image_as_base64(img);
    append_img(base64_str)
}

#[wasm_bindgen]
pub fn append_img(img_src: String) -> Result<(), JsValue> {
    let window = web_sys::window().expect("n global window exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element("img")?;
    val.set_attribute("src", &img_src)?;
    val.set_attribute("style", "height: 200px")?;
    body.append_child(&val)?;
    Ok(())
}
