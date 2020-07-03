#!/bin/bash

# links cmsis folder into our inc folder
# ln -s /storage/Shared/Documents/ASF_LIBS/xdk-asf-3.48.0/thirdparty/CMSIS/Include ./inc/cmsis
ln -s ${EPSF_HAL}/arch/common/inc/cmsis/ ./inc/cmsis
# links asf3 samd21 include folder into our inc folder
# ln -s /storage/Shared/Documents/ASF_LIBS/xdk-asf-3.48.0/sam0/utils/cmsis/samd21/include ./inc/asf3-inc
ln -s ${EPSF_HAL}/arch/arm/SAM_D2X/hal/inc ./inc/hal
ln -s ${EPSF_HAL}/arch/arm/SAM_D2X/hdi/inc ./inc/hdi
ln -s ${EPSF_HAL}/arch/arm/SAM_D2X/hal/src ./src/hal
ln -s ${EPSF_HAL}/arch/arm/SAM_D2X/hdi/src ./src/hdi
ln -s ${EPSF_HAL}/arch/arm/common/inc ./inc/common
ln -s ${EPSF_HAL}/arch/arm/common/src ./src/common
# links linker script into our linker folder 
# ln -s /storage/Shared/Documents/ASF_LIBS/xdk-asf-3.48.0/sam0/utils/linker_scripts/samd21/gcc/samd21j18a_flash.ld ./linker/.
ln -s ${EPSF_HAL}/arch/arm/SAM_D2X/ld/ex.ld ./ld/ex.ld
# links asf3 startup code for d21 into our src folder
# ln -s /storage/Shared/Documents/ASF_LIBS/xdk-asf-3.48.0/sam0/utils/cmsis/samd21/source/gcc/startup_samd21.c ./src/.
# ln -s /storage/Shared/Documents/ASF_LIBS/xdk-asf-3.48.0/sam0/utils/cmsis/samd21/source/system_samd21.c ./src/.
# ln -s /storage/Shared/Documents/ASF_LIBS/xdk-asf-3.48.0/sam0/utils/cmsis/samd21/source/system_samd21.h ./inc/.
