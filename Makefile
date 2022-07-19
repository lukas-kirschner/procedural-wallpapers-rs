all: thumbnails

thumbnails:
	for MODE in clouds flow islands lightning nearestpoint tangles cellularone squares squareshor squaresver squaresdiag; do \
		printf "Building thumbnails for $${MODE}"; \
		RUST_BACKTRACE=full cargo run --release -- --mode "$${MODE}" --width 400 --height 400 --seed 123456 -o "examples/$${MODE}.png"; \
	done