use async_openai::config::OpenAIConfig;
use async_openai::types::{ChatCompletionRequestMessageContentPart, ChatCompletionRequestMessageContentPartImage, ChatCompletionRequestMessageContentPartText, ChatCompletionRequestUserMessage, ChatCompletionRequestUserMessageContent, CreateChatCompletionRequestArgs, ImageUrl};
use image::DynamicImage;
use lazy_static::lazy_static;
use serde::de::Unexpected::Option;

lazy_static! {
    static ref OPENAI_CLIENT: async_openai::Client<OpenAIConfig> = async_openai::Client::new();
}

fn image_to_base64url(image: DynamicImage) -> String {
    let image = image.to_rgba8();
    let image = image.into_vec();
    let image = base64::encode(image);
    let image = format!("data:image/png;base64,{}", image);
    image
}

#[derive(thiserror::Error, Debug)]
pub enum OpenAiError {
    #[error("OpenAI returned an error")]
    OpenAi(#[from] async_openai::error::OpenAIError),
    #[error("There was an error with the OpenAI API")]
    OpenAiApi(#[from] async_openai::error::ApiError),
}

pub async fn process_image(image: DynamicImage) -> Result<String, OpenAiError> {
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4-vision-preview")
        .messages(
            vec![
                ChatCompletionRequestUserMessage {
                    content: Some(ChatCompletionRequestUserMessageContent::Array(vec![
                        ChatCompletionRequestMessageContentPart::Image(ChatCompletionRequestMessageContentPartImage {
                            image_url: ImageUrl::from(image_to_base64url(image)),
                            ..Default::default()
                        }),
                        ChatCompletionRequestMessageContentPart::Text(ChatCompletionRequestMessageContentPartText {
                            text: "This is a screenshot of a quiz, reply with the correct answer, I do not need the explanation".to_string(),
                            ..Default::default()
                        }),
                    ])),
                    ..Default::default()
                }
            ]
        )
        .build()?;

    let response = OPENAI_CLIENT.chat().create(request).await?;
    let merged = response.choices.into_iter().map(|choice| choice.message.content).filter(Option::is_some).collect::<Vec<_>>().join("");

    Ok(merged)
}
