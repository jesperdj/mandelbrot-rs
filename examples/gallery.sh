#!/usr/bin/env bash
#
# Render a small gallery of interesting locations in the Mandelbrot set.
#
# Build the release binary first:
#
#     cargo build --release
#
# Then run this script. By default it writes the images to the current
# directory; pass a directory to write them somewhere else:
#
#     examples/gallery.sh [output-dir]

set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
mandelbrot="$repo_root/target/release/mandelbrot"

if [[ ! -x "$mandelbrot" ]]; then
    echo "error: $mandelbrot not found; build it first with 'cargo build --release'" >&2
    exit 1
fi

out_dir="${1:-.}"
mkdir -p "$out_dir"

# Shared size and quality settings. The table palette resolves fine detail
# better than rainbow at these zoom depths.
common=(--width 1920 --height 1080 --palette table --sampler stratified --samples 16 --filter mitchell)

# 1. Mini-Mandelbrot: a complete miniature copy of the whole set, hidden among
#    the filaments (the set is self-similar).
"$mandelbrot" --center-re -0.1592 --center-im 1.0317 --scale 0.02 --max-iterations 1200 "${common[@]}" -o "$out_dir/minibrot.png"

# 2. Seahorse Valley: interlocking spiral "seahorse tail" shapes in the neck
#    between the main cardioid and the large left bulb.
"$mandelbrot" --center-re -0.7453 --center-im 0.1127 --scale 0.0055 --max-iterations 1500 "${common[@]}" -o "$out_dir/seahorse.png"

# 3. Elephant Valley: a parade of spiral "elephant" trunks along the right side
#    of the cardioid.
"$mandelbrot" --center-re 0.3 --center-im 0.02 --scale 0.02 --max-iterations 800 "${common[@]}" -o "$out_dir/elephant.png"

echo "Wrote $out_dir/minibrot.png, $out_dir/seahorse.png and $out_dir/elephant.png"
