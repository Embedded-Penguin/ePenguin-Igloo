# Generated Variables
PROJECT_NAME=testdir
CC=arm-none-eabi-gcc
CCX=arm-none-eabi-g++
OBJCOPY=arm-none-eabi-objcopy
OBJDUMP=arm-none-eabi-objdump
SIZE=arm-none-eabi-size
GDB=arm-none-eabi-gdb
AS=arm-none-eabi-as

MCPU=cortex-m0plus
MCU=__SAMD21J18A__

LD_PATH=../ld
LD_SCRIPT=$(LD_PATH)/flash.ld

# Generated Flags
CFLAGS=-x c \
-DDEBUG \
-Os \
-ffunction-sections \
-mlong-calls \
-g3 \
-Wall \
-std=gnu99 \
-D$(MCU) \
-mcpu=$(MCPU) \
$(DIR_INCLUDES) \
-MD -MP \
-MF$(QUOTE)$(@:%.o=%.d)$(QUOTE) \
-MT$(QUOTE)$(@:%.o=%.d)$(QUOTE) \
-MT$(QUOTE)$(@:%.o=%.o)$(QUOTE)

ELF_FLAGS=-Wl,--start-group -l m -Wl,--end-group -mthumb \
-Wl,-Map=$(QUOTE)$(PROJECT_NAME).map$(QUOTE) --specs=nano.specs -Wl,--gc-sections -mcpu=$(MCPU) \
-T$(QUOTE)$(LD_SCRIPT)$(QUOTE) \
-L$(QUOTE)$(LD_PATH)$(QUOTE)

HEX_FLAGS=-R .eeprom \
-R .fuse \
-R .lock \
-R .signature

EEP_FLAGS=-j .eeprom --set-section-flags=.eeprom=alloc,load --change-section-lma \
.eeprom=0 --no-change-warnings

ifdef SystemRoot
	SHELL = cmd.exe
	MK_DIR = mkdir
else
	ifeq ($(shell uname), Linux)
		MK_DIR = mkdir -p
	endif

	ifeq ($(shell uname | cut -d _ -f 1), CYGWIN)
		MK_DIR = mkdir -p
	endif

	ifeq ($(shell uname | cut -d _ -f 1), MINGW32)
		MK_DIR = mkdir -p
	endif

	ifeq ($(shell uname | cut -d _ -f 1), MINGW64)
		MK_DIR = mkdir -p
	endif

	ifeq ($(shell uname | cut -d _ -f 1), DARWIN)
		MK_DIR = mkdir -p
	endif
endif

# List the subdirectories for creating object files
SUB_DIRS=
# List the object files

OBJS_AS_ARGS +=  \

# List the directories containing header files
DIR_INCLUDES +=  \

# List the dependency files
DEPS := $(OBJS:%.o=%.d)

DEPS_AS_ARGS +=  \

vpath %.c ../
vpath %.s ../
vpath %.S ../
# All Target
all: $(SUB_DIRS) $(PROJECT_NAME).elf \
$(PROJECT_NAME).bin \
$(PROJECT_NAME).hex \
$(PROJECT_NAME).eep \
$(PROJECT_NAME).lss

# Linker target
# Make ELF
$(PROJECT_NAME).elf: $(OBJS)
	@echo Building target: $@
	@echo Invoking: ARM/GNU Linker
	$(QUOTE)$(CC)$(QUOTE) -o $@ $(OBJS_AS_ARGS) $(ELF_FLAGS)

	@echo Finished building target: $@
	$(QUOTE)$(SIZE)$(QUOTE) $(QUOTE)$@$(QUOTE)

# Make BIN
$(PROJECT_NAME).bin: $(PROJECT_NAME).elf
	@echo Producing $@
	$(QUOTE)$(OBJCOPY)$(QUOTE) -O binary $(QUOTE)$<$(QUOTE) $(QUOTE)$@$(QUOTE)

# Make HEX
$(PROJECT_NAME).hex: $(PROJECT_NAME).elf
	@echo Producing $@
	$(QUOTE)$(OBJCOPY)$(QUOTE) -O ihex $(HEX_FLAGS) $(QUOTE)$<$(QUOTE) $(QUOTE)$@$(QUOTE)

# Make EEP
$(PROJECT_NAME).eep: $(PROJECT_NAME).elf
	@echo Producing $@
	$(QUOTE)$(OBJCOPY)$(QUOTE) $(EEP_FLAGS) -O binary $(QUOTE)$<$(QUOTE) \
	$(QUOTE)$@$(QUOTE) || exit 0

# Make LSS
$(PROJECT_NAME).lss: $(PROJECT_NAME).elf
	$(QUOTE)$(OBJDUMP)$(QUOTE) -h -S $(QUOTE)$<$(QUOTE)


# Compiler targets
%.o: %.c
	@echo Building file: $<
	@echo ARM/GNU C Compiler
	$(QUOTE)$(CC)$(QUOTE) $(CFLAGS) -o $(QUOTE)$@$(QUOTE) $(QUOTE)$<$(QUOTE)
	@echo Finished building: $<

%.o: %.s
	@echo Building file: $<
	@echo ARM/GNU Assembler
	$(QUOTE)$(AS)$(QUOTE) $(CFLAGS) -o $(QUOTE)$@$(QUOTE) $(QUOTE)$<$(QUOTE)
	@echo Finished building: $<

%.o: %.S
	@echo Building file: $<
	@echo ARM/GNU Preprocessing Assembler
	$(QUOTE)$(CC)$(QUOTE) $(CFLAGS) -o $(QUOTE)$@$(QUOTE) $(QUOTE)$<$(QUOTE)
	@echo Finished building: $<

# Detect changes in the dependent files and recompile the respective object files.
ifneq ($(MAKECMDGOALS),clean)
ifneq ($(strip $(DEPS)),)
-include $(DEPS)
endif
endif

$(SUB_DIRS):
	$(MK_DIR) "$@"

clean:
	rm -f $(OBJS_AS_ARGS)
	rm -f $(OUTPUT_FILE_PATH)
	rm -f $(DEPS_AS_ARGS)
	rm -f $(OUTPUT_FILE_NAME).a $(OUTPUT_FILE_NAME).hex $(OUTPUT_FILE_NAME).bin \
        $(OUTPUT_FILE_NAME).lss $(OUTPUT_FILE_NAME).eep $(OUTPUT_FILE_NAME).map \
        $(OUTPUT_FILE_NAME).srec

debug: $(OUTPUT_FILE_NAME).elf
	$(QUOTE)arm-none-eabi-gdb$(QUOTE) -iex $(QUOTE)target extended-remote localhost:3333$(QUOTE) $(OUTPUT_FILE_NAME).elf
QUOTE := "
