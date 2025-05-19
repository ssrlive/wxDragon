SHELL=/bin/bash # Explicitly set shell for $(shell ...) compatibility
CXX = g++
# IMPORTANT: Ensure WX_CONFIG points to the correct wx-config script.
# Example: /Users/allen/Documents/RustProjects/wxdragon/target/debug/build/wxdragon-sys-SOMEHASH/out/build/wxwidgets_build/wx-config
WX_CONFIG = /Users/allen/Documents/RustProjects/wxdragon/target/debug/build/wxdragon-sys-da17f4d21acae0e9/out/build/wxwidgets_build/wx-config # Removed quotes

# Get compiler and linker flags from wx-config
CXXFLAGS_CMD = $(WX_CONFIG) --cxxflags --unicode=yes --debug=yes
LIBS_CMD = $(WX_CONFIG) --libs core,base,adv --unicode=yes --debug=yes

CXXFLAGS := $(shell $(CXXFLAGS_CMD))
LIBS := $(shell $(LIBS_CMD))

$(info Building with WX_CONFIG: $(WX_CONFIG))
$(info CXXFLAGS_CMD: $(CXXFLAGS_CMD))
$(info Resolved CXXFLAGS: $(CXXFLAGS))
$(info LIBS_CMD: $(LIBS_CMD))
$(info Resolved LIBS: $(LIBS))

TARGET_EXEC = const_extractor

# Source files
SOURCES = main.cpp

# Default target is now the bundle
all: $(TARGET_EXEC)

$(TARGET_EXEC): $(SOURCES)
	@echo "Compiling and linking $(TARGET_EXEC)..."
	$(CXX) $(SOURCES) -o $(TARGET_EXEC) $(CXXFLAGS) $(LIBS)
	@echo "Build finished."

clean:
	rm -f $(TARGET_EXEC) $(SOURCES:.cpp=.o)

.PHONY: all clean 