use raylib::prelude::*;
use std::collections::HashMap;
use raylib::ffi::UnloadTexture;

pub struct TextureManager {
    textures: HashMap<String, Texture2D>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn load_texture(
        &mut self,
        texture_path: &str,
        texture_name: &str,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) -> Result<&Texture2D, Box<dyn std::error::Error>> {
        if self.textures.contains_key(texture_name) {
            return Ok(self.textures.get(texture_name).unwrap());
        }
        let texture2d = rl.load_texture(thread, texture_path)?;
        self.textures.insert(texture_name.to_string(), texture2d);
        Ok(self.textures.get(texture_name).unwrap())
    }

    pub fn texture(&self, texture_name: &str) -> Option<&Texture2D> {
        self.textures.get(texture_name)
    }
}

impl Drop for TextureManager {
    fn drop(&mut self) {
        for (_, texture) in self.textures.drain() {
            unsafe { UnloadTexture(texture.unwrap()) };
        }
    }
}

pub struct Assets {
    texture_manager: TextureManager
}

impl Assets {
    pub fn new() -> Self {
        Self { texture_manager: TextureManager::new() }
    }

    pub fn texture_manager(&self) -> &TextureManager {
        &self.texture_manager
    }

    pub fn texture_manager_mut(&mut self) -> &mut TextureManager {
        &mut self.texture_manager
    }
}
