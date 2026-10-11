/*
Queries for a songs lyrics.

Input:
- songHash
- lyrics locale (en, cz, jp...)

Output:
- Available lyrics locales
- Lyrics
*/

-- List of available lyric locales for a song
SELECT
    DISTINCT LyricsLine.locale
FROM LyricsLine
WHERE
    songHash = '1111111'
;

-- List of lyrics in shown order
SELECT
    LyricsLine.startTimeMillis,
    LyricsLine.lyrics
FROM LyricsLine
WHERE 
    locale = 'en'
    AND songHash = '1111111'
ORDER BY startTimeMillis ASC
;