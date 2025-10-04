-- Your SQL goes here
CREATE TABLE `accounts`(
	`id` TEXT NOT NULL PRIMARY KEY,
	`oidc` TEXT NOT NULL UNIQUE,
	`name` TEXT NOT NULL,
	`mail` TEXT NOT NULL
);

CREATE TABLE `oidc`(
	`token` TEXT NOT NULL PRIMARY KEY,
	`pkce_verifier` TEXT NOT NULL,
	`csrf_token` TEXT NOT NULL,
	`nonce` TEXT NOT NULL,
	`expires` DATE NOT NULL
);

CREATE TABLE `sessions`(
	`token` TEXT NOT NULL PRIMARY KEY,
	`account_id` TEXT NOT NULL,
	`expires` DATE NOT NULL,
	FOREIGN KEY (`account_id`) REFERENCES `accounts`(`id`)
);

