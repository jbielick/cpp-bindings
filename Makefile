ifeq ($(OS), Windows_NT)
	DETECTED_OS = Windows
else
	UNAME_S := $(shell uname -s)
	ifeq ($(UNAME_S), Linux)
		DETECTED_OS = Linux
	else
		ifeq ($(UNAME_S), Darwin)
			DETECTED_OS = Darwin
		else
			DETECTED_OS = Unknown
		endif
	endif
endif

ifndef BUILD_ENV
	BUILD_ENV = debug
endif

ifeq ($(CARGO_BUILD_TARGET),)
	# default target
	RUST_TARGET_DIR = target
	RELEASE_FILE = lib-ruby-parser-unknown-unknown-unknown
else
	CARGOFLAGS += --target $(CARGO_BUILD_TARGET)
	RUST_TARGET_DIR = target/$(CARGO_BUILD_TARGET)
	RELEASE_FILE = lib-ruby-parser-$(CARGO_BUILD_TARGET)
endif

BINDINGS_DIR = lib-ruby-parser-cpp-bindings

ifeq ($(DETECTED_OS), Windows)
	CXXFLAGS += /std:c++17 /nologo /bigobj /MT /Zi
	CXXOBJFLAGS += /c /Fo
	ifeq ($(BUILD_ENV), debug)
		CXXFLAGS += /Od /DEBUG
	else
		CXXFLAGS += /O2
	endif

	OBJ_FILE_EXT = .obj
	STATIC_LIB_EXT = .lib
	EXEC_EXT = .exe

	CXX_SET_OUT_FILE = /link /OUT:
	LIST_DEPS = dumpbin /dependents
	RUST_OBJ_FILE = lib_ruby_parser_cpp_bindings.lib
endif

ifeq ($(DETECTED_OS), Linux)
	CXXFLAGS += -Wall -Wextra -std=c++17
	CXXOBJFLAGS += -fPIC -c
	CXXEXECFLAGS = -lpthread -ldl
	LDFLAGS = -r

	ifeq ($(BUILD_ENV), debug)
		CXXFLAGS += -g -O0
	else
		CXXFLAGS += -O2
	endif

	OBJ_FILE_EXT = .o
	STATIC_LIB_EXT = .a
	EXEC_EXT =

	CXX_SET_OUT_FILE = -o #
	LIST_DEPS = ldd
	RUST_OBJ_FILE = liblib_ruby_parser_cpp_bindings.a
endif

ifeq ($(DETECTED_OS), Darwin)
	CXXFLAGS += -Wall -Wextra -std=c++17
	CXXOBJFLAGS += -fPIC -c
	CXXEXECFLAGS = -lpthread -ldl
	LDFLAGS = -r

	ifeq ($(BUILD_ENV), debug)
		CXXFLAGS += -g -O0
	else
		CXXFLAGS += -O2
	endif
	OBJ_FILE_EXT = .o
	STATIC_LIB_EXT = .a
	EXEC_EXT =

	CXX_SET_OUT_FILE = -o #
	LIST_DEPS = otool -L
	RUST_OBJ_FILE = liblib_ruby_parser_cpp_bindings.a
endif


ifeq ($(BUILD_ENV), debug)
	RUST_ENV = debug
	TARGET_DIR = target/debug
	CARGOFLAGS +=
else
	RUST_ENV = release
	TARGET_DIR = target/release
	CARGOFLAGS += --release
endif

print-env:
	echo "BUILD_ENV = $(BUILD_ENV)"
	echo "CXXFLAGS = $(CXXFLAGS)"
	echo "RUST_ENV = $(RUST_ENV)"

setup:
	mkdir -p target/debug
	mkdir -p target/release

RUST_OBJ = $(TARGET_DIR)/lib-ruby-parser-rust-static$(STATIC_LIB_EXT)
$(RUST_OBJ):
	cd $(BINDINGS_DIR) && cargo build $(CARGOFLAGS)
	ls -l $(BINDINGS_DIR)/$(RUST_TARGET_DIR)/$(RUST_ENV)/
	cp $(BINDINGS_DIR)/$(RUST_TARGET_DIR)/$(RUST_ENV)/$(RUST_OBJ_FILE) $(RUST_OBJ)

