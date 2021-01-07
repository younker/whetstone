class SimpleLinkedList
  include Enumerable

  attr_accessor :length, :head
  def initialize(nodes=nil)
    @length = 0
    @head = nil

    Array(nodes).each { |n| self.push(n) }
  end

  def push(node)
    node = Node.wrap(node)
    @length += 1
    node.next = @head
    @head = node
    self
  end

  def pop
    @length -= 1
    @head.tap { @head = @head.next if @head }
  end

  def reverse!
    # This would be easier/cleaner but this is suppose to be an in-place
    # transform:
    # SimpleLinkedList.new(to_a)

    arr = []
    while node = pop
      arr << node
    end

    arr.each { |n| push(n) }

    self
  end

  def each
    node = @head
    while node
      yield node
      node = node.next
    end
  end

  def to_a
    map(&:value)
  end
end
