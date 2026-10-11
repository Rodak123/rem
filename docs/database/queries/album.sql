/*
Queries for an album.

Input:
- albumId

Output:
- Album
    - title
    - comment
    - cover art

- Album stats
    - disk count
    - song count
    - total duration

- Songs in order
    - id
*/

-- Album
SELECT
    Album.title AS albumTitle,
    Album.comment AS albumComment,
    Album.artPath AS coverArt
FROM Album
WHERE
    Album.id = '1'
;

-- Album stats
SELECT
    COUNT(*) as songCount,
    COUNT(DISTINCT SongInAlbumDisk.albumDiskNumber) as diskCount,
    SUM(Song.durationSeconds) as totalDurationSeconds
FROM SongInAlbumDisk
LEFT JOIN Song ON Song.hash = SongInAlbumDisk.songHash 
WHERE SongInAlbumDisk.albumId = '1'
;

-- Songs in order
SELECT
    SongInAlbumDisk.songHash,
    SongInAlbumDisk.albumDiskNumber as diskNumber,
    SongInAlbumDisk.'order'
FROM SongInAlbumDisk
WHERE 
    SongInAlbumDisk.albumId = '1'
ORDER BY 
    SongInAlbumDisk.albumDiskNumber ASC, 
    SongInAlbumDisk.'order' ASC
;