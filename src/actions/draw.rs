
fn draw_tile(context: &Context, rgb: (f32,f32,f32), x: f64, y: f64, x2: f64, y2: f64) {
    context.set_source_rgb(rgb.0 as f64, rgb.1 as f64, rgb.2 as f64);
    context.new_path();
    context.move_to(x, y);
    context.line_to(x2, y);
    context.line_to(x, y2);
    context.move_to(x2, y2);
    context.line_to(x2, y);
    context.line_to(x, y2);
    context.close_path();
    context.fill();
}


use std::fs::File;
use std::io::prelude::*;

use cairo::{ Context, Format, ImageSurface };


pub fn draw_points_to_file(name: &String, size: (f32,f32), points: Vec<( (f32,f32), (f32,f32,f32))>  ){


    let width = size.0 as i32;
    let height = size.1 as i32;
    let surface = ImageSurface::create(Format::ARgb32, width, height).unwrap();
    let ctx = Context::new(&surface).unwrap();
    let default_output = name;



    for point in points{

        let (xpos, ypos) = point.0;

        let rgb = point.1;

        draw_tile(&ctx, rgb, xpos as f64, ypos as f64, xpos as f64 + 10., ypos as f64 + 10.);
    }


    let mut file = File::create(default_output).unwrap();

    surface.write_to_png(&mut file).unwrap();

}

use std::collections::HashMap;
use crate::DeckPosition;



pub fn draw_decks(posdecks: &Vec<DeckPosition>, nameadd: &str, colormapping: &HashMap<String, (u32,u32,u32,u32,u32)>) {

    use std::time::SystemTime;

    let width = 700.;
    let height = 700.;
    let filename = format!("images/{:?}{:?}.png", nameadd, SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() );
    let mut points: Vec<( (f32,f32), (f32,f32,f32)  )> = Vec::new();


    //display in the range of (-1,-1) to (1,1)

    for deck in posdecks{

        let pos = deck.get_position();

        let pos = (   (pos.0*width/2.) + width/2.  ,    (pos.1*height/2.) + height/2.      );

        //let rgb = deck.deck.get_color();
        let rgb = deck.deck.get_color(colormapping);

        points.push( (pos, rgb)  );

    }


    draw_points_to_file( &filename, (width,height), points);


}
