#!/usr/bin/env ruby

require 'json'

data = JSON.parse(STDIN.read)
file_path = data.dig('tool_input', 'file_path')
content = File.read(file_path)

unless content.end_with?("\n")
  puts "Adding newline to EOF: #{file_path}"
  File.write(file_path, "#{content}\n")
end
