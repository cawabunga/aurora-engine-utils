.PHONY: all build clean

# Build the project using wasm-pack
build:
	wasm-pack build --target nodejs

# Clean the project
clean:
	rm -rf pkg target

# Help command to display available targets
help:
	@echo "Available targets:"
	@echo "  all    - Clean and then build the project (default)"
	@echo "  build  - Build the project using wasm-pack"
	@echo "  clean  - Remove build artifacts"
	@echo "  help   - Display this help message"