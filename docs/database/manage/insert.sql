-- Insert Genres
INSERT INTO "Genre" ("name") VALUES 
('Electronic'), 
('Rock'), 
('Synthwave');

-- Insert Moods
INSERT INTO "Mood" ("name") VALUES 
('Energetic'), 
('Melancholic'), 
('Chill');

-- Insert Keywords
INSERT INTO "Keyword" ("name") VALUES 
('fun'),
('fast');

-- Insert Creators
INSERT INTO "Creator" ("id", "name", "comment") VALUES 
(1, 'Daft Punk', 'A French electronic music duo'),
(2, 'Wasureranneyo', 'Japanese rock and punk-rock band'),
(3, 'Julian Casablancas', NULL),
(4, 'The Voidz', NULL);

-- Insert Songs
INSERT INTO "Song" ("hash", "title", "comment", "rating", "isFavorite", "dateAdded", "filePath", "durationSeconds") VALUES 
(1111111, 'Infinity Repeating (2013 demo)', NULL, 8, 0, '2026-01-10 12:00:00', '../music/infinity-repeating.mp3', 239),
(2222222, 'Nannmonee', 'From Chainsmoker Cat', 10, 1, '2026-02-14 15:30:00', '../music/nannmonee.mp3', 171);

-- Link Creators to Songs
INSERT INTO "CreatorInSong" ("creatorRoleName", "creatorId", "songHash") VALUES 
('artist', 1, 1111111),
('artist', 3, 1111111),
('artist', 4, 1111111),
('composer', 1, 1111111),
('artist', 2, 2222222);

-- Associate Songs with Genres
INSERT INTO "SongGenre" ("songHash", "genreName") VALUES 
(1111111, 'Synthwave'),
(1111111, 'Electronic'),
(2222222, 'Rock');

-- Associate Songs with Moods
INSERT INTO "SongMood" ("songHash", "moodName") VALUES 
(1111111, 'Melancholic'),
(2222222, 'Energetic');

-- Associate Songs with Keywords
INSERT INTO "SongKeyword" ("songHash", "keywordName") VALUES 
(1111111, 'fun'),
(2222222, 'fun'),
(2222222, 'fast');

-- Insert Song Play History
INSERT INTO "SongPlayed" ("dateTime", "songHash") VALUES 
('2026-03-01 08:15:00', 1111111),
('2026-03-01 08:20:00', 1111111),
('2026-03-02 21:45:00', 2222222);

-- Insert Playlists
INSERT INTO "Playlist" ("id", "title", "comment", "artPath", "order") VALUES 
(1, 'Bangers', 'These songs are great', '../art/playlist_workout.jpg', 1);

-- Add Songs to Playlists
INSERT INTO "SongInPlaylist" ("playlistId", "order", "songHash") VALUES 
(1, 1, 1111111);  

-- Insert Albums
INSERT INTO "Album" ("id", "title", "comment", "year", "rating", "artPath") VALUES 
(1, 'Random Access Memories (10th Anniversary Edition)', NULL, 2023, 7, '../art/random-access-memories-10th-anniversary-edition.jpg'),
(2, 'Nannmonee', NULL, 2026, 9, '../art/nannmonne.jpg');

-- Insert Album Disks
INSERT INTO "AlbumDisk" ("albumId", "number") VALUES 
(1, 2),
(2, 1);

-- Place Songs onto Album Disks
INSERT INTO "SongInAlbumDisk" ("songHash", "albumId", "albumDiskNumber", "order") VALUES 
(1111111, 1, 2, 4),
(2222222, 2, 1, 1);

-- Link Album Artists
INSERT INTO "AlbumArtist" ("albumId", "creatorId") VALUES 
(1, 1),
(2, 2);

-- Insert Lyrics Lines
INSERT INTO "LyricsLine" ("startTimeMillis", "songHash", "locale", "lyrics") VALUES 
(17000, 1111111, 'en', 'I was crossing the border'),
(20000, 1111111, 'en', 'Two old friends coincidence');