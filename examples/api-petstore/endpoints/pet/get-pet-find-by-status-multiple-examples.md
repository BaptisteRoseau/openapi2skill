# GET /pet/findByStatus/MultipleExamples

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/pet/findByStatus/MultipleExamples` |
| **Full URL** | `http://mypetstore.com/api/v1/pet/findByStatus/MultipleExamples?status=string` |
| **Full URL** | `https://127.0.0.1:8080/api/pet/findByStatus/MultipleExamples?status=string` |
| **Auth** | petstore_auth (scopes: write:pets, read:pets) |

## Input

### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `status` | array | Yes | Status values that need to be considered for filter |

## Response 200

**Response Content-Type:** `application/json`

successful operation

```jsonc
[
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
]
```

## Response 400

Invalid status value

