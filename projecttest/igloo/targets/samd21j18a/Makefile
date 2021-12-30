# ePenguin Generated Variables
PROJECT_NAME=projecttest
TARGET_NAME=samd21j18a

TOOLCHAIN=arm-none-eabi
ELF_FLAGS= \
-Wl,--start-group -l m \
-Wl,--end-group -mthumb \
-Wl,-Map=$(QUOTE)$(PROJECT_NAME).map$(QUOTE) \
--specs=nano.specs \
-Wl,--gc-sections \
-T$(QUOTE)$(LD_SCRIPT)$(QUOTE) \
-L$(QUOTE)$(LD_PATH)$(QUOTE)

QUOTE:="
