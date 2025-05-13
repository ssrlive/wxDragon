SHELL=/bin/bash # Explicitly set shell for $(shell ...) compatibility
CXX = g++
# IMPORTANT: Ensure WX_CONFIG points to the correct wx-config script.
# Example: /Users/allen/Documents/RustProjects/wxdragon/target/debug/build/wxdragon-sys-SOMEHASH/out/build/wxwidgets_build/wx-config
WX_CONFIG = /Users/allen/Documents/RustProjects/wxdragon/target/debug/build/wxdragon-sys-5668b26e81c632f2/out/build/wxwidgets_build/wx-config # Removed quotes

# Get compiler and linker flags from wx-config
CXXFLAGS_CMD = $(WX_CONFIG) --cxxflags --unicode=yes --debug=yes
LIBS_CMD = $(WX_CONFIG) --libs core,base,adv,aui --unicode=yes --debug=yes

CXXFLAGS := $(shell $(CXXFLAGS_CMD))
LIBS := $(shell $(LIBS_CMD))

$(info Building with WX_CONFIG: $(WX_CONFIG))
$(info CXXFLAGS_CMD: $(CXXFLAGS_CMD))
$(info Resolved CXXFLAGS: $(CXXFLAGS))
$(info LIBS_CMD: $(LIBS_CMD))
$(info Resolved LIBS: $(LIBS))

TARGET_EXEC = minimal_cpp_notif_test
APP_BUNDLE_NAME = $(TARGET_EXEC).app
APP_BUNDLE_CONTENTS = $(APP_BUNDLE_NAME)/Contents
APP_BUNDLE_MACOS = $(APP_BUNDLE_CONTENTS)/MacOS
APP_BUNDLE_RESOURCES = $(APP_BUNDLE_CONTENTS)/Resources

# Source files
SOURCES = main.cpp

# Default target is now the bundle
all: bundle

$(TARGET_EXEC): $(SOURCES)
	@echo "Compiling and linking $(TARGET_EXEC)..."
	$(CXX) $(SOURCES) -o $(TARGET_EXEC) $(CXXFLAGS) $(LIBS)
	@echo "Build finished."

# Bundle target depends on the executable
bundle: $(TARGET_EXEC)
	@echo "Creating application bundle $(APP_BUNDLE_NAME)..."
	@rm -rf $(APP_BUNDLE_NAME)
	@mkdir -p $(APP_BUNDLE_MACOS)
	@mkdir -p $(APP_BUNDLE_RESOURCES)
	@cp $(TARGET_EXEC) $(APP_BUNDLE_MACOS)/
	@cp Info.plist $(APP_BUNDLE_CONTENTS)/
	# @cp AppIcon.icns $(APP_BUNDLE_RESOURCES)/  # Optional: Uncomment if you create an AppIcon.icns
	@echo "Bundle created: $(APP_BUNDLE_NAME)"

clean:
	rm -f $(TARGET_EXEC) $(SOURCES:.cpp=.o)
	rm -rf $(APP_BUNDLE_NAME)

.PHONY: all bundle clean 