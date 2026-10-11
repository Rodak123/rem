/*
Queries for a playlist.

Input:
- playlistId

Output:
- Playlist
    - title
    - comment
    - art

- Playlist stats
    - song count
    - total duration

- Songs in order
    - id
*/

-- Playlist
SELECT
    Playlist.title AS playlistTitle,
    Playlist.comment AS playlistComment,
    Playlist.artPath AS coverArt
FROM Playlist
WHERE
    Playlist.id = '1'
;

-- Playlist stats
SELECT
    COUNT(*) as songCount,
    SUM(Song.durationSeconds) as totalDurationSeconds
FROM SongInPlaylist
LEFT JOIN Song ON Song.hash = SongInPlaylist.songHash 
WHERE SongInPlaylist.playlistId = '1'
;

-- Songs in order
SELECT
    SongInPlaylist.songHash,
    SongInPlaylist.'order'
FROM SongInPlaylist
WHERE 
    SongInPlaylist.playlistId = '1'
ORDER BY 
    SongInPlaylist.'order' ASC
;