use mpris::PlayerFinder;
use std::time::{Duration};
use unicode_segmentation::UnicodeSegmentation;

/// Returns the complete display message
///
/// Message takes the form: "Track title — Artist[, artist[, artist[...]]]"
fn get_message(player: mpris::Player) -> String {
    match player.get_playback_status() {
        Ok(n) => match n {
            mpris::PlaybackStatus::Playing => {
                let metadata = player.get_metadata()
                                     .expect("Could not get player metadata");

                let mut artists = String::new();
                for (i, artist) in metadata.artists().unwrap().iter().enumerate() {
                    if i != 0 {
                        artists += ", "
                    }
                    artists += artist;
                }
                return format!(
                    "{} — {}",
                    metadata.title().unwrap_or("No Title"),
                    artists
                )
            },
            mpris::PlaybackStatus::Paused => return "⏸".to_string(),
            mpris::PlaybackStatus::Stopped => return "".to_string(),
        },
        Err(_) => return String::from("Could not get playback status")
    }
}

/// Return a display string scrolled to the appropriate location
///
/// This function takes a message and generates the appropriate display text
/// based on the absolute system time in milliseconds since the Epoch.
///
/// # Parameters
///
/// - `message`: A string containing the message to be rendered
/// - `display_width`: the target width of the display in Unicode Graphemes
/// - `song_position`: a `Duration` indicating elapsed playtime for current song
/// - `delay`: the delay, in clock ticks, to hold at the beginning of the message
/// - `tick_duration`: time, in milliseconds, per clock tick
///
/// # Returns
///
/// A `String` which contains the display text, scrolled and wrapped accordingly
fn marquee(
    mut message: String,
    display_width: usize,
    song_position: Duration,
    delay: u64,
    tick_duration: u128
) -> String {
    if message.len() <= display_width { return message; }
    let time = (song_position.as_millis() / tick_duration) as u64;
    message = format!("{} | ", message);
    let message: &str = message.as_str();
    let message_length = message.graphemes(true).count() as u64;
    let tick = time % (message_length + delay);

    let display: String = if tick < delay {
        message.graphemes(true).take(display_width).collect()
    } else if (tick - delay + display_width as u64) < message_length {
        message
            .graphemes(true)
            .skip((tick - delay) as usize)
            .take(display_width)
            .collect()
    } else {
        let wrapped_segment: String = message
            .graphemes(true)
            .take(((tick - delay + display_width as u64)
                   % message_length) as usize)
            .collect();
        let unwrapped_segment: String = message
            .graphemes(true)
            .skip((tick - delay) as usize)
            .collect();
        format!("{}{}", unwrapped_segment, wrapped_segment)
    };
    format!("{}⁢", display)
}

fn main() {
    // TODO: Accept config options from command line
    let marquee_width = 25;
    let marquee_delay = 3;
    let tick_length = 500;

    let mut song_pos = Duration::from_secs(0);
    let message = match PlayerFinder::new() {
        Ok(pf) => match pf.find_active() {
            Ok(p) =>  {
                song_pos = match p.get_position() {
                    Ok(pos) => pos,
                    Err(_) => return
                };
                get_message(p)
            },
            Err(_) => String::from("Could not find player")
        },
        Err(_) => String::from("Could not connect to DBus")
    };

    println!(
        "♫ {}",
        marquee(
            message,
            marquee_width,
            song_pos,
            marquee_delay,
            tick_length
        )
    );
}
