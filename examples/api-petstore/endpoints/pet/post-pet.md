# POST /pet

| | |
|--|--|
| **Method** | `POST` |
| **URL** | `/pet` |
| **Full URL** | `http://mypetstore.com/api/v1/pet` |
| **Full URL** | `https://127.0.0.1:8080/api/pet` |
| **Auth** | None |
| **Request Content-Type** | `application/json`, `application/xml` |

## Input

### Payload

```jsonc
{
  "id": 0,  // integer (int64), optional
  "category": {
    "id": 0,  // integer (int64), optional
    "name": "string"  // string, optional
  },
  "name": "doggie",  // string, required
  "photoUrls": [  // array of string, required
    "string"
  ],
  "tags": [  // array of Tag, optional
    {
      "id": 0,  // integer (int64), optional
      "name": "string"  // string, optional
    }
  ],
  "status": "available"  // string, optional, enum: "available", "pending", "sold", pet status in the store
}
```

## Response 405

Invalid input

