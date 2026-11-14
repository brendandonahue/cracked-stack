use rocket::serde::json::Json;
use rocket::post;
use rocket::form::Form;
use rocket::FromForm;
use serde::{Deserialize};
use crate::error::Error;
use crate::auth::{ 
    AuthUser,
};
use log::{info, error};
use serde_json::json;
use serde_json::Value;
use tempfile::tempdir;
use base64::engine::general_purpose;
use base64::Engine as _;  // brings .encode() into scope
use std::env;
use reqwest::Client;
use rocket::fs::{TempFile};
use rocket::tokio::fs::File;
use rocket::tokio::io::AsyncReadExt;

// Struct for form data
#[derive(FromForm)]
pub struct MediaUpload<'r> {
    prompt: String,
    file: TempFile<'r>,
}

// Response struct from Ollama
#[derive(Deserialize)]
struct OllamaResponse {
    message: OllamaMessage,
}

#[derive(Deserialize)]
struct OllamaMessage {
    content: String,
}

#[post("/generate", format = "json", data = "<payload>")]
pub async fn generate(user: AuthUser, payload: Json<Value>) -> Result<Json<Value>, Error> {
    if user.role != "business" { return Err(Error::Unauthorized); }
    let mut p = payload.into_inner();
    p["stream"] = Value::Bool(false);           // important!
    p["model"] = Value::String("qwen3:4b".into());

    let user_messages = match p.get("messages") {
        Some(Value::Array(arr)) => arr.clone(),
        _ => return Err(Error::BadRequest("No Messages".to_string()))
    };

    let mut messages = vec![json!({
        "role": "system",
        "content": "You are a social media manager and are tasked with creating the user's 
            social media manifesto for the day which should include an outline of exactly what they have to do to 
            create engaging content for their audience."
    })];

    messages.extend(user_messages);

    let client = Client::new();

    info!("Asking ollama");

    let ollama_chat_completions_url = env::var("OLLAMA_CHAT_COMPLETIONS_URL").unwrap_or_else(|_| "http://ollama:11434/v1/chat/completions".to_string()); //fallback for prod

    let res = client
        .post(&ollama_chat_completions_url)
        .json(&p)
        .send()
        .await
        .map_err(|_| Error::BadGateway)?;

    if !res.status().is_success() {
        let status = res.status();
        let text = res.text().await.unwrap_or_default();
        error!("Ollama error: {} - {}", status, text);
        return Err(Error::BadGateway);
    }

    info!("ollama responded successfully");

    let json: Value = res.json().await.map_err(|_| Error::Internal("Error parsing json".to_string()))?;

    info!("{}", json);

    Ok(Json(json))
}

// Image route: POST /image
#[post("/image", data = "<upload>")]
pub async fn image_route(user: AuthUser, mut upload: Form<MediaUpload<'_>>) -> Result<Json<Value>, Error> {
    if user.role != "business" { return Err(Error::Unauthorized); }
    let temp_dir = tempdir()?;
    let file_path = temp_dir.path().join(upload.file.name().unwrap_or("image"));
    upload.file.persist_to(&file_path).await?;

    // Read file to bytes
    let mut file = File::open(&file_path).await?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).await?;

    // Base64 encode
    let base64_image = general_purpose::STANDARD.encode(&bytes);

    // Call Ollama API
    let client = Client::new();
    let ollama_chat_url = env::var("OLLAMA_CHAT_URL").unwrap_or_else(|_| "http://ollama:11434/api/chat".to_string());
    let response = client
        .post(&ollama_chat_url)
        .json(&json!({
            "model": "llava",
            "messages": [{
                "role": "user",
                "content": upload.prompt,
                "images": [base64_image]
            }],
            "stream": false,
        }))
        .send()
        .await?;

    if response.status().is_success() {
        let ollama_res: OllamaResponse = response.json().await.map_err(|e| Error::Internal(e.to_string()))?;
        Ok(Json(json!({
            "content": ollama_res.message.content,
            "model": "llava",
            "success": true
        })))
    } else {
        Err(Error::BadRequest("Ollama error".to_string()))
    }
}

// Video route: POST /video
// #[post("/video", data = "<upload>")]
// pub async fn video_route(user: AuthUser, mut upload: Form<MediaUpload<'_>>) -> Result<Json<Value>, Error> {
//     if user.role != "business" { return Err(Error::Unauthorized); }
//     let temp_dir = tempdir()?;
//     let video_path = temp_dir.path().join(upload.file.name().unwrap_or("video.mp4"));
//     upload.file.persist_to(&video_path).await?;

//     let Ok((frame_paths, transcript)) = tokio::task::spawn_blocking(move || {
//         // Extract audio to WAV using FFmpeg (via command for simplicity; use ffmpeg-next for pure Rust)
//         let audio_path = temp_dir.path().join("audio.wav");

//         // Transcribe audio using whisper-rs
//         // Assume model path: let model_path = Path::new("/path/to/ggml-base.en.bin");
//         let model_path = Path::new("/models/whisper/ggml-base.en.bin");  // Replace with actual path
//         let params = WhisperContextParameters::default();

//         let ctx = WhisperContext::new_with_params(
//             model_path.to_str().unwrap(),
//             params,
//         )?;
//         let mut state = ctx.create_state()?;

//         // Read WAV
//         let mut reader = hound::WavReader::open(audio_path)?;
//         let audio: Vec<f32> = reader.samples::<f32>().map(|s| s.unwrap_or(0.0)).collect();

//         state.full(whisper_rs::FullParams::new(whisper_rs::SamplingStrategy::Greedy { best_of: 1 }), &audio[..])?;

