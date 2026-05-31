-- Change server_settings::last_bump to TEXT. Can use Intellij's thing
-- `CREATE UNIQUE INDEX idx_unique_item_user ON aquired_items (item_id, user_id);

UPDATE economy
SET balance = floor(balance)
WHERE balance IS NOT NULL AND balance != floor(balance);

-- ALTER TABLE user_data ADD pronoun_set TEXT DEFAULT 'they' NOT NULL;


-- ALTER TABLE economy ADD from_mc INT NOT NULL DEFAULT 0;
-- ALTER TABLE economy ADD from_mc_lost INT NOT NULL DEFAULT 0;

-- CREATE TABLE swear_jar (
--     user_id TEXT NOT NULL,
--     server_id TEXT NOT NULL,
--     word TEXT NOT NULL,
--     uses INT NOT NULL DEFAULT 0,
--     UNIQUE(user_id, server_id, word)
-- );

-- ALTER TABLE server_count_ruins ADD ruined_by TEXT;
-- ALTER TABLE server_count ADD allow_consecutive_counts INTEGER DEFAULT 0 NOT NULL;

-- ALTER TABLE server_settings ADD streak_reactions INTEGER DEFAULT 0 NOT NULL;
-- ALTER TABLE server_settings ADD streak_end_reactions INTEGER DEFAULT 1 NOT NULL;

CREATE TABLE IF NOT EXISTS persistent_messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    message_id TEXT NOT NULL,
    channel_id TEXT NOT NULL,
    server_id TEXT NOT NULL
);

ALTER TABLE server_settings DROP last_bump;
ALTER TABLE server_settings ADD last_bump TEXT DEFAULT NULL;