rust-object: $(RUST_OBJ)
gen-headers: $(RUST_OBJ)

# headers

LIB_RUBY_PARSER_TMP_H = target/lib-ruby-parser-tmp.h
LIB_RUBY_PARSER_H = target/lib-ruby-parser.h
$(LIB_RUBY_PARSER_H):
	cat src/byte_ptr.h > $(LIB_RUBY_PARSER_TMP_H)
	cat src/comment_type.h >> $(LIB_RUBY_PARSER_TMP_H)
	cat src/error_level.h >> $(LIB_RUBY_PARSER_TMP_H)
	cat src/magic_comment_kind.h >> $(LIB_RUBY_PARSER_TMP_H)

	cat src/bytes.h >> $(LIB_RUBY_PARSER_TMP_H)
	cat src/input.h >> $(LIB_RUBY_PARSER_TMP_H)
	cat src/loc.h >> $(LIB_RUBY_PARSER_TMP_H)
	cat src/token.h >> $(LIB_RUBY_PARSER_TMP_H)

	cat src/gen/messages/classes.h >> $(LIB_RUBY_PARSER_TMP_H)
	cat src/gen/messages/variant.h >> $(LIB_RUBY_PARSER_TMP_H)
	cat src/message.h >> $(LIB_RUBY_PARSER_TMP_H)

	cat src/gen/nodes/classes.h >> $(LIB_RUBY_PARSER_TMP_H)
	cat src/gen/nodes/variant.h >> $(LIB_RUBY_PARSER_TMP_H)
	cat src/node.h >> $(LIB_RUBY_PARSER_TMP_H)

	cat src/comment.h >> $(LIB_RUBY_PARSER_TMP_H)
	cat src/custom_decoder.h >> $(LIB_RUBY_PARSER_TMP_H)
	cat src/diagnostic.h >> $(LIB_RUBY_PARSER_TMP_H)
	cat src/helpers.h >> $(LIB_RUBY_PARSER_TMP_H)
	cat src/magic_comment.h >> $(LIB_RUBY_PARSER_TMP_H)
	cat src/token_rewriter.h >> $(LIB_RUBY_PARSER_TMP_H)

	cat src/parser_options.h >> $(LIB_RUBY_PARSER_TMP_H)
	cat src/parser_result.h >> $(LIB_RUBY_PARSER_TMP_H)

	echo "#ifndef LIB_RUBY_PARSER_H" > $(LIB_RUBY_PARSER_H)
	echo "#define LIB_RUBY_PARSER_H" >> $(LIB_RUBY_PARSER_H)

	cat $(LIB_RUBY_PARSER_TMP_H) | \
		grep -v "#include \"" | \
		grep -v "#ifndef LIB_RUBY_PARSER_" | \
		grep -v "#define LIB_RUBY_PARSER_" | \
		grep -v "#endif // LIB_RUBY_PARSER_" \
		>> $(LIB_RUBY_PARSER_H)
	echo "#endif // LIB_RUBY_PARSER_H" >> $(LIB_RUBY_PARSER_H)

	rm -f $(LIB_RUBY_PARSER_TMP_H)

# objects
OBJECTS =

# gen/messages

$(TARGET_DIR)/gen/messages/classes$(OBJ_FILE_EXT): src/gen/messages/classes.h src/gen/messages/classes.cpp
	$(CXX) src/gen/messages/classes.cpp $(CXXFLAGS) $(CXXOBJFLAGS)
	mv classes$(OBJ_FILE_EXT) $(TARGET_DIR)/gen/messages/classes$(OBJ_FILE_EXT)
OBJECTS += $(TARGET_DIR)/gen/messages/classes$(OBJ_FILE_EXT)

