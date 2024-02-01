# Cache git sources from 7Ji/git-mirrorer instance for Arch's makepkg routine

Clone and build this project:
```
git clone https://github.com/7Ji/archpkg_cachegit.git
cd archpkg_cachegit
cargo build --release
```

Optionally, install the binary to a place which is indexed in `PATH`:
```
strip target/release/archpkg_cachegit -o ~/bin/archpkg_cachegit
```

From now on, for any `PKGBUILD`, run `archpkg_cachepkg` before `makepkg` to cache git sources, e.g.

```
git clone https://aur.archlinux.org/qbittorrent-enhanced-nox-git.git
cd qbittorrent-enhanced-nox-git
~/bin/archpkg_cachegit http://gmr.lan
```

Since our git sources are all cached, you can add `--holdver` to avoid cloning/updating the git sources for a second time
```
makepkg --holdver
```

For any future updates, remember to run cacher before `makepkg`:

```
~/bin/archpkg_cachegit http://gmr.lan
makepkg --holdver
```