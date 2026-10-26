use crate::{crawler::collecting::CollectedTracks, structs::audio::track::Track};
use anyhow::Result;
use lofty::{
    file::{AudioFile, TaggedFileExt},
    probe::Probe,
    tag::Accessor,
};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};
use xxhash_rust::xxh3::Xxh3;

pub fn build_track(c: &CollectedTracks, known: &HashMap<PathBuf, Track>) -> Result<Track> {
    let old = known.get(&c.path);
    let unchanged = old.filter(|o| o.mtime == c.mtime);

    Ok(if let Some(un_track) = unchanged {
        un_track.clone()
    } else {
        let tagged = Probe::open(&c.path)?.read()?;
        let duration = tagged.properties().duration().as_secs();
        let tag = tagged.primary_tag().or_else(|| tagged.first_tag());

        let title = tag
            .and_then(|t| t.title())
            .map(|t| fix_legacy_encoding(t.into_owned()))
            .or_else(|| c.path.file_stem().map(|s| s.to_string_lossy().into_owned()))
            .unwrap_or_default();
        let comment = tag
            .and_then(|t| t.comment())
            .map(|t| fix_legacy_encoding(t.into_owned()))
            .unwrap_or_default();

        let fp = fingerprint(&c.path, c.size)?;
        let hash = None;
        let (id, rating, is_favorite) = (0, 0, false);
        Track {
            id,
            path: c.path.clone(),
            mtime: c.mtime,
            fingerprint: fp,
            hash,
            title,
            rating,
            comment,
            duration,
            is_favorite,
        }
    })
}

/// This will make cheap check of Head Body and Tail.
/// That means you have bigger chance of duplicate fingerprints
fn fingerprint(path: &Path, size: u64) -> Result<u64> {
    const CHUNK: usize = 64 * 1024;
    let mut file = File::open(path)?;
    let mut hasher = Xxh3::new();
    let mut buf = vec![0u8; CHUNK];
    hasher.update(&size.to_le_bytes());

    let mut sample = |f: &mut File, pos: u64, h: &mut Xxh3| -> Result<()> {
        f.seek(SeekFrom::Start(pos))?;
        let mut filled = 0;
        while filled < CHUNK {
            let n = f.read(&mut buf[filled..])?;
            if n == 0 {
                break;
            }
            filled += n;
        }
        h.update(&buf[..filled]);
        Ok(())
    };

    sample(&mut file, 0, &mut hasher)?;
    if size > (CHUNK * 3) as u64 {
        sample(&mut file, size / 2, &mut hasher)?;
        sample(&mut file, size - CHUNK as u64, &mut hasher)?;
    }
    Ok(hasher.digest())
}

/// Makes hash from file content. This is much more heavy than the fingerprint,
/// but you have much smaller chance of getting same identifier.
pub fn hash<P: AsRef<Path>>(path: P) -> Result<u64> {
    const CHUNK: usize = 64 * 1024;
    let mut file = File::open(path)?;
    let mut hasher = Xxh3::new();
    let mut buf = vec![0u8; CHUNK];

    loop {
        let bytes_read = file.read(&mut buf)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buf[..bytes_read]);
    }

    Ok(hasher.digest())
}

const CP1250_HIGH: [char; 128] = [
    '€', '\u{81}', '‚', '\u{83}', '„', '…', '†', '‡', '\u{88}', '‰', 'Š', '‹', 'Ś', 'Ť', 'Ž', 'Ź',
    '\u{90}', '‘', '’', '“', '”', '•', '–', '—', '\u{98}', '™', 'š', '›', 'ś', 'ť', 'ž', 'ź',
    '\u{a0}', 'ˇ', '˘', 'Ł', '¤', 'Ą', '¦', '§', '¨', '©', 'Ş', '«', '¬', '\u{ad}', '®', 'Ż', '°',
    '±', '˛', 'ł', '´', 'µ', '¶', '·', '¸', 'ą', 'ş', '»', 'Ľ', '˝', 'ľ', 'ż', 'Ŕ', 'Á', 'Â', 'Ă',
    'Ä', 'Ĺ', 'Ć', 'Ç', 'Č', 'É', 'Ę', 'Ë', 'Ě', 'Í', 'Î', 'Ď', 'Đ', 'Ń', 'Ň', 'Ó', 'Ô', 'Ő', 'Ö',
    '×', 'Ř', 'Ů', 'Ú', 'Ű', 'Ü', 'Ý', 'Ţ', 'ß', 'ŕ', 'á', 'â', 'ă', 'ä', 'ĺ', 'ć', 'ç', 'č', 'é',
    'ę', 'ë', 'ě', 'í', 'î', 'ď', 'đ', 'ń', 'ň', 'ó', 'ô', 'ő', 'ö', '÷', 'ř', 'ů', 'ú', 'ű', 'ü',
    'ý', 'ţ', '˙',
];

/// Repairs tags labeled in Latin-1 for some fucking reason.
fn fix_legacy_encoding(s: String) -> String {
    if s.is_ascii() || s.chars().any(|c| c as u32 > 0xFF) {
        return s;
    }
    s.chars()
        .map(|c| match c as u32 {
            0..=0x7F => c,
            b => CP1250_HIGH[(b - 0x80) as usize],
        })
        .collect()
}
