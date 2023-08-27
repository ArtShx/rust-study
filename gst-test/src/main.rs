use anyhow::{anyhow, Context, Error};
use gst::prelude::*;
use std::sync::{Arc, Mutex};
use tracing::info;
use tracing_subscriber::prelude::*;

// Simple wrapper around gst_element_factory_make with a more useful error message
pub fn make_element(element: &str, name: Option<&str>) -> Result<gst::Element, Error> {
use crate::common::make_element;
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

fn initialize_logging(envvar_name: &str) -> Result<(), Error> {
    tracing_log::LogTracer::init()?;
    let env_filter = tracing_subscriber::EnvFilter::try_from_env(envvar_name)
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_thread_ids(true)
        .with_target(true)
        .with_span_events(
            tracing_subscriber::fmt::format::FmtSpan::NEW
                | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
        );
    let subscriber = tracing_subscriber::Registry::default()
        .with(env_filter)
        .with(fmt_layer);
    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

// mod helloworld;
mod common;
mod simple;
mod dynamic_pipe;

fn main() -> Result<(), Error> {
    initialize_logging("GST_STUDY");
    // common::my_func();
    gst::init()?;
    // let app = helloworld::App::new()?;
    // let app = App::new()?;
    // app.run()?;
    dynamic_pipe::run()?;

    Ok(())
}
