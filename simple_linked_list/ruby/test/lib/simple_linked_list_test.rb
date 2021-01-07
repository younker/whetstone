require 'test_helper'
require_relative '../../lib/node'
require_relative '../../lib/simple_linked_list'

class LinkedListTest < Minitest::Test
  def test_list_length_for_empty_list
    list = SimpleLinkedList.new
    assert_equal 0, list.length
  end

  def test_list_length_for_one
    list = SimpleLinkedList.new
    node1 = Node.new(1)
    list.push(node1)
    assert_equal 1, list.length
  end

  def test_list_length_for_many
    list = SimpleLinkedList.new
    node1 = Node.new(1)
    node2 = Node.new(2)
    node3 = Node.new(3)
    list.push(node1).push(node2).push(node3)
    assert_equal 3, list.length
  end

  def test_list_length_after_pop
    list = SimpleLinkedList.new
    node1 = Node.new(1)
    node2 = Node.new(2)
    list.push(node1).push(node2).pop
    assert_equal 1, list.length
  end

  def test_list_length_after_drained
    list = SimpleLinkedList.new
    node1 = Node.new(1)
    node2 = Node.new(2)
    list.push(node1).push(node2)
    list.pop
    list.pop
    assert_equal 0, list.length
  end

  def test_list_head
    list = SimpleLinkedList.new
    node1 = Node.new(1)
    node2 = Node.new(2)
    list.push(node1).push(node2)
    assert_equal node2, list.head
  end

  def test_list_head_after_pop
    list = SimpleLinkedList.new
    node1 = Node.new(1)
    node2 = Node.new(2)
    list.push(node1).push(node2).pop
    assert_equal node1, list.head
  end

  def test_head_for_empty_list
    list = SimpleLinkedList.new
    assert_nil list.head
  end

  def test_head_for_empty_list_after_pop
    list = SimpleLinkedList.new
    node1 = Node.new(1)
    list.push(node1).pop
    assert_nil list.head
  end

  def test_list_push
    list = SimpleLinkedList.new
    node = Node.new(1)
    assert_equal list, list.push(node)
  end

  def test_list_pop
    list = SimpleLinkedList.new
    node = Node.new(1)
    list.push(node)
    assert_equal node, list.pop
  end

  def test_list_pop_empty
    list = SimpleLinkedList.new
    assert_nil list.pop
  end

  def test_list_pop_is_last_in_first_out
    list = SimpleLinkedList.new
    first = Node.new(1)
    second = Node.new(2)
    list.push(first).push(second)
    assert_equal second, list.pop
  end

  def test_list_empty_to_array
    list = SimpleLinkedList.new
    assert_equal [], list.to_a
  end

  def test_list_single_to_array
    list = SimpleLinkedList.new
    first = Node.new(1)
    list.push(first)
    assert_equal [1], list.to_a
  end

  def test_list_multiple_to_array
    list = SimpleLinkedList.new
    first = Node.new(1)
    second = Node.new(2)
    third = Node.new(3)
    list.push(first).push(second).push(third)
    assert_equal [3, 2, 1], list.to_a
  end

  def test_list_create_from_array
    array = [1, 2, 3]
    list = SimpleLinkedList.new(array)
    assert_equal [3, 2, 1], list.to_a
  end

  def test_list_created_from_array_still_made_up_of_elements
    array = [1, 2, 3]
    list = SimpleLinkedList.new(array)
    assert_equal Node, list.pop.class
  end

  def test_list_from_array_still_acts_as_lifo
    array = [1, 2, 3]
    list = SimpleLinkedList.new(array)
    node = list.pop
    assert_equal 3, node.value
  end

  def test_list_in_place_reverse!
    first = Node.new(1)
    second = Node.new(2)
    third = Node.new(3)
    list = SimpleLinkedList.new
    list.push(first).push(second).push(third)

    assert_equal [1, 2, 3], list.reverse!.to_a
  end

  def test_list_in_place_reverse_are_the_same_elements
    first = Node.new(1)
    second = Node.new(2)
    list = SimpleLinkedList.new
    list.push(first).push(second)

    list.reverse!

    assert_equal first, list.pop
    assert_equal second, list.pop
  end

  def test_list_reverse_empty_list
    list = SimpleLinkedList.new
    assert_equal list, list.reverse!
  end

  def test_works_for_1_through_10
    list = SimpleLinkedList.new(1..10)
    expected = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
    assert_equal expected, list.to_a
  end
end
