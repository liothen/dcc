#!/usr/bin/env ruby
require 'mechanize'
agent = Mechanize.new
source = agent.get('http://directus.darkscience.net:6789/')
problem = source.at("p").to_html.scan(/\d+/).map!(&:to_i)
form = source.form()
form.answer = problem.inject(:*)
agent.submit(form, form.buttons.first)
result = agent.submit(form, form.buttons.first)
puts "Solution for #{problem.join(' x ')} is #{problem.inject(:*)} (#{result.at("body").content.to_s}!)"