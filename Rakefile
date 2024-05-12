require "peppermint/rake"
require "faraday"
require "semantic_logger"
require "tomlrb"

unless ENV["LOG_LEVEL"].nil?
  SemanticLogger.default_level = ENV["LOG_LEVEL"].downcase.strip.to_sym
end
SemanticLogger.add_appender(io: $stdout, formatter: :color)

desc "publish a release"
task :release do
  logger = SemanticLogger["release"]
  cargo_toml = begin
    Tomlrb.load_file(File.join(__dir__, "Cargo.toml"), symbolize_keys: true)
  rescue => e
    logger.debug e
    logger.error "issue reading or parsing Cargo.toml"
    exit 1
  end
  crate_name = cargo_toml[:package][:name]
  local_version = cargo_toml[:package][:version]
  conn = Faraday.new("https://crates.io/api/v1/crates/#{crate_name}") do |f|
    f.response :json
  end
  res = conn.get.body
  crate_version = begin
    res["crate"]["max_version"]
  rescue
    logger.error "couldn't get crate version"
    exit 1
  end
  if crate_version == local_version
    logger.error "crate version needs to be bumped; it matches an already live version, #{crate_version}"
    exit 1
  else
    logger.info "publishing crate version #{local_version}"
    sh "cargo publish"
  end
end
