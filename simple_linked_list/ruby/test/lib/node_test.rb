require 'test_helper'
require_relative '../../lib/node'

class NodeTest < Minitest::Test
  def test_element
    node = Node.new(1)
    assert_equal 1, node.value
  end

  def test_element_can_hold_a_different_value
    node = Node.new(10)
    assert_equal 10, node.value
  end

  def test_element_next
    node = Node.new(1)
    assert_nil node.next
  end

  def test_element_next_can_be_assigned_to
    n1 = Node.new(1)
    n2 = Node.new(2)
    n1.next = n2
    assert_equal n2, n1.next
  end
end
