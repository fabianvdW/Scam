TARGET = x86_64-pc-windows-msvc

ifeq ($(shell echo "test"), "test")
	CLEAN = rmdir /s /q "tmp"
	MOVE = move
	SET = set
else
	CLEAN = $(RM) -rf "tmp"
	MOVE = mv
	SET = export
endif

default:
	$(SET) RUSTFLAGS="-Cprofile-generate=./tmp/pgo-data -C target-cpu=native" && \
	cargo build --release --target=$(TARGET)
	"./target/$(TARGET)/release/scam.exe" bench
	"./target/$(TARGET)/release/scam.exe" perftbench
	llvm-profdata merge -o ./tmp/pgo-data/merged.profdata ./tmp/pgo-data
	$(SET) RUSTFLAGS="-Cprofile-use=./tmp/pgo-data/merged.profdata -C target-cpu=native" && \
	cargo build --release --target=$(TARGET)
	$(CLEAN)
	$(MOVE) .\target\$(TARGET)\release\scam.exe Scam.exe
