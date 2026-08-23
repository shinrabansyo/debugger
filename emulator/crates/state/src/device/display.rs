use std::collections::HashMap;

use image::{DynamicImage, GenericImage, Rgba};

use super::Device;

#[derive(Debug, Clone, Default)]
pub struct Display {
    // パレット
    palette: HashMap<u32, Rgba<u8>>,

    // 表示内容
    width: u32,
    height: u32,
    canvas: DynamicImage,
}

impl Device for Display {
    fn read(&self, _: usize) -> anyhow::Result<u32> {
        Ok(0)
    }

    fn write(&mut self, addr: usize, data: u32) -> anyhow::Result<()> {
        match addr {
            // 出力モード指定
            0x0000_0300 => {
                (self.width, self.height) = match data {
                    0x00 => (0, 0),
                    0x01 => (640, 480),
                    0x02 => (128, 128),
                    _ => (0, 0),
                };
                self.canvas = DynamicImage::new_rgb8(self.width, self.height);
            }

            // パレット設定 (16色モード)
            0x0000_0301 => {
                let idx = (data >> 24) & 0x0F;
                let r = ((data >> 16) & 0xFF) as u8;
                let g = ((data >> 8) & 0xFF) as u8;
                let b = (data & 0xFF) as u8;
                self.palette.insert(idx, Rgba([r, g, b, 0]));
            }

            // フレームバッファ (16色モード)
            0x0100_0000..=0x01FF_FFFF if self.width == 128 => {
                let x = ((addr - 0x0100_0000) % 128) as u32;
                let y = ((addr - 0x0100_0000) / 128) as u32;
                let pixel = data & 0x0F; // パレット番号
                let pixel = *self.palette.get(&pixel).unwrap_or(&Rgba([0, 0, 0, 0]));
                self.canvas.put_pixel(x, y, pixel);
            }

            _ => {}
        }
        Ok(())
    }
}

impl Display {
    pub fn get_stat(&self) -> ((u32, u32), DynamicImage) {
        ((self.width, self.height), self.canvas.clone())
    }
}
