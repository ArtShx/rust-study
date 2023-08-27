/*
*
* Make a dynamic pipeline
* 1: Create a working pipeline
* 2: Change something in the pipeline (after n seconds or after some keyboard input)
*/

use std::time::Duration;
use anyhow::Error;
use gst::prelude::*;
use tracing::info;
use glib::MainContext;

use crate::common::make_element;

fn timeout_func() -> glib::Continue{
    println!("Timeout func");
    return glib::Continue(true);
}

pub fn run() -> Result<(), Error> {

    let main_loop = glib::MainLoop::new(None, false);
    let pipeline = gst::Pipeline::new(None);

    let src = make_element("videotestsrc", Some("src"))?;
    let vcvt = make_element("videoconvert", None)?;
    let sink = make_element("autovideosink", None)?;
    // src.set_property("pattern", gst_base::GstV"ball");
    // gst_base::prelude::UnsignedIntoSigned
    // glib::EnumClass::values(&src);
    // let prop = src.find_property("pattern").unwrap().value_type();
    // println!("prop: {prop:?}");

    // GstVideoTestSrcPattern
    // gst::prelude::TaskPoolExt


    pipeline.add_many(&[&src, &vcvt, &sink])?;
    gst::Element::link_many(&[&src, &vcvt, &sink])?;


    let bus = pipeline.bus().expect("Failed to get pipeline bus");
    pipeline.set_state(gst::State::Playing)?;
    let pipeline = pipeline.dynamic_cast::<gst::Pipeline>().unwrap();



    let pipe_clone = pipeline.downgrade();
    glib::timeout_add_seconds(1 as u32, move || {
        if let Some(pipeline) = pipe_clone.upgrade() {

            let mut src_el = gst::Pipeline::by_name(&pipeline, "src");
            // if src_el.is_some() {
            if let Some(el) = src_el.as_mut() {
                pipeline.set_state(gst::State::Paused);
                // println!("working ... {src_el:?}");

                // Retrieve the current value of the "pattern" property
                // if let Ok(prop_value) = el.property_value("pattern") {
                //     if let Ok(current_pattern) = prop_value.get::<String>() {
                //         // Decide on the new pattern value (for example, "snow" -> "balls")
                //         let new_pattern = if current_pattern == "snow" {
                //             "balls"
                //         } else {
                //             "snow"
                //         };

                //         // Update the "pattern" property with the new value
                //         el.set_property_from_str("pattern", new_pattern);
                //     }
                // }

                let prop = el.property_value("pattern");
                let val = prop.get::<String>();
                println!("Val: {val:?}");
                el.set_property_from_str("pattern", "snow");
                pipeline.set_state(gst::State::Playing);

            }
        }
        glib::Continue(true)
    });

    // for (i, send_eos) in [false, true].iter().enumerate() {
    //     println!("Hi .. {i:?}");
    //     let pipeline_weak = pipeline.downgrade();
    //     glib::timeout_add_seconds(5 as u32, move || {
    //         println!("World .. {i:?}");
    //         // Here we temporarily retrieve a strong reference on the pipeline from the weak one
    //         // we moved into this callback.
    //         let pipeline = match pipeline_weak.upgrade() {
    //             Some(pipeline) => pipeline,
    //             None => return glib::Continue(false),
    //         };
    //         println!("Sending custom event to the pipeline with send_eos={send_eos}");
    //         // let ev = ExampleCustomEvent::new(*send_eos);
    //         // if !pipeline.send_event(ev) {
    //         //     println!("Warning: Failed to send custom event");
    //         // }
    //         // Remove this handler, the pipeline will shutdown once our pad probe catches the custom
    //         // event and sends EOS
    //         glib::Continue(true)
    //     });
    // }


    // Operate GStreamer's bus, facilitating GLib's mainloop here.
    // This function call will block until you tell the mainloop to quit
    // (see above for how to do this).
    print!("calling main_loop.run");
    main_loop.run();
    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
    // if let Some(msg) =
    //     bus.timed_pop_filtered(None, &[gst::MessageType::Error, gst::MessageType::Eos])
    // {
    //     match msg.view() {
    //         gst::MessageView::Error(err) => {
    //             info!(
    //                 "Error received from element {:?}: {:?}",
    //                 msg.src().map(|s| s.path_string()),
    //                 err.error()
    //             );
    //         }
    //         gst::MessageView::Eos(_) => {
    //             info!("End of stream reached.");
    //         }
    //         _ => {
    //             // Ignore other messages
    //         }
    //     }
    // }

    // pipeline.set_state(gst::State::Null)?;

    Ok(())
}
