-- Insert test data for users
INSERT INTO users (id, name, password_hash, created_at, last_active, salt)
VALUES 
('12345678-1234-5678-1234-567812345678', 'Alice Johnson', '$2a$12$12345678901234567890abcdef1234567890abcdef1234', 
  CURRENT_TIMESTAMP - INTERVAL '2 years',
  CURRENT_TIMESTAMP - INTERVAL '1 year',
  'salt123'),
('87654321-8765-4321-8765-43218765432178', 'Bob Smith', '$2a$12$09876543210987654321098765432109876543210', 
  CURRENT_TIMESTAMP - INTERVAL '1 year',
  CURRENT_TIMESTAMP - INTERVAL '6 months',
  'salt456');

-- Insert test data for threads
INSERT INTO threads (id, name, created_at, last_active)
VALUES 
('12345678-1234-5678-1234-567812345678', 'General Discussion',
  CURRENT_TIMESTAMP - INTERVAL '1 year',
  CURRENT_TIMESTAMP - INTERVAL '6 months'),
('87654321-8765-4321-8765-43218765432178', 'Programming Forum',
  CURRENT_TIMESTAMP - INTERVAL '6 months',
  CURRENT_TIMESTAMP);

-- Insert test data for posts
INSERT INTO posts (id, thread_id, author_id, content, created_at)
VALUES 
('12345678-1234-5678-1234-567812345678', 
 '87654321-8765-4321-8765-43218765432178', 
 '12345678-1234-5678-1234-567812345678', 
 'Hello, everyone! This is my first post.',
  CURRENT_TIMESTAMP - INTERVAL '3 months'),
('23456789-2345-6789-2345-67892345678923', 
 '12345678-1234-5678-1234-567812345678', 
 '87654321-8765-4321-8765-43218765432178', 
 'I love this forum!',
  CURRENT_TIMESTAMP - INTERVAL '2 months'),
('34567890-3456-7890-3456-78903456789034', 
 '12345678-1234-5678-1234-567812345678', 
 '12345678-1234-5678-1234-567812345678', 
 'Can someone explain recursion?',
  CURRENT_TIMESTAMP - INTERVAL '1 month');
