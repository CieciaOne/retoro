-- Create users table
CREATE TABLE users (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    salt VARCHAR(255) NOT NULL
);

-- Add index on name column
CREATE INDEX idx_users_name ON users(name);

-- Create threads table
CREATE TABLE threads (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Create posts table
CREATE TABLE posts (
    id UUID PRIMARY KEY,
    thread_id UUID NOT NULL,
    author_id UUID NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (thread_id) REFERENCES threads(id),
    FOREIGN KEY (author_id) REFERENCES users(id)
);

-- Create index on posts.thread_id for faster retrieval
CREATE INDEX idx_posts_thread_id ON posts(thread_id);

-- Create index on posts.author_id for faster retrieval
CREATE INDEX idx_posts_author_id ON posts(author_id);

-- Create index on posts.created_at for efficient ordering
CREATE INDEX idx_posts_created_at ON posts(created_at);
