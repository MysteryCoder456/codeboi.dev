-- Rename 'description' to 'short_description'
ALTER TABLE projects RENAME COLUMN description TO short_description;

-- Add 'long_description' column
ALTER TABLE projects ADD COLUMN long_description TEXT NOT NULL DEFAULT 'A really cool project';
