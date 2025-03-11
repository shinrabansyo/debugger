use std::cell::RefCell;
use std::cmp::max;

use crossterm::event::{KeyEvent, KeyCode};
use image::{DynamicImage, ImageReader};
use ratatui::layout::Rect;
use ratatui_image::picker::Picker;
use ratatui_image::protocol::Protocol;
use ratatui_image::{Image, Resize};

use sb_emu::State as EmuState;
use sb_dbg_tui_engine::widget::{Widget, WidgetView};

pub struct Display<const W: usize, const H: usize> {
    // 表示画像
    image_src: DynamicImage,
    image_protocol: RefCell<Protocol>,

    // 領域
    x: u32,
    y: u32,
}

impl<const W: usize, const H: usize> Default for Display<W, H> {
    fn default() -> Display<W, H> {
        // 表示対象画像
        let image_src = ImageReader::open("logo.jpg").unwrap().decode().unwrap();

        // ratatui 表示用プロトコル準備 (仮)
        let image_protocol = new_picker()
            .new_protocol(
                image_src.clone(),
                Rect::new(0, 0, W as u16, H as u16),
                Resize::Crop(None)
            )
            .unwrap();

        // ウィジェット生成 & 画像の調整
        let mut display_widget = Display {
            image_src,
            image_protocol: RefCell::new(image_protocol),
            x: 0,
            y: 0,
        };
        display_widget.update_view();

        display_widget
    }
}

impl<const W: usize, const H: usize> Widget for Display<W, H> {
    fn draw(&self, _: &Rect, _: &EmuState) -> WidgetView {
        // self.image の可変借用を取り，ライフタイムの解釈を変更して十分長くする
        //   ->  WidgetView の生成から描画 (=破棄) までの短い期間の参照であれば問題ない
        let mut image_ref = self.image_protocol.borrow_mut();
        let image_ptr = &mut *image_ref;
        let static_ref = unsafe { std::mem::transmute(image_ptr) };

        // Image ウィジェット準備
        let image = Image::new(static_ref);

        WidgetView::default()
            .title(format!(" Device: Display ({}x{})", W, H))
            .body(image)
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Up => {
                self.y = max(self.y as i32 - 10, 0) as u32;
                self.update_view();
            }
            KeyCode::Down => {
                self.y = self.y + 10;
                self.update_view();
            }
            KeyCode::Left => {
                self.x = max(self.x as i32 - 10, 0) as u32;
                self.update_view();
            }
            KeyCode::Right => {
                self.x = self.x + 10;
                self.update_view();
            }
            _ => {}
        }
    }
}

impl<const W: usize, const H: usize> Display<W, H> {
    fn update_view(&mut self) {
        let new_x = self.x;
        let new_y = self.y;

        let scale_x = self.image_src.width() as f32 / (W as f32);
        let scale_y = self.image_src.height() as f32 / (H as f32);

        let new_scaled_x = ((new_x as f32) * scale_x) as u32;
        let new_scaled_y = ((new_y as f32) * scale_y) as u32;

        let new_width = self.image_src.width() - new_scaled_x;
        let new_height = self.image_src.height() - new_scaled_y;

        let image_src = self.image_src.crop_imm(new_scaled_x, new_scaled_y, new_width, new_height);
        let image_protocol = new_picker()
            .new_protocol(
                image_src,
                Rect::new(0, 0, W as u16, H as u16),
                Resize::Crop(None),
            )
            .unwrap();

        self.image_protocol = RefCell::new(image_protocol);
    }
}

fn new_picker() -> Picker {
    match Picker::from_query_stdio() {
        Ok(picker) => picker,
        Err(_) => Picker::from_fontsize((8, 12)),
    }
}
