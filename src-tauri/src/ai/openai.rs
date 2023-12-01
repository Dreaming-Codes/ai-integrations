use std::io::{Cursor};
use async_openai::config::OpenAIConfig;
use async_openai::types::{ChatCompletionRequestMessage, ChatCompletionRequestMessageContentPart, ChatCompletionRequestMessageContentPartImageArgs, ChatCompletionRequestMessageContentPartTextArgs, ChatCompletionRequestUserMessageArgs, ChatCompletionRequestUserMessageContent, CreateChatCompletionRequestArgs, ImageUrl};
use image::DynamicImage;
use lazy_static::lazy_static;

lazy_static! {
    static ref OPENAI_CLIENT: async_openai::Client<OpenAIConfig> = async_openai::Client::new();
}

fn image_to_base64url(image: DynamicImage) -> String {
    let mut image_data = Cursor::new(Vec::new());
    image.write_to(&mut image_data, image::ImageOutputFormat::Jpeg(75)).expect("Unable to write image to buffer");
    let image_base64 = base64::encode(&image_data.get_ref());
    let image_url = format!("data:image/jpeg;base64,{}", image_base64);
    image_url
}

pub async fn process_image(image: DynamicImage) -> Result<String, async_openai::error::OpenAIError> {
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4-vision-preview")
        .messages(
            vec![
                ChatCompletionRequestMessage::User(
                    ChatCompletionRequestUserMessageArgs::default()
                        .content(ChatCompletionRequestUserMessageContent::Array(vec![
                            ChatCompletionRequestMessageContentPart::Image(
                                ChatCompletionRequestMessageContentPartImageArgs::default()
                                    .image_url(ImageUrl::from(image_to_base64url(image)))
                                    .build()?
                            ),
                            ChatCompletionRequestMessageContentPart::Text(
                                ChatCompletionRequestMessageContentPartTextArgs::default()
                                    .text("This is a screenshot of a quiz, reply with the correct answer, I do not need the explanation".to_string())
                                    .build()?
                            ),
                        ]))
                        .build()?
                )
            ]
        )
        .build()?;

    let response = OPENAI_CLIENT.chat().create(request).await?;
    let merged = response.choices.into_iter().map(|choice| choice.message.content);
    let merged = merged.filter_map(|content| content).collect::<Vec<_>>().join("");

    Ok(merged)
}
