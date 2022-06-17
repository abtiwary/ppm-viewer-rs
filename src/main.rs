use std::error::Error;
use std::env;
use std::process;

use show_image::{ImageView, ImageInfo, event, create_window};

use ppm_viewer::PPMReader;

#[show_image::main]
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please specify the path to a ppm file (binary, P6)");
        process::exit(0x0100);
    }

    //let ppm_file = String::from("/Users/abtiwary/temp/ppm_images/stop01.ppm");
    let ppm_file = String::from(args[1].clone());

    let (ppm_reader, width, height) = PPMReader::from_file(&ppm_file).unwrap();

    let image_data = ppm_reader.get_image_data().unwrap();
    let image_data = image_data.borrow();
    //println!("{:?}", &image_data[0..16]);

    let image = ImageView::new(ImageInfo::rgb8(width as u32, height as u32), &image_data);
    let window = create_window("ppm image", Default::default()).unwrap();
    window.set_image("ppm image", image).unwrap();

    for event in window.event_channel()? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            println!("{:#?}", event);
            if event.input.key_code == Some(event::VirtualKeyCode::Escape) && event.input.state.is_pressed() {
                break;
            }
        }
    }

    println!("{}", "exiting!");

    Ok(())
}
