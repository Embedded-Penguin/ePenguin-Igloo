LDSCRIPT = ./ld/ex.ld
MCUTYPE=__SAMD21J18A__
print-%  : ; @echo $* = $($*)
BUILD_DIR:=build
SRC_DIR:=src
INC_DIR:=inc
BIN_DIR:=bin

VERBOSE:=0
_ATSYM_:=@

ifeq ($(VERBOSE),1)
_ATSYM_=
clean-build:
else
_ATSYM_=@
endif

# Tools
CC=arm-none-eabi-g++
LD=arm-none-eabi-gcc
AR=arm-none-eabi-ar
AS=arm-none-eabi-as

ELF=$(BIN_DIR)/main.elf

INC:=$(sort $(shell find -L . -type f,l \( -name '*.h' -o -name '*.hpp' \) -printf '-I%h ' | sort -u))

vpath %.c $(sort $(shell find -L ./src -type f,l -name "*.c" -printf '%h\n' | sort -u))
vpath %.s $(sort $(shell find -L ./src -type f,l -name "*.s" -printf '%h\n' | sort -u))
vpath %.cpp $(sort $(shell find -L ./src -type f,l -name "*.cpp" -printf '%h\n' | sort -u))
vpath %.o $(BUILD_DIR)

OBJ_C=$(shell find -L ./src -type f,l -name '*.c')
OBJ_CPP=$(shell find -L ./src -type f,l -name '*.cpp')
OBJ_S=$(shell find -L ./src -type f,l -name '*.s')


OBJ:=$(addprefix $(BUILD_DIR)/, $(notdir $(OBJ_C:%.c=%.o)))
OBJ+=$(addprefix $(BUILD_DIR)/, $(notdir $(OBJ_CPP:%.cpp=%.o)))
OBJ+=$(addprefix $(BUILD_DIR)/, $(notdir $(OBJ_S:%.s=%.o)))


DEPS:=$(OBJ:.o=.d)

LDFLAGS+= -T$(LDSCRIPT) -Wall -mthumb -mcpu=cortex-m0 -Wl,--gc-sections
CFLAGS+= $(INC) -mcpu=cortex-m0 -mthumb -ggdb -D$(MCUTYPE)

.PHONY: all clean-build clean

all: $(ELF)
$(ELF): $(OBJ)
	@mkdir -p $(dir $@)
	@echo "***************"
	@echo "Linking Target: " $@
	@echo "prereq: $(OBJ)"
	$(_ATSYM_)$(LD) $(LDFLAGS) -o $@ $(OBJ) $(LDLIBS)

# compile and generate dependency info
$(BUILD_DIR)/%.o: %.c
	@mkdir -p $(dir $@)
	@echo "***************"
	@echo "Compiling $< ---> $@"
	$(_ATSYM_)arm-none-eabi-gcc -c $(CFLAGS) $< -o $@
	@echo "Outputting Dependency from $< ---> "$(BUILD_DIR)/$*.d
	$(_ATSYM_)arm-none-eabi-gcc -MM $(CFLAGS) $< > $(BUILD_DIR)/$*.d

# compile and generate dependency info
$(BUILD_DIR)/%.o: %.cpp
	@mkdir -p $(dir $@)
	@echo "***************"
	@echo "Compiling $< ---> $@"
	$(_ATSYM_)$(CC) -c $(CFLAGS) $< -o $@
	@echo "Outputting Dependency from $< ---> "$(BUILD_DIR)/$*.d
	$(_ATSYM_)$(CC) -MM $(CFLAGS) $< > $(BUILD_DIR)/$*.d
$(BUILD_DIR)/%.o: %.s
	@mkdir -p $(dir $@)
	@echo "***************"
	@echo "Assembling $< ---> $@"
	$(_ATSYM_)$(AS) $< -o $@

clean:
	@echo "Cleaning project..."
	$(_ATSYM_)rm -f $(OBJ) $(DEPS)
	$(_ATSYM_)rm -f $(BUILD_DIR)/*.d $(BUILD_DIR)/*.o
	$(_ATSYM_)rm -f $(ELF) .gdb_history

clean-build: clean all

debug:  $(ELF)
	arm-none-eabi-gdb -iex "target extended-remote localhost:3333" $(ELF)

# pull in dependencies
-include $(DEPS)


