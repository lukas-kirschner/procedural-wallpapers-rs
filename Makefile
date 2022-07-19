MODES := clouds flow islands lightning nearestpoint tangles cellularone squares squareshor squaresver squaresdiag

export RUST_BACKTRACE=full

define build_thumbnail =
$(1):
	@echo "Building Thumbnail for $$@"
	cargo run --release -- --mode "$$@" --width 400 --height 400 --seed 123456 -o "examples/$$@.png"
endef

.PHONY: all thumbnails clean

thumbnails: $(MODES)

all: thumbnails

$(foreach MODE, $(MODES), $(eval $(call build_thumbnail,$(MODE))) )

clean:
	rm -f "examples/$(BUILDS)"