$(TARGET_DIR)/gen/messages/make$(OBJ_FILE_EXT): src/gen/messages/make.h src/gen/messages/make.cpp
	$(CXX) src/gen/messages/make.cpp $(CXXFLAGS) $(CXXOBJFLAGS)
	mv make$(OBJ_FILE_EXT) $(TARGET_DIR)/gen/messages/make$(OBJ_FILE_EXT)
OBJECTS += $(TARGET_DIR)/gen/messages/make$(OBJ_FILE_EXT)

$(TARGET_DIR)/gen/messages/render$(OBJ_FILE_EXT): src/gen/messages/render.h src/gen/messages/render.cpp
	$(CXX) src/gen/messages/render.cpp $(CXXFLAGS) $(CXXOBJFLAGS)
	mv render$(OBJ_FILE_EXT) $(TARGET_DIR)/gen/messages/render$(OBJ_FILE_EXT)
OBJECTS += $(TARGET_DIR)/gen/messages/render$(OBJ_FILE_EXT)

# gen/nodes

$(TARGET_DIR)/gen/nodes/classes$(OBJ_FILE_EXT): src/gen/nodes/classes.h src/gen/nodes/classes.cpp
	$(CXX) src/gen/nodes/classes.cpp $(CXXFLAGS) $(CXXOBJFLAGS)
	mv classes$(OBJ_FILE_EXT) $(TARGET_DIR)/gen/nodes/classes$(OBJ_FILE_EXT)
OBJECTS += $(TARGET_DIR)/gen/nodes/classes$(OBJ_FILE_EXT)

$(TARGET_DIR)/gen/nodes/make$(OBJ_FILE_EXT): src/gen/nodes/make.h src/gen/nodes/make.cpp
	$(CXX) src/gen/nodes/make.cpp $(CXXFLAGS) $(CXXOBJFLAGS)
	mv make$(OBJ_FILE_EXT) $(TARGET_DIR)/gen/nodes/make$(OBJ_FILE_EXT)
OBJECTS += $(TARGET_DIR)/gen/nodes/make$(OBJ_FILE_EXT)

# non-gen

$(TARGET_DIR)/byte_ptr$(OBJ_FILE_EXT): src/byte_ptr.h src/byte_ptr.cpp
	$(CXX) src/byte_ptr.cpp $(CXXFLAGS) $(CXXOBJFLAGS)
	mv byte_ptr$(OBJ_FILE_EXT) $(TARGET_DIR)/byte_ptr$(OBJ_FILE_EXT)
OBJECTS += $(TARGET_DIR)/byte_ptr$(OBJ_FILE_EXT)

$(TARGET_DIR)/bytes$(OBJ_FILE_EXT): src/bytes.h src/bytes.cpp
	$(CXX) src/bytes.cpp $(CXXFLAGS) $(CXXOBJFLAGS)
	mv bytes$(OBJ_FILE_EXT) $(TARGET_DIR)/bytes$(OBJ_FILE_EXT)
OBJECTS += $(TARGET_DIR)/bytes$(OBJ_FILE_EXT)

$(TARGET_DIR)/comment$(OBJ_FILE_EXT): src/comment.h src/comment.cpp
	$(CXX) src/comment.cpp $(CXXFLAGS) $(CXXOBJFLAGS)
	mv comment$(OBJ_FILE_EXT) $(TARGET_DIR)/comment$(OBJ_FILE_EXT)
OBJECTS += $(TARGET_DIR)/comment$(OBJ_FILE_EXT)

$(TARGET_DIR)/custom_decoder$(OBJ_FILE_EXT): src/custom_decoder.h src/custom_decoder.cpp
	$(CXX) src/custom_decoder.cpp $(CXXFLAGS) $(CXXOBJFLAGS)
	mv custom_decoder$(OBJ_FILE_EXT) $(TARGET_DIR)/custom_decoder$(OBJ_FILE_EXT)
OBJECTS += $(TARGET_DIR)/custom_decoder$(OBJ_FILE_EXT)

$(TARGET_DIR)/diagnostic$(OBJ_FILE_EXT): src/diagnostic.h src/diagnostic.cpp
	$(CXX) src/diagnostic.cpp $(CXXFLAGS) $(CXXOBJFLAGS)
	mv diagnostic$(OBJ_FILE_EXT) $(TARGET_DIR)/diagnostic$(OBJ_FILE_EXT)
