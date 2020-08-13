use std::string::String;
use std::default::Default;
use rusttype::{point, Font, Scale,PositionedGlyph};
use std::option::Option;

#[derive(Default,Copy,Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {

    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r, g: g, b: b,
        }
    }

}

#[derive(Default,Copy,Clone)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {

    pub fn new(w: u32, h: u32) -> Self {
        Self {
            width: w, height: h,
        }
    }

}

#[derive(Default,Copy,Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {

    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x: x, y: y,
        }
    }

}

#[derive(Default,Clone)]
pub struct TextLine {
    pub position: Position,
    size: Size,
    pub scale: f32,
    pub color: Color,
    rendered: bool,
    pub characters: String,
    pub pixels: Vec<u8>, // rgba
}

impl TextLine {

    pub fn render_text(&mut self, new_chars: &str, font: Font<'_>) {

        let scale = Scale::uniform(self.scale);
        let v_metrics = font.v_metrics(scale);
        let p = point(1.0f32, 1.0f32 + v_metrics.ascent);

        let glyphs = font.layout(new_chars, scale, p)
                          .collect::<Vec<PositionedGlyph<'_>>>();

        self.size.height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        self.size.width = {
            let min_x = glyphs
                .first()
                .map(|g| {
                    let bbox = g.pixel_bounding_box().unwrap();
                    bbox.min.x
                })
                .unwrap();

            let max_x = glyphs
                .last()
                .map(|g| {
                    let bbox = g.pixel_bounding_box().unwrap();
                    bbox.max.x
                })
                .unwrap();
            (max_x - min_x) as u32
        };

        self.size.width += 10; // padding at the end
        self.size.height += 10; // padding at the end

        let mut pixels: Vec<u8> = vec![];
        pixels.resize(( self.size.width*self.size.height*4 ) as usize, 0);

        for g in glyphs.iter() {
            if let Some(bbox) = g.pixel_bounding_box() {

                g.draw(|x, y, v| {

                    let x = x as usize + ( bbox.min.x as usize );
                    let y = ( y as usize + ( bbox.min.y as usize ) ) * self.size.width as usize;
                    let i = ( ( x + y ) * 4 ) as usize;

                    // place a pixel in the buffer
                    pixels[i] = self.color.r;
                    pixels[i+1] = self.color.g;
                    pixels[i+2] = self.color.b;
                    pixels[i+3] = (v * 255.0 ).floor() as u8;

                });

            }
        }

        self.rendered = true;
        self.pixels = pixels;
    }

    pub fn get_pixels(&self) -> Option<(u32, u32, &Vec<u8>)> {

        if self.rendered {
            Some((self.size.width, self.size.height, &self.pixels))
        }
        else {
            None
        }

    }

    pub fn builder() -> TextLineBuilder {
        TextLineBuilder {
            t: Self {
                ..Default::default()
            }
        }
    }

    pub fn get_size(&self) -> Size {
        self.size
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

}

pub struct TextLineBuilder {
    t: TextLine,
}

impl TextLineBuilder {

    pub fn build(&self) -> TextLine {
        self.t.clone()
    }

    pub fn with_position(&mut self, p: Position) -> &mut Self {
        self.t.position = p;
        self
    }

    pub fn with_text<S: Into<String>>(&mut self, c: S) -> &mut Self {
        self.t.characters = c.into();
        self
    }

    pub fn with_color(&mut self, c: Color) -> &mut Self {
        self.t.color = c;
        self
    }

    pub fn with_scale(&mut self, s: f32) -> &mut Self {
        self.t.scale = s;
        self
    }

}
