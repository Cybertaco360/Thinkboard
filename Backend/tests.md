# Backend Manual Test Commands

## 1. Test the health endpoint (`/`)
```bash
curl -i http://127.0.0.1:8080/
```

## 2. Test the `/generate` endpoint
```bash
curl -i -X POST http://127.0.0.1:8080/generate \
  -H "Content-Type: application/json" \
  -d '{"prompt": "Say hello!"}'
```

## 3. Test the `/signin` endpoint with valid credentials
```bash
curl -i -X POST http://127.0.0.1:8080/signin \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "password": "testpassword"}'
```

## 4. Test the `/signin` endpoint with invalid credentials
```bash
curl -i -X POST http://127.0.0.1:8080/signin \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "password": "wrongpassword"}'
```

## 5. Test the `/signin` endpoint with a non-existent user
```bash
curl -i -X POST http://127.0.0.1:8080/signin \
  -H "Content-Type: application/json" \
  -d '{"email": "nouser@example.com", "password": "irrelevant"}'
```

---

# Signup Endpoint Tests

## 6. Test `/signup` with a new user
```bash
curl -i -X POST http://127.0.0.1:8080/signup \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice Example", "email": "alice@example.com", "password": "alicepassword"}'
```

## 7. Test `/signup` with an existing user (should fail)
```bash
curl -i -X POST http://127.0.0.1:8080/signup \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice Example", "email": "alice@example.com", "password": "alicepassword"}'
```

## 8. Test `/signup` with missing fields (should fail)
```bash
curl -i -X POST http://127.0.0.1:8080/signup \
  -H "Content-Type: application/json" \
  -d '{"email": "bob@example.com", "password": "bobpassword"}'
```

## 9. Test `/signup` with empty password (should fail)
```bash
curl -i -X POST http://127.0.0.1:8080/signup \
  -H "Content-Type: application/json" \
  -d '{"name": "Charlie", "email": "charlie@example.com", "password": ""}'
```

> **Note:**  
> - Replace the email and password with actual values from your MongoDB for meaningful results.
> - These commands check if endpoints are reachable and return expected responses, serving as simple integration/unit tests for your backend.
