#!/usr/bin/env ruby
# frozen_string_literal: true

# Input is
input = ARGF.read.split("\n")

# PART 1
#
# Combine the first and last digit to get a 2-digit number

numbers = input.map do |l|
  (l[l.index(/\d/)] + l[l.rindex(/\d/)]).to_i
end

puts "Part 1: #{numbers.sum}"

# PART 2
#
# As above, but also recognize digits as words i.e. 'one' == 1

replacements = { 'one' => '1e',
                 'two' => '2o',
                 'three' => '3e',
                 'four' => '4r',
                 'five' => '5e',
                 'six' => '6x',
                 'seven' => '7n',
                 'eight' => '8t',
                 'nine' => '9e',
                 'zero' => '0o' }

input.map! do |l|
  10.times do
    l.sub!(
      Regexp.union(replacements.keys),
      replacements
    )
  end

  l
end

numbers = input.map do |l|
  (l[l.index(/\d/)] + l[l.rindex(/\d/)]).to_i
end

puts "Part 2: #{numbers.sum}"
