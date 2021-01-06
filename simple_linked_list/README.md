# (Simple) Linked List

- helpful when insertion/deletion is primary concern
- do not need random access, only storage
- pervasive in functional programming languages (eg Clojure, Erlang, Haskell) and less common in imperative languages (eg Ruby, Python)
- often used to represent sequences (LIFO stack)

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
