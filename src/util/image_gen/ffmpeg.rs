use crate::cmd_util::TrancerError;
use crate::util::config::CONFIG;
use std::fs;
use std::fs::exists;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

fn get_font_file() -> Result<&'static str, TrancerError> {
    let fonts = [
        "/usr/share/fonts/TTF/Impact.TTF",
        "/usr/share/fonts/msttcore/impact.ttf",
    ];

    fonts
        .iter()
        .find(|x| Path::new(x).exists())
        .copied()
        .ok_or_else(|| TrancerError::Generic("Could not find a valid font file".to_string()))
}

fn run_ffmpeg(command: Vec<String>, output_name: &PathBuf) -> Result<Vec<u8>, TrancerError> {
    let mut output_path = PathBuf::from(&CONFIG.general.data_dir);
    output_path.push("output");

    if !output_path.exists() {
        fs::create_dir(&output_path)?;
    }

    output_path.push(&output_name);

    let status = Command::new("ffmpeg")
        .args(command)
        .arg(output_path.to_string_lossy().to_string())
        .arg("-y")
        .status();

    match status {
        Ok(ok) if !ok.success() => {
            return Err(TrancerError::Generic(
                "Ffmpeg failed to run, gave bad status code".to_string(),
            ))
        }
        Err(err) => {
            return Err(TrancerError::Generic(format!(
                "Ffmpeg failed to run: {}",
                err.to_string()
            )))
        }
        _ => (),
    };

    fs::read(output_path).map_err(|x| TrancerError::from(x))
}

pub fn add_caption_to_gif(
    input_path: &str,
    caption: &str,
    ext: &str,
) -> Result<(Vec<u8>, String), TrancerError> {
    let safe_caption = caption.replace("'", "\\'");
    let lines = split_every_n(&safe_caption, 30);
    let whitespace = 50 * lines.len();

    let font = get_font_file()?;

    let filter = format!(
        "pad=iw:ih+{whitespace}:0:{whitespace}:white,\
drawtext=text='{text}':fontfile='{font}':x=(w-text_w)/2:y=10:fontsize=36:fontcolor=black",
        whitespace = whitespace,
        text = lines.join("\\n"),
        font = font
    );

    let output_name = PathBuf::from(format!("output.{}", ext));

    let args = vec!["-i".into(), input_path.into(), "-vf".into(), filter];

    run_ffmpeg(args, &output_name).map(|x| (x, output_name.to_string_lossy().to_string()))
}

fn split_every_n(s: &str, n: usize) -> Vec<String> {
    s.chars()
        .collect::<Vec<_>>()
        .chunks(n)
        .map(|chunk| chunk.iter().collect())
        .collect()
}
