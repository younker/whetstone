# Schema Inferrer

Exercise to do some basic schema inference and to implement:
- `oneOf`: reflect understanding of the various types a field may be
- `recursive discovery`: print out basic of a deeploy nested structure

### Example

For the following structure (currently hard-coded in `main.js`):
```json
{
  "id": 42,
  "bar": [
    {
      "a": "eh?",
      "b": { "bee": "B" }
    },
    true,
    123.123
  ]
},
```

And command:

```bash
$ cd ~/projects/whetstone/schema_inferrer/javascript
$ node main.js | jq
```

You would receive the following output:

```json
{
  "type": "object<Hash>",
  "properties": {
    "id": {
      "type": "number"
    },
    "bar": {
      "type": "object<Array>",
      "oneOf": [
        {
          "type": "object<Hash>",
          "properties": {
            "a": {
              "type": "string"
            },
            "b": {
              "type": "object<Hash>",
              "properties": {
                "bee": {
                  "type": "string"
                }
              }
            }
          }
        },
        {
          "type": "boolean"
        },
        {
          "type": "number"
        }
      ]
    }
  }
}
```

### Next Step...

Passing in several responses will not work. This needs to be changed so that as responses are fed into `receiveSchema`, the current understanding of the schema is updated and not overwritten.

There are many other problems but we can start there.
