require "peppermint/rake/ruby"
require_relative "lib/cvefixer"

task :publish do
  sh "rm -v tmp/*"
  sh "gem build"
  sh "mv cvefixer-#{CVEFixer::VERSION}.gem tmp"
  sh "gem push ./tmp/cvefixer-#{CVEFixer::VERSION}.gem"
  puts "ok. now merge into main. hit enter twice when done."
  $stdin.gets
  $stdin.gets
  sh "git checkout main"
  sh "git pull origin main"
  sh "git", "tag", "v#{CVEFixer::VERSION}"
  sh "git push --tags"
end