//         let num_segments = state.full_n_segments() as usize;  // safe cast

//         let mut transcript = String::new();

//         for i in 0..num_segments {
//             match state.get_segment(i.try_into().unwrap()) {  // note: c_int = i32
//                 Some(segment) => {
//                     match segment.to_str() {
//                         Ok(text) => {
//                             transcript.push_str(text);
//                             transcript.push(' ');
//                         }
//                         Err(e) => {
//                             eprintln!("Segment {} encoding error: {}", i, e);
//                             transcript.push_str("[bad encoding] ");
//                         }
//                     }
//                 }
//                 None => continue  // empty segment, skip
//             }
//         }

//         let transcript = transcript.trim().to_string();
//         // Extract frames using ffmpeg-next
//         ffmpeg_next::init()?;
//         let mut input = ffmpeg_next::format::input(&video_path)?;
//         let input_stream = input
//             .streams()
//             .best(ffmpeg_next::media::Type::Video)
//             .ok_or(Error::Internal("No video stream found".to_string()))?;
//         let stream_index = input_stream.index();
//         let decoder_ctx = codec::context::Context::from_parameters(input_stream.parameters())?;
//         let _decoder_codec = decoder_ctx
//             .codec()
//             .ok_or(Error::Internal("No codec found for stream".to_string()))?;
//         let mut decoder = decoder_ctx
//             .decoder()
//             .video()
//             .map_err(|e| Error::Internal(format!("Failed to open video decoder: {}", e)))?;

//         let mut frame_count = 0;
//         let interval = 60; // every 60th frame (~2s at 30fps)
//         let mut frame_paths = Vec::new();

//         let _packet = ffmpeg_next::Packet::empty();
//         while input.packets().next().is_some() {  // or while input.read_packet(&mut packet).is_ok()
//             let (stream, pkt) = input.packets().next().unwrap();  // adjust if using older iterator style

//             if stream.index() == stream_index {
//                 decoder.send_packet(&pkt)?;

//                 let mut decoded = ffmpeg_next::frame::Video::empty();
//                 while decoder.receive_frame(&mut decoded).is_ok() {
//                     if frame_count % interval == 0 {
//                         let frame_path = temp_dir.path().join(format!("frame_{}.jpg", frame_count));

//                         // Create scaler from decoded frame format → RGB24
//                         let mut scaler = Context::get(
//                             decoded.format(),
//                             decoded.width(),
//                             decoded.height(),
//                             ffmpeg_next::format::Pixel::RGB24,
//                             decoded.width(),
//                             decoded.height(),
//                             Flags::BILINEAR,  // or Flags::BICUBIC for better quality
//                         )?;

//                         // Allocate output frame for RGB data
//                         let mut rgb_frame = Video::empty();
//                         rgb_frame.set_format(ffmpeg_next::format::Pixel::RGB24);
//                         rgb_frame.set_width(decoded.width());
//                         rgb_frame.set_height(decoded.height());

//                         // Allocate buffer manually, requires unsafe
//                         unsafe {
//                             rgb_frame.alloc(
//                                 ffmpeg_next::format::Pixel::RGB24,
//                                 decoded.width(),
//                                 decoded.height(),
//                             );
//                         }

//                         // Scale decoded → rgb_frame
//                         scaler.run(&decoded, &mut rgb_frame)?;

//                         // Now access rgb_frame data
//                         let width = rgb_frame.width() as u32;
//                         let height = rgb_frame.height() as u32;

//                         // RGB24 = 3 bytes per pixel (R,G,B packed)
//                         let _stride = rgb_frame.stride(0) as usize;  // usually width * 3, but check
//                         let data = rgb_frame.data(0);

//                         // Build ImageBuffer from raw bytes
//                         let img_buf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(
//                             width,
//                             height,
//                             data.to_vec(),  // copy the plane (safe for RGB24)
//                         ).ok_or(Error::Internal("Invalid RGB buffer".to_string()))?;

//                         let dyn_img = DynamicImage::ImageRgb8(img_buf);

//                         dyn_img.save(&frame_path)
//                             .map_err(|e| Error::Internal(format!("JPEG save failed: {}", e)))?;

//                         frame_paths.push(frame_path);

//                         if frame_paths.len() >= 5 { break; }
//                     }
//                     frame_count += 1;
//                 }
//             }
//         }
//         Ok((frame_paths, transcript))
//     })
//     .await?;

//     // Base64 encode frames
//     let mut base64_images = Vec::new();
//     for path in frame_paths {
//         let mut file = File::open(path).await?;
//         let mut bytes = Vec::new();
//         file.read_to_end(&mut bytes).await?;
//         base64_images.push(general_purpose::STANDARD.encode(&bytes));
//     }

//     // Combine prompt with transcript
//     let full_prompt = format!("{} \nTranscript: {}", upload.prompt, transcript);

//     // Call Ollama
//     let client = Client::new();
//     let response = client
//         .post("http://localhost:11434/api/chat")
//         .json(&json!({
//             "model": "llava",
//             "messages": [{
//                 "role": "user",
//                 "content": full_prompt,
//                 "images": base64_images
//             }],
//             "stream": false
//         }))
//         .send()
//         .await?;

//     if response.status().is_success() {
//         let ollama_res: OllamaResponse = response.json().await?;
//         Ok(Json(json!({
//             "content": ollama_res.message.content,
//             "model": "llava",
//             "success": true
//         })))
//     } else {
//         Err(Error::BadRequest("Ollama error".to_string()))
//     }
// }