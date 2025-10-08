-- Your SQL goes here
CREATE TABLE `services`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL,
	`technology` TEXT NOT NULL,
	`website` TEXT NOT NULL,
	`instance` TEXT NOT NULL,
	`icon` TEXT NOT NULL,
	`icon_height` INTEGER NOT NULL,
	`icon_width` INTEGER NOT NULL
);