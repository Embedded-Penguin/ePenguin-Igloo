#include "epenguin.hpp"
#include "hal_sam_d2x.hpp"
#include <inttypes.h>


#define PORT_ADDR (0x41004400)

#define PORT_GROUP_SIZE (0x80)

#define PORT_A_OFF (0x00)
#define PORT_B_OFF (0x80)

#define PORT_DIR_OFF (0x00)
#define PORT_OUT_OFF 		(0x10)

// LED 0: PA09
// LED 1: PB01

#define LED0_PORT		  	(0)
#define LED0_PIN			(9)

#define LED1_PORT               (1)
#define LED1_PIN    			(1)

static void init_pin(int port, int pin);
static void set_pin(int port, int pin);
static void clr_pin(int port, int pin);

static void delay(int n)
{
    int i;
    
    for(;n>0;n--)
    {
        for(i=0;i<100;i++)
        {

            asm volatile("nop");
        }
    }
}



int main()
{
    init_pin(LED0_PORT, LED0_PIN);
    init_pin(LED1_PORT, LED1_PIN);
    for(;;)
    {
        set_pin(LED0_PORT, LED0_PIN);
        clr_pin(LED1_PORT, LED1_PIN);
        delay(500);
        clr_pin(LED0_PORT, LED0_PIN);
        set_pin(LED1_PORT, LED1_PIN);
        delay(500);
    }
    return 0;
}

// port 0: A
// port 1: B


void init_pin(int port, int pin)
{
    uint32_t* dir_reg = (uint32_t*)((PORT_ADDR | (port * PORT_GROUP_SIZE) | PORT_DIR_OFF));
    *dir_reg |= (1 << pin);
}

void set_pin(int port, int pin)
{
    uint32_t* out_reg = (uint32_t*)((PORT_ADDR | (port * PORT_GROUP_SIZE) | PORT_OUT_OFF));
    *out_reg |= (1 << pin);
}

void clr_pin(int port, int pin)
{
    uint32_t* out_reg = (uint32_t*)((PORT_ADDR | (port * PORT_GROUP_SIZE) | PORT_OUT_OFF));
    *out_reg &= ~(1 << pin);
}





