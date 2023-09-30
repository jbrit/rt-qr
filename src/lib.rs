use wasm_bindgen::prelude::*;
use base64::{engine::general_purpose, Engine as _, DecodeError};
use std::string::String;
use image::{DynamicImage, load_from_memory};
use rqrr::PreparedImage;

pub fn from_base64(base64: String) -> Result<Vec<u8>, DecodeError>{
    let offset = base64.find(',').unwrap_or(base64.len()) + 1;
    let mut value = base64;
    value.drain(..offset);
    general_purpose::STANDARD.decode(value)
}

#[wasm_bindgen]
pub struct ReturnType {
    pub valid: bool,
    message: String,
}

#[wasm_bindgen]
impl ReturnType {
    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }
}

#[wasm_bindgen]
pub fn find_qr(image: &str) -> ReturnType {
    let img_buffer_result = from_base64(String::from(image));
    #[allow(unused_mut)]
    let mut img_buffer: Vec<u8>;
    match img_buffer_result{
        Ok(buff) => {
            img_buffer = buff;
        },
        Err(..) => {
            return ReturnType{
                valid: false,
                message: String::from("Could Not Load Image")
            };
        }
    };
    let dynamic_image_result = load_from_memory(&img_buffer[..]);
    #[allow(unused_mut)]
    let mut dynamic_image: DynamicImage;
    match dynamic_image_result{
        Ok(buff) => {
            dynamic_image = buff;
        },
        Err(..) => {
            return ReturnType{
                valid: false,
                message: String::from("Could Not Load Image")
            };
        }
    };
    let mut prepared_img = PreparedImage::prepare(dynamic_image.to_luma8());
    let grids = prepared_img.detect_grids();
    for i in 0..grids.len() {
        let grid = grids[i].decode();
        match grid {
            Ok((_metadata, message)) => {
                return ReturnType {
                    valid: true,
                    message: message,
                };
            }
            Err(..) => {
                return ReturnType {
                    valid: false,
                    message: String::from("Could not find QR code"),
                };
            }
        }
    }
    return ReturnType {
        valid: false,
        message: String::from("Could not find QR code"),
    };
}
