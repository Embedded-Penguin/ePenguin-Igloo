#ifndef _EPENGUIN_HPP_
#define _EPENGUIN_HPP_



/* -- ePenguin --

   This master file dictates all supported hardware. It lists supported
   architectures, mcu families, and specific mcus. This software framework
   targets hardware at the mcu level--not at the board level. Board layers can
   be applied, but this framework targets the mcu and the mcu alone.

*/
#ifndef __UARCHITECTURES__
#define __UARCHITECTURES__
#define __UARCH_ARM__			(0)
#define __UARCH_ARM64__		(1)
#define __UARCH_AVR__			(2)
#define __UARCH_AVR32__		(3)
#define __UARCH_X86__			(4)
#define __UARCH_RISCV__		(5)
#endif

#ifndef __UFAMILIES__
#define __UFAMILIES__
/* Support ARM MCU Families */

/*  Microchip */
#define __SAM_D1X__			(0)
#define __SAM_C1X__			(1)
#define __SAM_D2X__			(2)
#define __SAM_E_D5X__			(3)
#define __SAM_C2X__			(4)
#define __SAM_L2X__			(5)

/* STMicroelectronics */

/* NXP */

#endif

#ifndef __UCONTROLLERS__
#define __UCONTROLLERS__

/* ARM  */
/* Microchip */
/* SAM_D2X */
/* D2X Series E */
#define __ATSAMD21E15A__ (0)
#define __ATSAMD21E15B__ (1)
#define __ATSAMD21E15BU__ (2)
#define __ATSAMD21E15L__ (3)
#define __ATSAMD21E16A__ (4)
#define __ATSAMD21E16B__ (5)
#define __ATSAMD21E16BU__ (6)
#define __ATSAMD21E16L__ (7)
#define __ATSAMD21E17A__ (8)
#define __ATSAMD21E17D__ (9)
#define __ATSAMD21E17DU__ (10)
#define __ATSAMD21E17L__ (11)
#define __ATSAMD21E18A__ (12)
/* D2X Series G */
#define __ATSAMD21G15A__ (13)
#define __ATSAMD21G15B__ (14)
#define __ATSAMD21G15L__ (15)
#define __ATSAMD21G16A__ (16)
#define __ATSAMD21G16B__ (17)
#define __ATSAMD21G16L__ (18)
#define __ATSAMD21G17A__ (19)
#define __ATSAMD21G17AU__ (20)
#define __ATSAMD21G17D__ (21)
#define __ATSAMD21G17L__ (22)
#define __ATSAMD21G18A__ (23)
#define __ATSAMD21G18AU__ (24)
/*  D2X Series J */
#define __ATSAMD21J15A__ (25)
#define __ATSAMD21J15B__ (26)
#define __ATSAMD21J16A__ (27)
#define __ATSAMD21J16B__ (28)
#define __ATSAMD21J17A__ (29)
#define __ATSAMD21J17D__ (30)
#define __ATSAMD21J18A__ (31)

#endif
#include "epenguin_conf.h"

#endif


