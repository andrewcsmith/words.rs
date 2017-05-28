#! /usr/bin/env ruby

require 'pty'

phrase = "no recordamos el verbo pero el sonido del verbo".split(' ')

File.open './poem-es-end.txt', 'w' do |file|
  (4..4).each do |i|
    phrase.each_slice(i).each_cons(2) do |a, b|
      cmd = ['./target/release/words', "'#{a.join(' ')}'", "'#{b.join(' ')}'", 'data/es_large'].join(' ')
      puts "running #{cmd}"
      PTY.spawn cmd do |stdout, stdin, pid|
        file.puts stdout.read.strip
      end
    end

    file.puts "\n\n"
  end
end
