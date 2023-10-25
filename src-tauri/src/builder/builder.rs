use std::borrow::Cow;
use std::sync::{LockResult, Mutex};
use anyhow::{Result as AnyResult, Error as AnyError, anyhow};
use arboard::ImageData;
use image::DynamicImage;
use once_cell::sync::OnceCell;

struct Image(DynamicImage);
struct Text(String);

// Using arboard instead of tauri clipboard manager since it also handle images
fn clipboard_handle() -> Result<&'static Mutex<arboard::Clipboard>, arboard::Error> {
    static INSTANCE: OnceCell<Mutex<arboard::Clipboard>> = OnceCell::new();
    INSTANCE.get_or_try_init(|| {
        Ok(Mutex::new(arboard::Clipboard::new()?))
    })
}

impl CopyToClipboard for Image {
    fn copy_to_clipboard(&self) -> AnyResult<()> {
        let image = ImageData {
            width: self.0.width() as usize,
            height: self.0.height() as usize,
            bytes: Cow::from(self.0.as_bytes()),
        };
        let clipboard_handle = clipboard_handle()?;
        let mut clipboard_handle = clipboard_handle.lock().map_err(|e| anyhow!("Failed to lock clipboard handle"))?;

        clipboard_handle.set_image(image)?;

        Ok(())
    }
}

impl CopyToClipboard for Text {
    fn copy_to_clipboard(&self) -> AnyResult<()> {
        let clipboard_handle = clipboard_handle()?;
        let mut clipboard_handle = clipboard_handle.lock().map_err(|e| anyhow!("Failed to lock clipboard handle"))?;

        clipboard_handle.set_text(self.0.clone())?;

        Ok(())
    }
}

impl WriteToKeyboard for Image {
    fn write_to_keyboard(&self) -> AnyResult<()> {
        todo!()
    }
}

enum DataType {
    Image(DynamicImage),
    Text(String)
}

enum Target {
    Clipboard(Box<dyn CopyToClipboard>),
    VirtualKeyboard(Box<dyn WriteToKeyboard>)
}

trait CopyToClipboard {
    fn copy_to_clipboard(&self) -> AnyResult<()>;
}

trait WriteToKeyboard {
    fn write_to_keyboard(&self) -> AnyResult<()>;
}
