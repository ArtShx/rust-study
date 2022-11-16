use gst::glib::Error;

fn main() -> Result<(), Error>{
    println!("Hello, world!");
    gst::init()?;
    Ok(())
}
