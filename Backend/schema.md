## Database Schema Layout
The layout of the database should be as follows for concise and minimal verification.

```
/users
│
├── user1
│   ├── user_info.json    # Contains user information (name, email, etc.)
│   └── history/
│       ├── document1.json
│       ├── document2.json
│       └── ...
│
├── user2
│   ├── user_info.json
│   └── history/
│       ├── document1.json
│       ├── document2.json
│       └── ...
│
└── ...
```

Each user_info.json contains:
```json
{
    "id": "user1",
    "name": "John Doe",
    "email": "john@example.com",
    "password": "securepassword123",  // Add this field (use hashed passwords in production!)
    "created_at": "2023-07-01T12:00:00Z",
    "last_login": "2023-07-15T09:30:00Z"
}
```

Each document in history/ follows this format:
```json
{
    "id": "document1",
    "title": "Sample Document",
    "content": "Document content here",
    "created_at": "2023-07-02T14:30:00Z",
    "last_updated_at": "2023-07-05T10:15:00Z",
}
```
