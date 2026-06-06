fmt:
    cargo fmt
    git ls-files | grep -E '\.(yml|md|json)$' | xargs deno fmt

fix:
    just fmt
    cargo fix
