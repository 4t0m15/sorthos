use eframe::egui;
use std::path::Path;

pub struct GifHandler {
    pub frames: Vec<egui::TextureHandle>,
    pub current_time: f32,
    pub frame_duration: f32,
}

impl Default for GifHandler {
    fn default() -> Self {
        Self {
            frames: Vec::new(),
            current_time: 0.0,
            frame_duration: 1.0 / 12.0, // 12 FPS
        }
    }
}

impl GifHandler {

    pub fn load_gif_from_bytes(
        &mut self,
        ctx: &egui::Context,
        gif_bytes: &[u8],
        name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Clear existing frames
        self.frames.clear();

        use std::io::Cursor;
        use image::codecs::gif::GifDecoder;
        use image::AnimationDecoder;

        // Decode GIF and collect frames
        let reader = GifDecoder::new(Cursor::new(gif_bytes))?;
        let frames = reader.into_frames().collect_frames()?;

        // Upload each frame as a texture
        for (frame_index, frame) in frames.into_iter().enumerate() {
            // Convert frame into an ImageBuffer to get dimensions and raw data
            let buffer = frame.into_buffer(); // RGBA ImageBuffer<u8>
            let (w_u32, h_u32) = buffer.dimensions();
            let w = w_u32 as usize;
            let h = h_u32 as usize;
            let raw = buffer.into_raw(); // Vec<u8>
            let pixels = raw.chunks_exact(4)
                .map(|ch| egui::Color32::from_rgba_unmultiplied(ch[0], ch[1], ch[2], ch[3]))
                .collect::<Vec<_>>();

            let image = egui::ColorImage { size: [w, h], pixels };
            let frame_name = format!("{}_frame{}", name, frame_index);
            let texture = ctx.load_texture(frame_name, image, egui::TextureOptions::default());
            self.frames.push(texture);
        }

        Ok(())
    }

    pub fn load_gif_from_file(
        &mut self,
        ctx: &egui::Context,
        path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Try to read the file
        match std::fs::read(path) {
            Ok(bytes) => {
                let name = path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");
                self.load_gif_from_bytes(ctx, &bytes, name)
            }
            Err(e) => {
                // If file doesn't exist, return error
                Err(Box::new(e))
            }
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.frames.is_empty() {
            self.current_time += delta_time;
        }
    }

    pub fn get_current_frame(&self) -> Option<&egui::TextureHandle> {
        if self.frames.is_empty() {
            return None;
        }
        
        let frame_index = ((self.current_time / self.frame_duration) as usize) % self.frames.len();
        self.frames.get(frame_index)
    }

    pub fn render(&mut self, ui: &mut egui::Ui, size: [f32; 2]) {
        self.update(ui.input(|i| i.unstable_dt));
        
        if let Some(texture) = self.get_current_frame() {
            ui.image((texture.id(), egui::Vec2::from(size)));
        } else {
            // Show a placeholder when no GIF is loaded
            ui.vertical_centered(|ui| {
                ui.label("🦆");
                ui.small("Duck GIF not found");
            });
        }
    }
}
