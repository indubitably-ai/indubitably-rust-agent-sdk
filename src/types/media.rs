//! Media-related type definitions for the SDK.
//! 
//! This module defines the types used to represent different media types
//! including documents, images, and videos.

use serde::{Deserialize, Serialize};

/// Document content to include in a message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentContent {
    /// The type of document.
    #[serde(rename = "type")]
    pub content_type: DocumentType,
    /// The source of the document.
    pub source: DocumentSource,
}

/// The type of document content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DocumentType {
    Text,
    Pdf,
    Word,
    Excel,
    Powerpoint,
    Markdown,
    Html,
    Csv,
    Json,
    Xml,
}

/// The source of a document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentSource {
    /// The type of source.
    #[serde(rename = "type")]
    pub source_type: DocumentSourceType,
    /// The media type of the document.
    #[serde(rename = "mediaType")]
    pub media_type: String,
    /// The data of the document.
    pub data: DocumentData,
}

/// The type of document source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DocumentSourceType {
    Base64,
    S3,
    Http,
    File,
}

/// The data of a document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentData {
    /// The text content of the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// The base64 encoded content of the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base64: Option<String>,
    /// The URL of the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The file path of the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}

/// Image content to include in a message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageContent {
    /// The type of image.
    #[serde(rename = "type")]
    pub content_type: ImageType,
    /// The source of the image.
    pub source: ImageSource,
}

/// The type of image content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageType {
    Image,
    Screenshot,
    Photo,
    Diagram,
    Chart,
    Graph,
}

/// The source of an image.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageSource {
    /// The type of source.
    #[serde(rename = "type")]
    pub source_type: ImageSourceType,
    /// The media type of the image.
    #[serde(rename = "mediaType")]
    pub media_type: String,
    /// The data of the image.
    pub data: ImageData,
}

/// The type of image source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageSourceType {
    Base64,
    S3,
    Http,
    File,
}

/// The data of an image.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageData {
    /// The base64 encoded content of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base64: Option<String>,
    /// The URL of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The file path of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}

/// Video content to include in a message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoContent {
    /// The type of video.
    #[serde(rename = "type")]
    pub content_type: VideoType,
    /// The source of the video.
    pub source: VideoSource,
}

/// The type of video content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoType {
    Video,
    Animation,
    ScreenRecording,
    Movie,
    Clip,
}

/// The source of a video.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoSource {
    /// The type of source.
    #[serde(rename = "type")]
    pub source_type: VideoSourceType,
    /// The media type of the video.
    #[serde(rename = "mediaType")]
    pub media_type: String,
    /// The data of the video.
    pub data: VideoData,
}

/// The type of video source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoSourceType {
    Base64,
    S3,
    Http,
    File,
}

/// The data of a video.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoData {
    /// The base64 encoded content of the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base64: Option<String>,
    /// The URL of the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The file path of the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}

impl DocumentContent {
    /// Create a new text document.
    pub fn text(text: &str) -> Self {
        Self {
            content_type: DocumentType::Text,
            source: DocumentSource {
                source_type: DocumentSourceType::Base64,
                media_type: "text/plain".to_string(),
                data: DocumentData {
                    text: Some(text.to_string()),
                    base64: None,
                    url: None,
                    file_path: None,
                },
            },
        }
    }

    /// Create a new PDF document from base64 data.
    pub fn pdf_base64(base64: &str) -> Self {
        Self {
            content_type: DocumentType::Pdf,
            source: DocumentSource {
                source_type: DocumentSourceType::Base64,
                media_type: "application/pdf".to_string(),
                data: DocumentData {
                    text: None,
                    base64: Some(base64.to_string()),
                    url: None,
                    file_path: None,
                },
            },
        }
    }
}

impl ImageContent {
    /// Create a new image from base64 data.
    pub fn base64(base64: &str, media_type: &str) -> Self {
        Self {
            content_type: ImageType::Image,
            source: ImageSource {
                source_type: ImageSourceType::Base64,
                media_type: media_type.to_string(),
                data: ImageData {
                    base64: Some(base64.to_string()),
                    url: None,
                    file_path: None,
                },
            },
        }
    }

    /// Create a new image from a URL.
    pub fn url(url: &str, media_type: &str) -> Self {
        Self {
            content_type: ImageType::Image,
            source: ImageSource {
                source_type: ImageSourceType::Http,
                media_type: media_type.to_string(),
                data: ImageData {
                    base64: None,
                    url: Some(url.to_string()),
                    file_path: None,
                },
            },
        }
    }
}

impl VideoContent {
    /// Create a new video from base64 data.
    pub fn base64(base64: &str, media_type: &str) -> Self {
        Self {
            content_type: VideoType::Video,
            source: VideoSource {
                source_type: VideoSourceType::Base64,
                media_type: media_type.to_string(),
                data: VideoData {
                    base64: Some(base64.to_string()),
                    url: None,
                    file_path: None,
                },
            },
        }
    }

    /// Create a new video from a URL.
    pub fn url(url: &str, media_type: &str) -> Self {
        Self {
            content_type: VideoType::Video,
            source: VideoSource {
                source_type: VideoSourceType::Http,
                media_type: media_type.to_string(),
                data: VideoData {
                    base64: None,
                    url: Some(url.to_string()),
                    file_path: None,
                },
            },
        }
    }
}
