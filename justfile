crate := "cbz-thumbnailer"
cbz_thumbnailer := "cbz-thumbnailer"
katalog_proxy := "katalog-proxy"

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

# Run autoinherit
autoinherit:
	cargo autoinherit --prefer-simple-dotted

install-katalog-proxy:
	cargo +nightly install --path {{katalog_proxy}} -Z build-std=std,panic_abort -Z build-std-features="optimize_for_size"

build-katalog-proxy *EXTRA:
	cargo +nightly build --release -p {{katalog_proxy}} -Z build-std=std,panic_abort -Z build-std-features="optimize_for_size" {{EXTRA}}

install-cbz-thumbnailer:
	cargo +nightly install --path {{cbz_thumbnailer}} -Z build-std=std,panic_abort -Z build-std-features="optimize_for_size"

build-cbz-thumbnailer *EXTRA:
	cargo +nightly build --release -p {{cbz_thumbnailer}} -Z build-std=std,panic_abort -Z build-std-features="optimize_for_size" {{EXTRA}}

