#! /usr/bin/env ruby

require 'pty'

phrase = "we remember not the word but the sound of the word".split(' ')

File.open './poem-two.txt', 'w' do |file|
  phrase.each_cons(2) do |a, b|
    cmd = ['./target/release/examples/phrase_morph', a, b].join(' ')
    puts "running #{cmd}"
    PTY.spawn cmd do |stdout, stdin, pid|
      file.puts stdout.read 
    end
  end

  file.puts "\n\n"

  phrase.each_slice(2).each_cons(2) do |a, b|
    cmd = ['./target/release/examples/phrase_morph', "'#{a.join(' ')}'", "'#{b.join(' ')}'"].join(' ')
    puts "running #{cmd}"
    PTY.spawn cmd do |stdout, stdin, pid|
      file.puts stdout.read 
    end
  end

  # file.puts "\n\n"
  #
  # phrase.each_slice(3).each_cons(2) do |a, b|
  #   cmd = ['./target/release/examples/phrase_morph', "'#{a.join(' ')}'", "'#{b.join(' ')}'"].join(' ')
  #   puts "running #{cmd}"
  #   PTY.spawn cmd do |stdout, stdin, pid|
  #     file.puts stdout.read 
  #   end
  # end
  #
  # file.puts "\n\n"
  #
  # phrase.each_slice(4).each_cons(2) do |a, b|
  #   cmd = ['./target/release/examples/phrase_morph', "'#{a.join(' ')}'", "'#{b.join(' ')}'"].join(' ')
  #   puts "running #{cmd}"
  #   PTY.spawn cmd do |stdout, stdin, pid|
  #     file.puts stdout.read 
  #   end
  # end
end
