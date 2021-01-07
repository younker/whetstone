class Node
  def self.wrap(node)
    return node if node.class == Node
    Node.new(node)
  end

  attr_reader :value
  attr_accessor :next
  def initialize(value, next_node=nil)
    @value = value
    @next = next_node
  end
end
