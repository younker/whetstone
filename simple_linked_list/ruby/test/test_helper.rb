require 'minitest/autorun'

# Adds color to spec output
# https://github.com/kern/minitest-reporters
require 'minitest/reporters'
Minitest::Reporters.use!

# Allows you to focus on a single test
# https://github.com/seattlerb/minitest-focus
require 'minitest/focus'

# So we can use it at any point without the overhead of a require
require 'pry'
