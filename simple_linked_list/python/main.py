from simple_linked_list import LinkedList


def main():
    linked_list = LinkedList(['foo', 'bar'])
    print(list(linked_list))

    linked_list.push('baz')
    print(list(linked_list))

    for node in linked_list:
        print(f'node: {node}')

    print(list(linked_list.reversed()))


# python -m main
if __name__ == "__main__":
    main()
