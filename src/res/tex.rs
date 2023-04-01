use std::sync::{Mutex, Arc};

use ggez::graphics::{Image, Drawable, Rect};

use super::mgr::Loader;


struct TexLoader {
    ctx: Arc<Mutex<ggez::Context>>,
}

unsafe impl Send for TexLoader { }

unsafe impl Sync for TexLoader { }

impl TexLoader {
    pub fn new(ctx: Arc<Mutex<ggez::Context>>) -> Self {
        Self { ctx }
    }
}

impl Loader for TexLoader {
    type Resource = Image;

    fn load(&self, path: &str) -> Result<Self::Resource, String> {
        let ctx = self.ctx.lock().unwrap();
        let image = Image::from_path(&*ctx, path).map_err(|e| format!("Failed to load texture: {:?}", e))?;
        Ok(image)
    }
}