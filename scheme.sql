-- Create 'users' table to store Telegram users
CREATE TABLE IF NOT EXISTS users (
    id BIGINT PRIMARY KEY, -- Unique user ID (Telegram chat ID)
);

-- Create 'subscribed_users' table to store subscriptions of users
CREATE TABLE IF NOT EXISTS subscribed_users (
    user_id BIGINT PRIMARY KEY, -- Foreign key referring to the 'users' table
    proxy_types TEXT NOT NULL,  -- List of proxy types the user is subscribed to, stored as a JSON array
);


