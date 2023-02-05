.PHONY: clean wasm setup-mingw
.PHONY: win.exe m1.exe win.zip m1.zip win.resources m1.resources

WIN64 = x86_64-pc-windows-gnu
MACINTEL = x86_64-apple-darwin
M1 = aarch64-apple-darwin
WASM = wasm32-unknown-unknown

PLATFORMS = $(WIN64) $(WASM)
EXECUTABLE=root-tactics


wasm:
	wasm-bindgen target/$(WASM)/release/${EXECUTABLE}.wasm --out-dir wasm --no-modules --no-typescript

setup-cross:
	rustup target add $(WIN64)
	rustup target add $(M1)

setup-mingw:
	sudo apt install mingw-w64

clean:
	rm -rf target/* win/* m1/*

win.exe:
	cargo build --release --target $(WIN64)

m1.exe:
	cargo build --release --target $(M1)

macintel.exe:
	cargo build --release --target $(MACINTEL)

wasm.exe:
	cargo build --release --target $(WASM)

win.resources m1.resources: %.resources:
	rm -rf $*/data $*/resources $*/doc
	cp -rf resources data doc controls.md dev-plan.txt $*/

win.zip: win.exe win.resources
	ARCHIVE=$(PWD)/win/${EXECUTABLE}$$(date "+%Y-%m-%dT%H-%M").zip; \
	zip -r -P 123 $${ARCHIVE} data resources doc controls.md dev-plan.txt && \
	cd target/$(WIN64)/release/ && \
	zip -r -P 123 $${ARCHIVE} ${EXECUTABLE}.exe
	rm -rf win/data win/resources win/${EXECUTABLE}.exe

m1.zip: m1.exe m1.resources
	ARCHIVE=$(PWD)/m1/${EXECUTABLE}$$(date "+%Y-%m-%dT%H-%M").zip; \
	zip -r -P 123 $${ARCHIVE} data resources doc controls.md dev-plan.txt && \
	cd target/$(M1)/release/ && \
	zip -r -P 123 $${ARCHIVE} ${EXECUTABLE}
	rm -rf m1/data m1/resources m1/${EXECUTABLE}
