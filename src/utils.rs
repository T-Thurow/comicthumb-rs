use alphanumeric_sort::sort_str_slice;
use image::DynamicImage;
use regex::Regex;

pub fn guess_cover(files: &Vec<String>) -> Option<String> {
    let sorted = alphanumeric_sort(files);
    let ext_re = Regex::new(r"\.(jpg|jpeg|png|gif|tif|tiff|webp)\s*$").unwrap();
    let front_re = Regex::new(r"(cover|front)").unwrap();

    let images = sorted
        .iter()
        .filter(|&f| ext_re.is_match(f))
        .collect::<Vec<_>>();

    let candidate = images
        .iter()
        .find(|&f| front_re.is_match(f) && !f.to_lowercase().contains("back"))
        .map(|f| f.to_string());

    if candidate.is_some() {
        return candidate;
    }

    if let Some(img) = images.first() {
        return Some(img.to_string());
    }

    None
}

pub fn alphanumeric_sort(filenames: &Vec<String>) -> Vec<String> {
    let mut files = filenames.clone();
    sort_str_slice(&mut files);
    files
}

pub fn generate_tumbnail(im: &DynamicImage, size: u32) -> DynamicImage {
    let (x, y) = if im.width() > im.height() {
        (size, size * im.height() / im.width())
    } else {
        (size * im.width() / im.height(), size)
    };

    im.thumbnail(x, y)
}
