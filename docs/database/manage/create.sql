CREATE TABLE IF NOT EXISTS "Song" (
	-- Hash of the song
	"hash" INTEGER NOT NULL,
	"title" TEXT NOT NULL,
	"comment" TEXT,
	"rating" INTEGER NOT NULL,
	"isFavorite" BOOLEAN NOT NULL DEFAULT 0,
	"dateAdded" DATETIME NOT NULL,
	"filePath" TEXT NOT NULL,
	"durationSeconds" INTEGER NOT NULL,
	PRIMARY KEY("hash")
);

CREATE TABLE IF NOT EXISTS "SongPlayed" (
	"dateTime" DATETIME NOT NULL,
	"songHash" INTEGER NOT NULL,
	PRIMARY KEY("dateTime"),
	FOREIGN KEY ("songHash") REFERENCES "Song"("hash")
	ON UPDATE NO ACTION ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "Playlist" (
	"id" INTEGER NOT NULL,
	"title" TEXT NOT NULL,
	"comment" TEXT,
	"artPath" TEXT NOT NULL,
	"order" INTEGER NOT NULL UNIQUE,
	PRIMARY KEY("id")
);

CREATE TABLE IF NOT EXISTS "SongInPlaylist" (
	"playlistId" INTEGER NOT NULL,
	"order" INTEGER NOT NULL,
	"songHash" INTEGER NOT NULL,
	PRIMARY KEY("playlistId", "order"),
	FOREIGN KEY ("playlistId") REFERENCES "Playlist"("id")
	ON UPDATE NO ACTION ON DELETE CASCADE,
	FOREIGN KEY ("songHash") REFERENCES "Song"("hash")
	ON UPDATE NO ACTION ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "Creator" (
	"id" INTEGER NOT NULL,
	"name" TEXT NOT NULL,
	"comment" TEXT,
	PRIMARY KEY("id")
);

CREATE TABLE IF NOT EXISTS "CreatorInSong" (
	"creatorRoleName" TEXT NOT NULL CHECK(creatorRoleName IN ('artist', 'composer', 'lyricist', 'publisher', 'conductor')),
	"creatorId" INTEGER NOT NULL,
	"songHash" INTEGER NOT NULL,
	PRIMARY KEY("creatorRoleName", "creatorId", "songHash"),
	FOREIGN KEY ("creatorId") REFERENCES "Creator"("id")
	ON UPDATE NO ACTION ON DELETE CASCADE,
	FOREIGN KEY ("songHash") REFERENCES "Song"("hash")
	ON UPDATE NO ACTION ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "Album" (
	"id" INTEGER NOT NULL,
	"title" TEXT NOT NULL,
	"comment" TEXT,
	"year" INTEGER NOT NULL,
	"rating" INTEGER NOT NULL,
	"artPath" TEXT NOT NULL,
	PRIMARY KEY("id")
);

CREATE TABLE IF NOT EXISTS "AlbumDisk" (
	"number" INTEGER NOT NULL,
	"albumId" INTEGER NOT NULL,
	PRIMARY KEY("number", "albumId"),
	FOREIGN KEY ("albumId") REFERENCES "Album"("id")
	ON UPDATE NO ACTION ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "SongInAlbumDisk" (
	"songHash" INTEGER NOT NULL,
	"albumDiskNumber" INTEGER NOT NULL,
	"albumId" INTEGER NOT NULL,
	"order" INTEGER NOT NULL,
	PRIMARY KEY("albumDiskNumber", "albumId", "order"),
	FOREIGN KEY ("songHash") REFERENCES "Song"("hash")
	ON UPDATE NO ACTION ON DELETE CASCADE,
	FOREIGN KEY ("albumId", "albumDiskNumber") REFERENCES "AlbumDisk"("albumId", "number")
	ON UPDATE NO ACTION ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "AlbumArtist" (
	"albumId" INTEGER NOT NULL,
	"creatorId" INTEGER NOT NULL,
	PRIMARY KEY("albumId", "creatorId"),
	FOREIGN KEY ("albumId") REFERENCES "Album"("id")
	ON UPDATE NO ACTION ON DELETE CASCADE,
	FOREIGN KEY ("creatorId") REFERENCES "Creator"("id")
	ON UPDATE NO ACTION ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "Genre" (
	"name" TEXT NOT NULL,
	PRIMARY KEY("name")
);

CREATE TABLE IF NOT EXISTS "SongGenre" (
	"songHash" INTEGER NOT NULL,
	"genreName" TEXT NOT NULL,
	PRIMARY KEY("songHash", "genreName"),
	FOREIGN KEY ("genreName") REFERENCES "Genre"("name")
	ON UPDATE NO ACTION ON DELETE CASCADE,
	FOREIGN KEY ("songHash") REFERENCES "Song"("hash")
	ON UPDATE NO ACTION ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "Mood" (
	"name" TEXT NOT NULL,
	PRIMARY KEY("name")
);

CREATE TABLE IF NOT EXISTS "SongMood" (
	"songHash" INTEGER NOT NULL,
	"moodName" TEXT NOT NULL,
	PRIMARY KEY("songHash", "moodName"),
	FOREIGN KEY ("moodName") REFERENCES "Mood"("name")
	ON UPDATE NO ACTION ON DELETE CASCADE,
	FOREIGN KEY ("songHash") REFERENCES "Song"("hash")
	ON UPDATE NO ACTION ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "LyricsLine" (
	-- When this lyrics line appears
	"startTimeMillis" INTEGER NOT NULL,
	"songHash" INTEGER NOT NULL,
	-- Language of lyrics
	"locale" TEXT NOT NULL,
	"lyrics" TEXT,
	PRIMARY KEY("startTimeMillis", "songHash", "locale"),
	FOREIGN KEY ("songHash") REFERENCES "Song"("hash")
	ON UPDATE NO ACTION ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "Keyword" (
	"name" TEXT NOT NULL,
	PRIMARY KEY("name")
);

CREATE TABLE IF NOT EXISTS "SongKeyword" (
	"songHash" INTEGER NOT NULL,
	"keywordName" TEXT NOT NULL,
	PRIMARY KEY("songHash", "keywordName"),
	FOREIGN KEY ("songHash") REFERENCES "Song"("hash")
	ON UPDATE NO ACTION ON DELETE CASCADE,
	FOREIGN KEY ("keywordName") REFERENCES "Keyword"("name")
	ON UPDATE NO ACTION ON DELETE CASCADE
);
