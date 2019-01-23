// Draw some sample text to the screen
extern crate quicksilver;

use quicksilver::{
    combinators::result,
    geom::{Shape, Vector},
    graphics::{Background::Img, Color, Font, FontStyle, Image},
    lifecycle::{run, Asset, Settings, State, Window},
    Future, Result,
};

struct SampleText {
    asset: Asset<Image>,
}

impl State for SampleText {
    fn new() -> Result<SampleText> {
        // NOTE: this font causes a render overflow panic:
        let font_name = "font.ttf";
        // NOTE: and this one causes a slice out of bounds panic:
        //let font_name = "mononoki-Regular.ttf";
        let asset = Asset::new(Font::load(font_name).and_then(|font| {
            let style = FontStyle::new(72.0, Color::BLACK);
            result(font.render("Ty", &style))
        }));
        Ok(SampleText { asset })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        self.asset.execute(|image| {
            window.draw(&image.area(), Img(&image));
            Ok(())
        })
    }
}

fn main() {
    run::<SampleText>("Font Example", Vector::new(800, 600), Settings::default());
}
