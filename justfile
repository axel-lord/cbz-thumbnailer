crate := "cbz-thumbnailer"
cbz_thumbnailer := "cbz-thumbnailer"
katalog_proxy := "katalog-proxy"
katalog_proxy_thumbnailer := "katalog-proxy-thumbnailer"

default:
	just --list

# Generate documentation for default feature set.
docs *EXTRA:
	cargo doc -p {{crate}} {{EXTRA}}

# Generate documentation for default feature set.
docs-nightly *EXTRA:
	RUSTDOCFLAGS='--cfg=docsrs' cargo +nightly doc -p {{crate}} {{EXTRA}}

# Generate documentation for all features.
docs-nightly-all *EXTRA:
	RUSTDOCFLAGS='--cfg=docsrs' cargo +nightly doc --all-features -p {{crate}} {{EXTRA}}

# Generate documentation for minimal feature set.
docs-min *EXTRA:
	cargo doc --no-default-features -p {{crate}} {{EXTRA}}

# Format crates.
fmt:
	cargo fmt --all

# Run cargo install for package.
cargo-install package:
	cargo +nightly install --path {{package}} -Z build-std=std,panic_abort -Z build-std-features="optimize_for_size"

# Run cargo build for package.
cargo-build package:
	cargo +nightly build --release -p {{package}} -Z build-std=std,panic_abort -Z build-std-features="optimize_for_size"

# Run autoinherit
autoinherit:
	cargo autoinherit --prefer-simple-dotted

install: install-katalog-proxy install-cbz-thumbnailer install-katalog-proxy-thumbnailer

install-katalog-proxy: (cargo-install f"{{katalog_proxy}}")
install-katalog-proxy-thumbnailer: (cargo-install f"{{katalog_proxy_thumbnailer}}")
install-cbz-thumbnailer: (cargo-install f"{{cbz_thumbnailer}}")

build-katalog-proxy: (cargo-install f"{{katalog_proxy}}")
build-katalog-proxy-thumbnailer: (cargo-install f"{{katalog_proxy_thumbnailer}}")
build-cbz-thumbnailer: (cargo-install f"{{cbz_thumbnailer}}")