OBJECTS += $(TARGET_DIR)/diagnostic$(OBJ_FILE_EXT)

$(TARGET_DIR)/helpers$(OBJ_FILE_EXT): src/helpers.h src/helpers.cpp
	$(CXX) src/helpers.cpp $(CXXFLAGS) $(CXXOBJFLAGS)
	mv helpers$(OBJ_FILE_EXT) $(TARGET_DIR)/helpers$(OBJ_FILE_EXT)
OBJECTS += $(TARGET_DIR)/helpers$(OBJ_FILE_EXT)

$(TARGET_DIR)/low_level$(OBJ_FILE_EXT): src/low_level.h src/low_level.cpp
	$(CXX) src/low_level.cpp $(CXXFLAGS) $(CXXOBJFLAGS)
	mv low_level$(OBJ_FILE_EXT) $(TARGET_DIR)/low_level$(OBJ_FILE_EXT)
OBJECTS += $(TARGET_DIR)/low_level$(OBJ_FILE_EXT)

$(TARGET_DIR)/magic_comment$(OBJ_FILE_EXT): src/magic_comment.h src/magic_comment.cpp
	$(CXX) src/magic_comment.cpp $(CXXFLAGS) $(CXXOBJFLAGS)
	mv magic_comment$(OBJ_FILE_EXT) $(TARGET_DIR)/magic_comment$(OBJ_FILE_EXT)
OBJECTS += $(TARGET_DIR)/magic_comment$(OBJ_FILE_EXT)

$(TARGET_DIR)/parser_options$(OBJ_FILE_EXT): src/parser_options.h src/parser_options.cpp
	$(CXX) src/parser_options.cpp $(CXXFLAGS) $(CXXOBJFLAGS)
	mv parser_options$(OBJ_FILE_EXT) $(TARGET_DIR)/parser_options$(OBJ_FILE_EXT)
OBJECTS += $(TARGET_DIR)/parser_options$(OBJ_FILE_EXT)

$(TARGET_DIR)/parser_result$(OBJ_FILE_EXT): src/parser_result.h src/parser_result.cpp
	$(CXX) src/parser_result.cpp $(CXXFLAGS) $(CXXOBJFLAGS)
	mv parser_result$(OBJ_FILE_EXT) $(TARGET_DIR)/parser_result$(OBJ_FILE_EXT)
OBJECTS += $(TARGET_DIR)/parser_result$(OBJ_FILE_EXT)

$(TARGET_DIR)/loc$(OBJ_FILE_EXT): src/loc.h src/loc.cpp
	$(CXX) src/loc.cpp $(CXXFLAGS) $(CXXOBJFLAGS)
	mv loc$(OBJ_FILE_EXT) $(TARGET_DIR)/loc$(OBJ_FILE_EXT)
OBJECTS += $(TARGET_DIR)/loc$(OBJ_FILE_EXT)

$(TARGET_DIR)/token_rewriter$(OBJ_FILE_EXT): src/token_rewriter.h src/token_rewriter.cpp
	$(CXX) src/token_rewriter.cpp $(CXXFLAGS) $(CXXOBJFLAGS)
	mv token_rewriter$(OBJ_FILE_EXT) $(TARGET_DIR)/token_rewriter$(OBJ_FILE_EXT)
OBJECTS += $(TARGET_DIR)/token_rewriter$(OBJ_FILE_EXT)

$(TARGET_DIR)/token$(OBJ_FILE_EXT): src/token.h src/token.cpp
	$(CXX) src/token.cpp $(CXXFLAGS) $(CXXOBJFLAGS)
	mv token$(OBJ_FILE_EXT) $(TARGET_DIR)/token$(OBJ_FILE_EXT)
OBJECTS += $(TARGET_DIR)/token$(OBJ_FILE_EXT)

