#!/bin/sh

if peg src/parse.rustpeg > src/parse.gen.rs; then
    cat <<HEADER > src/parse.rs
// Generated by generate.sh.
// You may want to edit parse.rustpeg or parse.preamble.rs.
HEADER
    cat src/parse.preamble.rs           >> src/parse.rs
    cat src/parse.gen.rs | sed '/^#!/d' >> src/parse.rs
fi

rm -f src/parse.gen.rs
