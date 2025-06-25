SHELL=/bin/bash # Explicitly set shell for $(shell ...) compatibility

# ==============================================================================
# CMAKE-BASED BUILD WITH PREBUILT WXWIDGETS
# ==============================================================================
# Updated to use CMake with prebuilt wxWidgets libraries, similar to const_extractor

BUILD_DIR = build
TARGET_EXEC = xrc_test
APP_BUNDLE_NAME = $(TARGET_EXEC).app

.PHONY: all clean configure build bundle run

# Default target
all: build

# Configure the build with CMake
configure:
	@echo "Configuring XRC test with CMake..."
	@mkdir -p $(BUILD_DIR)
	cd $(BUILD_DIR) && cmake ..

# Build the project
build: configure
	@echo "Building XRC test..."
	cd $(BUILD_DIR) && $(MAKE)
	@if [ -f $(BUILD_DIR)/$(TARGET_EXEC) ]; then \
		echo "XRC test built successfully!"; \
	elif [ -d $(BUILD_DIR)/$(APP_BUNDLE_NAME) ]; then \
		echo "XRC test app bundle built successfully!"; \
	else \
		echo "Build failed - executable not found"; \
		exit 1; \
	fi

# Create standalone app bundle (macOS)
bundle: build
	@if [ -d $(BUILD_DIR)/$(APP_BUNDLE_NAME) ]; then \
		echo "App bundle already created by CMake at $(BUILD_DIR)/$(APP_BUNDLE_NAME)"; \
		cp -r $(BUILD_DIR)/$(APP_BUNDLE_NAME) ./$(APP_BUNDLE_NAME); \
		echo "Copied to: ./$(APP_BUNDLE_NAME)"; \
	elif [ -f $(BUILD_DIR)/$(TARGET_EXEC) ]; then \
		echo "Creating app bundle manually..."; \
		rm -rf $(APP_BUNDLE_NAME); \
		mkdir -p $(APP_BUNDLE_NAME)/Contents/MacOS; \
		mkdir -p $(APP_BUNDLE_NAME)/Contents/Resources; \
		cp $(BUILD_DIR)/$(TARGET_EXEC) $(APP_BUNDLE_NAME)/Contents/MacOS/; \
		cp Info.plist $(APP_BUNDLE_NAME)/Contents/; \
		cp test_panel.xrc $(APP_BUNDLE_NAME)/Contents/Resources/; \
		echo "Bundle created: $(APP_BUNDLE_NAME)"; \
	else \
		echo "No executable found to bundle"; \
		exit 1; \
	fi

# Run the application
run: build
	@echo "Running XRC test..."
	@if [ -d $(BUILD_DIR)/$(APP_BUNDLE_NAME) ]; then \
		open $(BUILD_DIR)/$(APP_BUNDLE_NAME); \
	elif [ -f $(BUILD_DIR)/$(TARGET_EXEC) ]; then \
		cd $(BUILD_DIR) && ./$(TARGET_EXEC); \
	else \
		echo "No executable found to run"; \
		exit 1; \
	fi

# Clean build artifacts
clean:
	@echo "Cleaning build files..."
	rm -rf $(BUILD_DIR)
	rm -rf $(APP_BUNDLE_NAME)

# Show help
help:
	@echo "Available targets:"
	@echo "  all        - Configure and build (default)"
	@echo "  configure  - Configure build with CMake"
	@echo "  build      - Build the project"
	@echo "  bundle     - Create app bundle (macOS)"
	@echo "  run        - Run the application"
	@echo "  clean      - Remove build files"
	@echo "  help       - Show this help" 