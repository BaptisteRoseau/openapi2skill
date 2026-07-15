# Pet

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
