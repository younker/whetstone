const typeOf = (thing) => {
  let t = typeof thing;
  if (t === 'object') {
    // A dirty and expensive way to avoid the nuances of differentiating an
    // array from an object. Perhaps before we ship to prod we will want to
    // rethink this. But then again, this is never going to prod so...
    const char = JSON.stringify(thing)[0];
    t = char === '{' ? 'object' : 'array';
  }

  return t;
};

const getNodeForValue = (value) => {
  const t = typeOf(value);

  let node;
  if (t === 'array') {
    node = new ArrayNode(value);
  } else if (t === 'object') {
    node = new HashNode(value);
  } else {
    node = new Node(value);
  }

  return node;
};

class Node {
  constructor(value) {
    this.value = value;
  }

  get type() {
    return typeof this.value;
  }

  toString() {
    return `{ "type": "${this.type}" }`;
  }
}

class ArrayNode {
  constructor(arr) {
    this.arr = arr;
  }

  get type() {
    return 'object<Array>';
  }

  get uniqNodes() {
    if (this._uniqNodes) return this._uniqNodes;

    const nodeMap = this.arr.reduce((memo, val) => {
      const node = getNodeForValue(val);

      // TODO: This assumes two versions of the same type are the same when in
      // fact that may not be true (especially with objects).
      const t = node.type;
      if (!memo[t]) memo[t] = node;

      return memo;
    }, {});

    this._uniqNodes = Object.values(nodeMap);
    return this._uniqNodes;
  }

  itemsStr() {
    const nodes = this.uniqNodes.map(n => n.toString()).join(', ');
    return `${nodes}`;
  }

  toString() {
    if (this.uniqNodes.length > 1) {
      return `{ "type": "${this.type}", "oneOf": [ ${this.itemsStr()} ] }`;
    } else {
      return `{ "type": "${this.type}", "items": "${this.itemsStr()}" }`;
    }
  }
}

class HashNode {
  constructor(hash) {
    this._hash = hash;
  }

  get type() {
    return 'object<Hash>';
  }

  get properties() {
    if (this._properties) return this._properties;

    this._properties = Object
      .entries(this._hash)
      .reduce((memo, [key, value]) => {
        memo[key] = getNodeForValue(value);
        return memo;
      }, {});

    return this._properties;
  }

  propertiesStr() {
    if (Object.keys(this.properties).length === 0) return '{asdf}';

    const pairs = Object.entries(this.properties).map(([key, node]) => {
      return `"${key}": ${node.toString()}`;
    }).join(', ');

    return `{ ${pairs} }`;
  }

  toString() {
    return `{ "type": "${this.type}", "properties": ${this.propertiesStr()} }`;
  }
}

class JsonInference {
  constructor() {
    this.schema = {};
  }

  receiveSchema(result) {
    // Object.entries(result).forEach(([key, value]) => {
    //   // const node = getNodeForValue(value);
    //   // this.schema[key] = node;
    //   this.schema = getNodeForValue(value);
    // });
    this.schema = getNodeForValue(result);
  }

  print() {
    // Object.entries(this.schema).forEach(([key, node]) => {
    //   console.log(`${key} => ${node.toString()}\n`);
    // });
    console.log(this.schema.toString());
  }
}

// TODO: multiple responses do not merge into this.schema
responses = [
  // { "id": 42 },
  // { "email": "jason@ynkr.org" },
  // { "tags": ["foo"] },
  // { "seen": { "iphone": true } },
  // { "misc": ["string", 42, {}, 123.123] },
  { "id": 42, "bar": [{ "a": "eh?", "b": { "bee": "B" } }, true, 123.123] },
]

let inferrer = new JsonInference();
responses.map(r => inferrer.receiveSchema(r));
inferrer.print();
