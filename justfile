fmt:
    cargo fmt
    find -name '*.json' -or -name '*.yml' -or -name '*.md' | grep -v '/target/' | xargs deno fmt

fix:
    just fmt
    cargo fix