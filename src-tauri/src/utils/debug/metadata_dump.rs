use std::path::Path;

use anyhow::Result;
use lofty::{
    file::{AudioFile, TaggedFileExt},
    picture::Picture,
    probe::Probe,
    tag::{Accessor, ItemKey, ItemValue, Tag, TagExt},
};

/// This is more of debug function, to know
/// what everything have the metadata (feel free to ignore it).
/// This will be most likely removed later
pub fn dump_info<P: AsRef<Path>>(path: P) -> Result<()> {
    eprintln!("========== TRACK INFO ==========");
    eprintln!("path: {}", path.as_ref().display());
    let tagged_file = Probe::open(&path)?.read()?;
    eprintln!("file type: {:?}", tagged_file.file_type());
    eprintln!("tag count: {}", tagged_file.tags().len());
    for t in tagged_file.tags() {
        eprintln!("  contains tag type: {:?}", t.tag_type());
    }

    eprintln!("--- properties ---");

    let properties = tagged_file.properties();
    let duration = properties.duration();
    let channel_count = properties.channels();
    let samples = properties.sample_rate();
    let bit_depth = properties.bit_depth();
    let audio_bitrate = properties.audio_bitrate();
    let overall_bitrate = properties.overall_bitrate();

    eprintln!("duration: {:?} ({} ms)", duration, duration.as_millis());
    eprintln!("channels: {:?}", channel_count);
    eprintln!("sample rate: {:?}", samples);
    eprintln!("bit depth: {:?}", bit_depth);
    eprintln!("audio bitrate: {:?} kbps", audio_bitrate);
    eprintln!("overall bitrate: {:?} kbps", overall_bitrate);
    // Full struct in case there's anything extra (channel mask, etc.)
    eprintln!("properties (debug): {:#?}", properties);

    // Dump every tag in the file
    for tag in tagged_file.tags() {
        dump_tag(tag);
    }

    Ok(())
}

pub fn dump_tag(tag: &Tag) {
    eprintln!("--- tag: {:?} ---", tag.tag_type());
    eprintln!("item count: {}", tag.len());
    eprintln!("picture count: {}", tag.picture_count());

    // Common accessors
    eprintln!("title: {:?}", tag.title());
    eprintln!("artist: {:?}", tag.artist());
    eprintln!("album: {:?}", tag.album());
    eprintln!("genre: {:?}", tag.genre());
    eprintln!("comment: {:?}", tag.comment());
    eprintln!("track: {:?} / {:?}", tag.track(), tag.track_total());
    eprintln!("disk: {:?} / {:?}", tag.disk(), tag.disk_total());
    for r in tag.ratings() {
        eprintln!("rating: {:?}", r);
    }

    // Specific keys by name
    let keys = [
        ItemKey::AlbumArtist,
        ItemKey::Composer,
        ItemKey::Lyrics,
        ItemKey::Bpm,
        ItemKey::InitialKey,
        ItemKey::Isrc,
        ItemKey::Label,
        ItemKey::CopyrightUrl,
        ItemKey::EncoderSoftware,
        ItemKey::ReplayGainTrackGain,
        ItemKey::ReplayGainAlbumGain,
        ItemKey::MusicBrainzTrackId,
        ItemKey::Year,
        ItemKey::RecordingDate,
    ];
    for key in keys {
        let name = format!("{key:?}");
        eprintln!("{name}: {:?}", tag.get_string(key));
    }

    // Pictures
    for (i, pic) in tag.pictures().iter().enumerate() {
        dump_picture(i, pic);
    }

    // Every raw item in the tag (catches anything not listed above)
    eprintln!("--- raw items ---");
    for item in tag.items() {
        match item.value() {
            ItemValue::Text(s) | ItemValue::Locator(s) => {
                eprintln!("{:?} = {:?}", item.key(), s);
            }
            ItemValue::Binary(bytes) => {
                eprintln!("{:?} = <binary, {} bytes>", item.key(), bytes.len());
            }
        }
    }
}

pub fn dump_picture(i: usize, pic: &Picture) {
    eprintln!("picture #{i}:");
    eprintln!("  type: {:?}", pic.pic_type());
    eprintln!("  mime: {:?}", pic.mime_type());
    eprintln!("  description: {:?}", pic.description());
    // pic.data() is the raw image data.
    eprintln!("  size: {} bytes", pic.data().len());
}
