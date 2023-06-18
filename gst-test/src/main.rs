use anyhow::{anyhow, Context, Error};
use gst::prelude::*;
use std::sync::{Arc, Mutex};

// Simple wrapper around gst_element_factory_make with a more useful error message
pub fn make_element(element: &str, name: Option<&str>) -> Result<gst::Element, Error> {
    gst::ElementFactory::make_with_name(element, name)
        .with_context(|| format!("Failed to make element {}", element))
}

struct State {
    pipeline: gst::Pipeline,
}

struct App {
    state: Arc<Mutex<State>>
}

impl App {
    fn new() -> Result<Self, Error> {
        Ok(Self {
            state: Arc::new(Mutex::new(State { 
                pipeline: gst::Pipeline::new(None), 
            }))
        })
    }

    fn run(&self) -> Result<(), Error> {
        let pipeline = {
            let state = self.state.lock().unwrap();
            state.pipeline.clone()
        };

        let src = make_element("videotestsrc", None)?;
        let videoscale = make_element("videoscale", None)?;
        let videocvt = make_element("videoconvert", None)?;
        let videosink = make_element("ximagesink", None)?;
        let capsfilter = make_element("capsfilter", None)?;

        capsfilter.set_property(
            "caps",
            gst::Caps::builder("video/x-raw")
                .field("width", 1280)
                .field("height", 720)
                .build()
        );

        pipeline.add_many(&[
            &src,
            &videoscale,
            &videocvt,
            &videosink,
            &capsfilter
        ])?;

        gst::Element::link_many(&[
            &src,
            &capsfilter,
            &videoscale,
            &videocvt,
            &videosink
        ])?;

        let bus = pipeline.bus().unwrap();
        pipeline.set_state(gst::State::Paused)?;

        // Wait for either End of Stream or an error on the bus
        if let Some(msg) =
            bus.timed_pop_filtered(None, &[gst::MessageType::Error, gst::MessageType::Eos]) {
            if let gst::MessageView::Error(err) = msg.view() {
                return Err(anyhow!(
                    "Error received from element {:?}: {}",
                    err.src().map(|s| s.path_string()),
                    err.error()
                ));
            }

            // info!("EOS!");
            println!("EOS");

            Ok(())
        } else {
            Err(anyhow!("Pipeline never completed"))
        }
    }
}

fn main() -> Result<(), Error>{
    println!("Hello, world!");
    gst::init()?;
    let app = App::new()?;
    app.run()?;
    Ok(())
}
