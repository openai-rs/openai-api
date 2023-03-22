// Given a prompt and/or an input image, the model will generate a new image.
// See: https://platform.openai.com/docs/api-reference/images

//! Images API

use super::{IMAGES_CREATE, IMAGES_EDIT, IMAGES_VARIATIONS};
use crate::{
    openai::OpenAI,
    requests::{ApiResult, Json, Requests},
};
use multipart::client::lazy::Multipart;
use serde::{Deserialize, Serialize};
use std::{fs::File, str};
use crate::requests::Error::ApiError;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImagesBody {
    /// A text description of the desired image(s). The maximum length is 1000 characters.
    pub prompt: Option<String>,
    /// The number of images to generate. Must be between 1 and 10.
    /// Defaults to 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    /// The size of the generated images. Must be one of 256x256, 512x512, or 1024x1024.
    /// Defaults to 1024x1024
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// The format in which the generated images are returned. Must be one of url or b64_json.
    /// Defaults to url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug)]
pub struct ImagesEditBody {
    /// The image to edit. Must be a valid PNG file, less than 4MB, and square.
    /// If mask is not provided, image must have transparency, which will be used as the mask.
    pub image: File,
    /// An additional image whose fully transparent areas (e.g. where alpha is zero)
    /// indicate where image should be edited.
    /// Must be a valid PNG file, less than 4MB, and have the same dimensions as image.
    pub mask: Option<File>,
    pub images_body: ImagesBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    created: u64,
    data: Option<Vec<ImageData>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageData {
    url: String,
}

pub trait ImagesApi {
    /// Given a prompt and/or an input image, the model will generate a new image.
    fn image_create(&self, images_body: &ImagesBody) -> ApiResult<Image>;
    /// Creates an edited or extended image given an original image and a prompt.
    fn image_edit(&self, images_edit_body: ImagesEditBody) -> ApiResult<Image>;
    /// Creates a variation of a given image.
    fn image_variation(&self, images_edit_body: ImagesEditBody) -> ApiResult<Image>;
}

impl ImagesApi for OpenAI {
    fn image_create(&self, images_body: &ImagesBody) -> ApiResult<Image> {
        if images_body.prompt.is_none() {
            return Err(ApiError("Prompt is required.".to_string()))
        }
        let request_body = serde_json::to_value(images_body).unwrap();
        let result = self.post(IMAGES_CREATE, request_body);
        let res: Json = result.unwrap();
        let image: Image = serde_json::from_value(res.clone()).unwrap();
        Ok(image)
    }

    fn image_edit(&self, images_edit_body: ImagesEditBody) -> ApiResult<Image> {
        if images_edit_body.images_body.prompt.is_none() {
            return Err(ApiError("Prompt is required.".to_string()))
        }

        let mut send_data = Multipart::new();

        send_data.add_text("prompt", images_edit_body.images_body.prompt.unwrap());
        if let Some(n) = images_edit_body.images_body.n {
            send_data.add_text("n", n.to_string());
        }
        if let Some(size) = images_edit_body.images_body.size {
            send_data.add_text("size", size.to_string());
        }
        if let Some(response_format) = images_edit_body.images_body.response_format {
            send_data.add_text("response_format", response_format.to_string());
        }
        if let Some(user) = images_edit_body.images_body.user {
            send_data.add_text("user", user.to_string());
        }
        if let Some(mask) = images_edit_body.mask {
            send_data.add_stream("mask", mask, Some("blob"), Some(mime::IMAGE_PNG));
        }
        send_data.add_stream(
            "image",
            images_edit_body.image,
            Some("blob"),
            Some(mime::IMAGE_PNG),
        );

        let result = self.post_multipart(IMAGES_EDIT, send_data);
        let res: Json = result.unwrap();
        let image: Image = serde_json::from_value(res.clone()).unwrap();
        Ok(image)
    }

    fn image_variation(&self, images_edit_body: ImagesEditBody) -> ApiResult<Image> {
        let mut send_data = Multipart::new();

        if let Some(n) = images_edit_body.images_body.n {
            send_data.add_text("n", n.to_string());
        }
        if let Some(size) = images_edit_body.images_body.size {
            send_data.add_text("size", size.to_string());
        }
        if let Some(response_format) = images_edit_body.images_body.response_format {
            send_data.add_text("response_format", response_format.to_string());
        }
        if let Some(user) = images_edit_body.images_body.user {
            send_data.add_text("user", user.to_string());
        }
        send_data.add_stream(
            "image",
            images_edit_body.image,
            Some("blob"),
            Some(mime::IMAGE_PNG),
        );

        let result = self.post_multipart(IMAGES_VARIATIONS, send_data);
        let res: Json = result.unwrap();
        let image: Image = serde_json::from_value(res.clone()).unwrap();
        Ok(image)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::{
        apis::images::{ImagesApi, ImagesBody, ImagesEditBody},
        openai::new_test_openai,
    };

    #[test]
    fn test_image_create() {
        let openai = new_test_openai();
        let body = ImagesBody {
            prompt: Some("A cute baby sea otter".to_string()),
            n: Some(2),
            size: Some("1024x1024".to_string()),
            response_format: None,
            user: None,
        };
        let rs = openai.image_create(&body);
        let images = rs.unwrap().data.unwrap();
        let image = images.get(0).unwrap();
        assert_eq!(image.url.contains("http"), true);
    }

    #[test]
    fn test_image_edit() {
        let openai = new_test_openai();
        let file = File::open("test_files/test.png").unwrap();
        let multipart = ImagesEditBody {
            images_body: ImagesBody {
                prompt: Some("A cute baby sea otter wearing a beret".to_string()),
                n: Some(2),
                size: Some("1024x1024".to_string()),
                response_format: None,
                user: None,
            },
            image: file,
            mask: None,
        };
        let rs = openai.image_edit(multipart);
        let images = rs.unwrap().data.unwrap();
        let image = images.get(0).unwrap();
        assert_eq!(image.url.contains("http"), true);
    }

    #[test]
    fn test_image_variations() {
        let openai = new_test_openai();
        let file = File::open("test_files/test.png").unwrap();
        let multipart = ImagesEditBody {
            images_body: ImagesBody {
                prompt: None,
                n: Some(2),
                size: Some("1024x1024".to_string()),
                response_format: None,
                user: None,
            },
            image: file,
            mask: None,
        };
        let rs = openai.image_variation(multipart);
        let images = rs.unwrap().data.unwrap();
        let image = images.get(0).unwrap();
        assert_eq!(image.url.contains("http"), true);
    }
}
