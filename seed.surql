-- ──────────────────────────────────────────────────────────────
-- Demo seed data  (development only — never run in production)
-- ──────────────────────────────────────────────────────────────

-- Admin user  (password: Admin1234)
INSERT IGNORE INTO user {
  id: user:admin,
  email: 'admin@example.com',
  name: 'Admin',
  pass: crypto::argon2::generate('Admin1234'),
  role: 'admin',
  created_at: time::now()
};

-- Regular demo user  (password: Demo1234)
INSERT IGNORE INTO user {
  id: user:demo,
  email: 'demo@example.com',
  name: 'Demo User',
  pass: crypto::argon2::generate('Demo1234'),
  role: 'user',
  created_at: time::now()
};

-- Sample items owned by the demo user
INSERT IGNORE INTO item {
  id: item:one,
  title: 'My First Item',
  description: 'This is a sample item created by the seed script.',
  created_by: user:demo,
  created_at: time::now(),
  updated_at: time::now()
};

INSERT IGNORE INTO item {
  id: item:two,
  title: 'Another Item',
  description: 'Feel free to edit or delete this in the dashboard.',
  created_by: user:demo,
  created_at: time::now(),
  updated_at: time::now()
};

INSERT IGNORE INTO item {
  id: item:three,
  title: 'Admin-Owned Item',
  description: 'Created by the admin account.',
  created_by: user:admin,
  created_at: time::now(),
  updated_at: time::now()
};
