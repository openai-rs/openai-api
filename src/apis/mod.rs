use serde::{Deserialize, Serialize};

mod chat;
mod completions;
mod edits;
mod images;
mod models;
mod embeddings;
mod audio;

// Models API
const MODELS_LIST: &str = "models";
const MODELS_RETRIEVE: &str = "models/";
// Completions API
const COMPLETION_CREATE: &str = "completions";
// Chat API
const CHAT_COMPLETION_CREATE: &str = "chat/completions";
// Edits API
const EDIT_CREATE: &str = "edits";
// Images API
const IMAGES_CREATE: &str = "images/generations";
const IMAGES_EDIT: &str = "images/edits";
const IMAGES_VARIATIONS: &str = "images/variations";
// Embeddings API
const EMBEDDINGS_CREATE: &str = "embeddings";
// Audio API
const AUDIO_TRANSCRIPTION_CREATE: &str = "audio/transcriptions";
const AUDIO_TRANSLATIONS_CREATE: &str = "audio/translations";


#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
}