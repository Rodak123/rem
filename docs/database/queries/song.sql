/*
Queries for a song.

Input:
- songHash

Output:
- Song info
    - title
    - duration seconds
    - is favorite

- All song artists
    - id
    - name

- Song album
    - id
    - title
    - cover art
*/

-- Song details
SELECT
    Song.title AS songTitle, 
    Song.durationSeconds AS songDurationSeconds, 
    Song.isFavorite AS songIsFavorite
FROM Song
WHERE Song.hash = '1111111'
;

-- Song artists
SELECT 
    Creator.id AS artistId,
    Creator.name AS artistName
FROM CreatorInSong
LEFT JOIN Creator ON CreatorInSong.creatorId = Creator.id
WHERE
    CreatorInSong.creatorRoleName = 'artist'
    AND CreatorInSong.songHash = '1111111'
;

-- Song album
SELECT
    Album.id AS albumId,
    Album.title AS albumTitle,
    Album.artPath AS coverArt
FROM SongInAlbumDisk
LEFT JOIN Album ON SongInAlbumDisk.albumId = Album.id
WHERE
    SongInAlbumDisk.songHash = '1111111'
;
