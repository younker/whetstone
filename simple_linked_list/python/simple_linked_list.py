class Node:
    def __init__(self, value, next_node=None):
        self._value = value
        self._next_node = next_node

    def value(self):
        return self._value

    def next(self):
        return self._next_node


class LinkedList:
    def __init__(self, values=[]):
        self._length = 0
        self._head = None

        for value in values:
            self.push(value)

    def __iter__(self):
        node = self._head
        while node:
            yield node.value()
            node = node.next()

    def __len__(self):
        return self._length

    def head(self):
        if self._head is None:
            raise EmptyListException("Empty list")

        return self._head

    def push(self, value):
        node = Node(value, self._head)
        self._length += 1
        self._head = node

    def pop(self):
        node = self.head()
        self._length -= 1
        self._head = node.next()
        return node.value()

    def reversed(self):
        return LinkedList(list(self))


class EmptyListException(Exception):
    pass
