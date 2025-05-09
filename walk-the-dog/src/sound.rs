use anyhow::{anyhow, Ok, Result};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::JsCast;
use web_sys::{AudioBuffer, AudioBufferSourceNode, AudioContext, AudioDestinationNode, AudioNode};
use js_sys::ArrayBuffer;


pub fn create_audio_context() -> Result<AudioContext> {
    AudioContext::new().map_err(|err| anyhow!("Could not create audio context {:#?}", err))
}

fn create_buffer_source(ctx: &AudioContext) -> Result<AudioBufferSourceNode> {
    ctx.create_buffer_source().map_err(|err| anyhow!("Error creating buffer source {:#?}", err))
}

fn create_track_source(ctx: &AudioContext, buffer: &AudioBuffer) -> Result<AudioBufferSourceNode> {
    let track_source = create_buffer_source(ctx)?;
    track_source.set_buffer(Some(buffer));
    connect_with_audio_node(&track_source, &ctx.destination())?;
    Ok(track_source)
}

fn connect_with_audio_node(
    buffer_source: &AudioBufferSourceNode,
    destination: &AudioDestinationNode,
) -> Result<AudioNode> {
    buffer_source
    .connect_with_audio_node(&destination)
    .map_err(|err| anyhow!("Error connecting audio source to destination {:#?}", err))
}

pub enum LOOPING {
    NO,
    YES,
}

pub fn play_sound(
    ctx: &AudioContext,
    buffer: &AudioBuffer,
    looping: LOOPING,
) -> Result<()> {
    let track_source = create_track_source(ctx, buffer)?;
    if matches!(looping, LOOPING::YES) {
        track_source.set_loop(true);
    }

    track_source.start().map_err(|err| anyhow!("Could not start sound! {:#?}", err))
}

pub async fn decode_audio_data(
    ctx: &AudioContext,
    audio_buffer: &ArrayBuffer,
) -> Result<AudioBuffer> {
    JsFuture::from(
        ctx.decode_audio_data(&audio_buffer)
        .map_err(|err| anyhow!("Could not decode audio from array buffer {:#?}", err))?,
    ).await
    .map_err(|err| anyhow!("Could not convert promise to future {:#?}", err))?
    .dyn_into()
    .map_err(|err| anyhow!("Could not cast into AudioBuffer {:#?}", err))
}