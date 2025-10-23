#!/usr/bin/env ruby

require 'net/http'
require 'json'

token = ENV.fetch('OPENAI_API_KEY')

url = URI.parse('https://api.openai.com/v1/chat/completions')
data = {
  'model': 'gpt-3.5-turbo',
  'messages': [
    {
      role: 'user',
      content: 'https://platform.openai.com/docs/api-reference/chat/create',
    },
    {
      role: 'user',
      content: 'Sumarize to Japanese',
    },
  ],
  'temperature': 0.7,
}
header = {
  'Content-Type': 'application/json',
  Authorization: "Bearer #{token}",
}

resp = Net::HTTP.post(url, data.to_json, header)
raise "Failed! #{resp}" unless resp.code == '200'

json = JSON.parse(resp.body, symbolize_names: true)
json[:choices].each do |choice|
  puts choice[:message][:content]
end
