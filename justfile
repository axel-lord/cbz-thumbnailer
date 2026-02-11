crate := "cbz-thumbnailer"

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

install:
	cargo +nightly install --path {{crate}} -Z build-std=std,panic_abort -Z build-std-features="optimize_for_size"

build *EXTRA:
	cargo +nightly build --release -p {{crate}} -Z build-std=std,panic_abort -Z build-std-features="optimize_for_size" {{EXTRA}}

