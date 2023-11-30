#!/usr/bin/env sh

docker run --interactive --tty --rm --volume $(pwd):/workdir mqsoh/knot "./src/rust/literate/*"
