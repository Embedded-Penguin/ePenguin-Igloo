# ePenguin Generated Variables
PROJECT_NAME=sampleprj
TARGET_NAME=samd21j18a
TOOLCHAIN=${ESF_DIR}/toolchains/arm-none-eabi-toolchain/bin/arm-none-eabi
CC=${TOOLCHAIN}-gcc
CXX=${TOOLCHAIN}-g++
OBJCOPY=${TOOLCHAIN}-objcopy
OBJDUMP=${TOOLCHAIN}-objdump
GDB=${TOOLCHAIN}-gdb-py
SIZE=${TOOLCHAIN}-size
AS=${TOOLCHAIN}-as


MCPU=cortex-m0plus
MCU=__SAMD21J18A__
LD_PATH=../../../ESF/ld
LD_SCRIPT=$(LD_PATH)/samd21j18a_flash.ld


CFLAGS= \
-D$(MCU) \
-mcpu=$(MCPU) \
-x c \
-DDEBUG \
-Os \
-g3 \
-Wall \
-c \
-std=gnu99 \
$(DIR_INCLUDES) \
-MD -MP \
-MF$(QUOTE)$(@:%.o=%.d)$(QUOTE) \
-MT$(QUOTE)$(@:%.o=%.d)$(QUOTE) \
-MT$(QUOTE)$(@:%.o=%.o)$(QUOTE)

ELF_FLAGS= \
-Wl,--start-group -l m \
-Wl,--end-group -mthumb \
-Wl,-Map=$(QUOTE)$(PROJECT_NAME).map$(QUOTE) \
--specs=nano.specs \
-Wl,--gc-sections \
-T$(QUOTE)$(LD_SCRIPT)$(QUOTE) \
-L$(QUOTE)$(LD_PATH)$(QUOTE)

HEX_FLAGS= \
-R .eeprom \
-R .fuse \
-R .lock \
-R .signature

EEP_FLAGS= \
-j .eeprom --set-section-flags=.eeprom=alloc,load \
--change-section-lma \
.eeprom=0 \
--no-change-warnings


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
SUB_DIRS+= \
src \
ESF/mcu/src

OBJS+= \
ESF/mcu/src/startup_samd21j18a.o \
ESF/mcu/src/system_samd21j18a.o \
src/main.o

OBJS_AS_ARGS+= \
$(QUOTE)ESF/mcu/src/startup_samd21j18a.o$(QUOTE) \
$(QUOTE)ESF/mcu/src/system_samd21j18a.o$(QUOTE) \
$(QUOTE)src/main.o$(QUOTE)

DIR_INCLUDES+= \
-I$(QUOTE)../../../ESF/common/inc$(QUOTE) \
-I$(QUOTE)../../../ESF/common/inc/cmsis$(QUOTE) \
-I$(QUOTE)../../../ESF/mcu/inc$(QUOTE) \
-I$(QUOTE)../../../inc$(QUOTE)

DEPS:=$(OBJS:%.o=%.d)

DEPS_AS_ARGS:=$(OBJS_AS_ARGS:%.o=%.d)

vpath %.c ../../../
vpath %.s ../../../
vpath %.S ../../../

.PHONY: debug clean

all:\
$(SUB_DIRS)\
$(PROJECT_NAME).elf\
$(PROJECT_NAME).bin\
$(PROJECT_NAME).hex\
$(PROJECT_NAME).eep\
$(PROJECT_NAME).lss
	$(QUOTE)$(SIZE)$(QUOTE) $(QUOTE)$(PROJECT_NAME).elf$(QUOTE)


$(PROJECT_NAME).elf:\
$(OBJS)
	$(QUOTE)$(CC)$(QUOTE) -o $@ $(OBJS_AS_ARGS) $(ELF_FLAGS)


$(PROJECT_NAME).bin:\
$(PROJECT_NAME).elf
	$(QUOTE)$(OBJCOPY)$(QUOTE) -O binary $(QUOTE)$<$(QUOTE) $(QUOTE)$@$(QUOTE)


$(PROJECT_NAME).hex:\
$(PROJECT_NAME).elf
	$(QUOTE)$(OBJCOPY)$(QUOTE) -O ihex $(HEX_FLAGS) $(QUOTE)$<$(QUOTE) $(QUOTE)$@$(QUOTE)


$(PROJECT_NAME).eep:\
$(PROJECT_NAME).elf
	$(QUOTE)$(OBJCOPY)$(QUOTE) $(EEP_FLAGS) -O binary $(QUOTE)$<$(QUOTE) $(QUOTE)$@$(QUOTE) || exit 0


$(PROJECT_NAME).lss:\
$(PROJECT_NAME).elf
	$(QUOTE)$(OBJDUMP)$(QUOTE) -h -S $(QUOTE)$<$(QUOTE) > $(QUOTE)$@$(QUOTE)


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


$(SUB_DIRS):
	$(MK_DIR) $(QUOTE)$@$(QUOTE)

ifneq ($(MAKECMDGOALS),clean)
ifneq ($(strip $(DEPS)),)
-include $(DEPS)
endif
endif

clean:
	@rm -f $(PROJECT_NAME).a
	@rm -f $(PROJECT_NAME).lss
	@rm -f $(PROJECT_NAME).srec
	@rm -f $(PROJECT_NAME).map
	@rm -f $(PROJECT_NAME).eep
	@rm -f $(OBJS_AS_ARGS)
	@rm -f $(DEPS_AS_ARGS)
	@rm -f $(PROJECT_NAME).bin
	@rm -f $(PROJECT_NAME).elf
	@rm -f $(PROJECT_NAME).hex


debug:\
all
	@$(GDB) $(PROJECT_NAME).elf -x $(QUOTE)scripts/debug.gdb$(QUOTE)


push:\
all
	@echo $(QUOTE)$(QUOTE)
	@echo $(QUOTE)Uploading $(PROJECT_NAME).elf...$(QUOTE)
	@$(GDB) $(PROJECT_NAME).elf -x $(QUOTE)scripts/push.gdb$(QUOTE) >/dev/null
	@echo $(QUOTE)$(QUOTE)$(PROJECT_NAME).elf $(QUOTE) uploaded!$(QUOTE)
	@$(QUOTE)$(SIZE)$(QUOTE) $(QUOTE)$(PROJECT_NAME).elf$(QUOTE)


QUOTE:="
