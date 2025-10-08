-- Your SQL goes here
CREATE TABLE `sessions`(
	`token` TEXT NOT NULL PRIMARY KEY,
	`account_id` INTEGER NOT NULL,
	`expires` TIMESTAMP NOT NULL,
	FOREIGN KEY (`account_id`) REFERENCES `accounts`(`id`)
);

CREATE TABLE `accounts`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`oidc` TEXT NOT NULL UNIQUE,
	`name` TEXT NOT NULL,
	`mail` TEXT NOT NULL,
	`admin` BOOL NOT NULL
);

