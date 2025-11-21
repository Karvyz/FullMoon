use anyhow::{Result, anyhow};
use iced::{
    advanced::{
        graphics::image::image_rs::{ImageBuffer, Rgba, imageops::crop_imm, open},
        image::Bytes,
    },
    widget::image::Handle,
};
use log::{error, trace};
use std::{fs, path::PathBuf, rc::Rc, time::SystemTime};

use crate::persona::{CharData, Persona, basic::Basic, card::Card};

pub enum Subdir {
    Chars,
    Users,
}

impl Subdir {
    fn default_persona(&self) -> Persona {
        match self {
            Subdir::Chars => Persona::default_char(),
            Subdir::Users => Persona::default_user(),
        }
    }

    fn default_handle(&self) -> Handle {
        match self {
            Subdir::Chars => Handle::from_path("assets/char.png"),
            Subdir::Users => Handle::from_path("assets/user.png"),
        }
    }
}

pub struct PersonaLoader {}

impl PersonaLoader {
    pub fn load_from_cache(subdir: Subdir) -> Vec<Persona> {
        let dir = Self::cache_path(&subdir);
        match Self::try_load_dir(dir, &subdir.default_handle()) {
            Ok(personas) => personas,
            Err(e) => {
                error!("{e}");
                vec![subdir.default_persona()]
            }
        }
    }

    pub fn load_most_recent_from_cache(subdir: Subdir) -> Persona {
        let cache_path = Self::cache_path(&subdir);
        match Self::most_recent_dir(&cache_path) {
            Ok(most_recent) => match Self::try_load_subdir(most_recent, &subdir.default_handle()) {
                Ok(persona) => return persona,
                Err(e) => error!("{e}"),
            },

            Err(e) => error!("{e}"),
        }
        subdir.default_persona()
    }

    fn most_recent_dir(path: &PathBuf) -> Result<PathBuf> {
        let mut most_recent_dir: Result<PathBuf> = Err(anyhow!("No file found"));
        let mut most_recent_change = SystemTime::UNIX_EPOCH;
        for entry in (fs::read_dir(path)?).flatten() {
            let path = entry.path();

            if path.is_dir() {
                let modified_time = Self::modified_time(&path);
                if modified_time > most_recent_change {
                    most_recent_change = modified_time;
                    most_recent_dir = Ok(path)
                }
            }
        }
        most_recent_dir
    }

    fn try_load_dir(dir: PathBuf, default_handle: &Handle) -> Result<Vec<Persona>> {
        let mut personas = vec![];
        for entry in (fs::read_dir(dir)?).flatten() {
            let path = entry.path();
            if path.is_dir()
                && let Ok(persona) = Self::try_load_subdir(path, default_handle)
            {
                personas.push(persona);
            }
        }
        Ok(personas)
    }

    fn try_load_subdir(dir: PathBuf, default_handle: &Handle) -> Result<Persona> {
        let mut image = Err(anyhow!("Persona not found"));
        let mut persona = Err(anyhow!("Persona not found"));
        for entry in (fs::read_dir(dir)?).flatten() {
            let path = entry.path();
            if path.is_file()
                && let Some(ext) = path.extension()
                && let Some(ext) = ext.to_str()
            {
                match ext {
                    "json" => persona = Self::load_persona(path),
                    "png" => image = Self::load_image(path),
                    _ => (),
                }
            }
        }

        match persona {
            Ok(data) => Ok(Persona::new(
                data,
                match image {
                    Ok(image) => image,
                    Err(_) => default_handle.clone(),
                },
            )),
            Err(_) => Err(anyhow!("Persona not found")),
        }
    }

    fn load_persona(path: PathBuf) -> Result<Rc<dyn CharData>> {
        let data = fs::read_to_string(&path)?;
        if let Ok(card) = Card::load_from_json(&data) {
            trace!("Loaded card {}", card.name());
            return Ok(card);
        }

        let basic = Basic::load_from_json(&data)?;
        trace!("Loaded simple {}", basic.name());
        Ok(basic)
    }

    fn load_image(path: PathBuf) -> Result<Handle> {
        let mut image = Self::crop_to_square(open(path)?.to_rgba8());

        let (width, height) = image.dimensions();
        let center_x = width as f64 / 2.0;
        let center_y = height as f64 / 2.0;
        let radius = width.min(height) as f64 / 2.0;

        // Process each pixel
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let distance_from_center =
                ((x as f64 - center_x).powi(2) + (y as f64 - center_y).powi(2)).sqrt();

            if distance_from_center > radius {
                pixel[3] = 0
            }
        }

        Ok(Handle::from_rgba(
            width,
            height,
            Bytes::from(image.into_raw()),
        ))
    }

    fn crop_to_square(image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let (width, height) = image.dimensions();
        let size = width.min(height);

        let x_offset = (width - size) / 2;
        let y_offset = (height - size) / 2;

        crop_imm(&image, x_offset, y_offset, size, size).to_image()
    }

    fn cache_path(subdir: &Subdir) -> PathBuf {
        dirs::cache_dir()
            .map(|mut path| {
                path.push("fullmoon");
                path.push(match subdir {
                    Subdir::Chars => "chars",
                    Subdir::Users => "users",
                });
                path
            })
            .unwrap()
    }

    fn modified_time(path: &PathBuf) -> SystemTime {
        if let Ok(metadata) = fs::metadata(path)
            && let Ok(modified_time) = metadata.modified()
        {
            return modified_time;
        }
        SystemTime::UNIX_EPOCH
    }
}