$(TARGET_DIR)/input$(OBJ_FILE_EXT): src/input.h src/input.cpp
	$(CXX) src/input.cpp $(CXXFLAGS) $(CXXOBJFLAGS)
	mv input$(OBJ_FILE_EXT) $(TARGET_DIR)/input$(OBJ_FILE_EXT)
OBJECTS += $(TARGET_DIR)/input$(OBJ_FILE_EXT)

TEST_O = $(TARGET_DIR)/test$(OBJ_FILE_EXT)
$(TEST_O): test.cpp $(LIB_RUBY_PARSER_H)
	$(CXX) test.cpp $(CXXFLAGS) $(CXXOBJFLAGS)
	mv test$(OBJ_FILE_EXT) $(TEST_O)

LIB_RUBY_PARSER_STATIC = $(TARGET_DIR)/lib-ruby-parser$(STATIC_LIB_EXT)

ifeq ($(DETECTED_OS), Windows)
	BUILD_STATIC = lib.exe ws2_32.lib advapi32.lib userenv.lib $(RUST_OBJ) $(OBJECTS) /OUT:$(LIB_RUBY_PARSER_STATIC)
else
	LIB_RUBY_PARSER_O = $(TARGET_DIR)/lib-ruby-parser$(OBJ_FILE_EXT)
	BUILD_STATIC = $(LD) -r $(RUST_OBJ) $(OBJECTS) -o $(LIB_RUBY_PARSER_O) && ar -rv $(LIB_RUBY_PARSER_STATIC) $(LIB_RUBY_PARSER_O)
endif

$(LIB_RUBY_PARSER_STATIC): $(RUST_OBJ) $(OBJECTS)
	$(BUILD_STATIC)

# tests

TEST_DEPS = $(LIB_RUBY_PARSER_STATIC) $(TEST_O)

$(TARGET_DIR)/test-runner: $(TEST_DEPS)
	$(CXX) $(TEST_O) $(LIB_RUBY_PARSER_STATIC) $(CXXFLAGS) $(CXXEXECFLAGS) $(CXX_SET_OUT_FILE)$(TARGET_DIR)/test-runner

test: $(TARGET_DIR)/test-runner
	$(TARGET_DIR)/test-runner

test-valgrind: $(TARGET_DIR)/test-runner
	valgrind --leak-check=full --error-exitcode=1 --num-callers=20 $(TARGET_DIR)/test-runner

clean:
	rm -rf target
	mkdir -p $(TARGET_DIR)
	mkdir -p $(TARGET_DIR)/gen/nodes
	mkdir -p $(TARGET_DIR)/gen/messages

test-cov:
	$(CXX) test.cpp $(RUST_OBJ) $(CXXFLAGS) $(CXXEXECFLAGS) -fprofile-instr-generate -fcoverage-mapping -o $(TARGET_DIR)/test
	LLVM_PROFILE_FILE="$(TARGET_DIR)/test.profraw" $(TARGET_DIR)/test
	llvm-profdata merge -sparse $(TARGET_DIR)/test.profraw -o $(TARGET_DIR)/test.profdata
	llvm-cov report $(TARGET_DIR)/test-runner -instr-profile=$(TARGET_DIR)/test.profdata
	# llvm-cov show $(TARGET_DIR)/test-runner -instr-profile=$(TARGET_DIR)/test.profdata

# releases

STATIC_RELEASE_LIB = $(TARGET_DIR)/$(RELEASE_FILE)$(STATIC_LIB_EXT)

$(STATIC_RELEASE_LIB): $(LIB_RUBY_PARSER_STATIC)
	cp $(LIB_RUBY_PARSER_STATIC) $(STATIC_RELEASE_LIB)

build-static: $(STATIC_RELEASE_LIB) $(TEST_O)
	# test
	$(CXX) $(TEST_O) $(STATIC_RELEASE_LIB) $(CXXFLAGS) $(CXXEXECFLAGS) $(CXX_SET_OUT_FILE)$(TARGET_DIR)/static-test-runner
	$(LIST_DEPS) $(TARGET_DIR)/static-test-runner
	$(TARGET_DIR)/static-test-runner
