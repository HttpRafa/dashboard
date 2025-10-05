-- Your SQL goes here
CREATE TABLE `sessions`(
	`token` TEXT NOT NULL PRIMARY KEY,
	`account_id` TEXT NOT NULL,
	`expires` TIMESTAMP NOT NULL,
	FOREIGN KEY (`account_id`) REFERENCES `accounts`(`id`)
);

CREATE TABLE `accounts`(
	`id` TEXT NOT NULL PRIMARY KEY,
	`oidc` TEXT NOT NULL,
	`name` TEXT NOT NULL,
	`mail` TEXT NOT NULL
);

