# (Simple) Linked List

- helpful when insertion/deletion is primary concern
- do not need random access, only storage
- pervasive in functional programming languages (eg Clojure, Erlang, Haskell) and less common in imperative languages (eg Ruby, Python)
- often used to represent sequences (LIFO stack)

## Interface

For these exercises, the linked list must implement the following:
- `length`: return the length of the linked list
- `peek`: (aka `head`) returns the head node
- `push`: places a new node at the head of the list
- `pop`: removes the node at the head of the list and replaces with it's `next` node
- `reverse`: returns a linked list with the nodes in reverse order

**Note**: In javascript, `push` and `pop` deal with operations at the end of an array while `shift` and `unshift` operate on the beginning. This can get confusing if you push/pop based on context (linked list vs array).

## Terms
- `head`:
- `tail`: node having "none" as next reference
- `traversal`: following pointers from node-to-node. aka `(link|node) hopping`

## Comparison w/Array

#### Linked Lists
- constant time insertion/deletion
- random access is not allowed
- access element in linked list is o(n) time (to traverse from head to element)
- do not have indices
- no fixed size
- uses space proportional to number of nodes

#### Arrays
- o(n) time for insertion/deletion operation
- constant time access
- random access allowed
- uses indices

## Cost
- `access`:
  - list: o(n)
  - array: o(1)
- `insert/rm` beginning:
  - list: o(1)
  - array: o(n)
- `insert/rm` middle:
  - list: o(n)
  - array: o(n)
- `insert/rm` end:
  - list: o(n)
  - array: o(1)
