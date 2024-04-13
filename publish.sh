#! /bin/bash

tmp=$(mktemp -d)

echo "$tmp"
currdir=$(pwd)

cp -r src "$tmp"/.
cp -r README.md "$tmp"/.

### Publish bevy_xpbd_2d_interp
sed 's#\.\./\.\./src#src#g' crates/bevy_xpbd_2d_interp/Cargo.toml > "$tmp"/Cargo.toml
cp -r crates/bevy_xpbd_2d_interp/examples "$tmp"/.
# cd "$tmp" && cargo publish --dry-run
cd "$tmp" && cargo publish

### Remove the 2D examples and return to previous directory
rm -rf examples
cd "$currdir" || exit

### Publish bevy_xpbd_3d_interp
sed 's#\.\./\.\./src#src#g' crates/bevy_xpbd_3d_interp/Cargo.toml > "$tmp"/Cargo.toml
cp -r crates/bevy_xpbd_3d_interp/examples "$tmp"/.
# cd "$tmp" && cargo publish --dry-run
cd "$tmp" && cargo publish

rm -rf "$tmp"
