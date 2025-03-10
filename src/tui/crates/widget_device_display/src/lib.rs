// use image::ImageReader;
use ratatui::layout::Rect;
use ratatui::text::Text;
// use ratatui_image::picker::Picker;
// use ratatui_image::{Image, Resize};

use sb_emu::State as EmuState;
use sb_dbg_tui_engine::widget::{Widget, WidgetView};

#[derive(Default)]
pub struct Display;

impl Widget for Display {
    fn draw(&self, _: &Rect, _: &EmuState) -> WidgetView {
        // let picker = Picker::from_query_stdio().unwrap();
        // let image = ImageReader::open("").unwrap().decode().unwrap();
        // let mut image = Box::new(picker.new_protocol(image, Rect::new(0, 0, 100, 100), Resize::Crop(None)).unwrap());
        // let image = Image::new(&mut image);

        // WidgetView::default().title(" Device: Display ").body(image)
        WidgetView::default().title(" Device: Display ").body(Text::raw("hello, world!"))
    }
}
