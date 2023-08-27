/*
Read a fila and shows it using autovideosink
gst-launch-1.0 filesrc location=video.mp4 ! qtdemux name=demux demux.video_0 ! \
    avdec_h264 ! videoconvert ! autovideosink

Created the video using this FFmpeg command
ffmpeg -f lavfi -re -i testsrc -r 25 -c:v libx264 video.mp4

*/
use anyhow::Error;
use gst::prelude::*;
use tracing::info;

use crate::common::make_element;

pub fn run() -> Result<(), Error> {
    let pipeline = gst::Pipeline::new(None);

    let src = make_element("filesrc", None)?;
    let demuxer = make_element("qtdemux", None)?;
    let decoder = make_element("avdec_h264", None)?;
    let videocvt = make_element("videoconvert", None)?;
    let sink = make_element("autovideosink", None)?;

    pipeline.add_many(&[&src, &demuxer, &decoder, &videocvt, &sink])?;

    gst::Element::link(&src, &demuxer)?;
    gst::Element::link_many(&[&decoder, &videocvt, &sink])?;

    src.set_property("location", "video.mp4");

    // demuxer.connect("pad-added", false, |values| {
    //     info!("in pad-added");
    //     None
    // });

    demuxer.connect_pad_added(move |demuxer, pad| {
        info!("in con pad added");
        info!("Hi from info!");

        let pad_type = pad
            .current_caps()
            .map(|caps| {
                caps.structure(0)
                    .map(|structure| structure.name())
                    .unwrap_or_else(|| "unknown")
            })
            .unwrap_or_else(|| "unknown");

        info!("Pad exposed: {}", pad_type);

        if pad_type.starts_with("audio/x-raw") {
        } else if pad_type.starts_with("video") {
            info!("Linking video pad...");
            let decoder_sinkpad = decoder
                .static_pad("sink")
                .expect("Decoder sink pad not found");
            pad.link(&decoder_sinkpad)
                .expect("Failed to link video pad and decoder pad");
        } else {
            info!("Unknown pad type: {}", pad_type);
        }
    });

    pipeline.set_state(gst::State::Playing)?;

    let bus = pipeline.bus().expect("Failed to get pipeline bus");

    if let Some(msg) =
        bus.timed_pop_filtered(None, &[gst::MessageType::Error, gst::MessageType::Eos])
    {
        match msg.view() {
            gst::MessageView::Error(err) => {
                info!(
                    "Error received from element {:?}: {:?}",
                    msg.src().map(|s| s.path_string()),
                    err.error()
                );
            }
            gst::MessageView::Eos(_) => {
                info!("End of stream reached.");
            }
            _ => {
                // Ignore other messages
            }
        }
    }

    pipeline.set_state(gst::State::Null)?;

    Ok(())
}
