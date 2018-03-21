#define GPIO_BASE (0x3F000000 + 0x200000)

volatile unsigned *GPIO_FSEL1 = (volatile unsigned *)(GPIO_BASE + 0x04);
volatile unsigned *GPIO_SET0  = (volatile unsigned *)(GPIO_BASE + 0x1C);
volatile unsigned *GPIO_CLR0  = (volatile unsigned *)(GPIO_BASE + 0x28);

static void spin_sleep_us(unsigned int us) {
  for (unsigned int i = 0; i < us * 6; i++) {
    asm volatile("nop");
  }
}

static void spin_sleep_ms(unsigned int ms) {
  spin_sleep_us(ms * 1000);
}

int main(void) {
  const unsigned int gpio_16 = 16;

  // Set GPIO 16 as output
  // bits 20-18 set to 001 = GPIO Pin 16 is an output
  //   0b0000'0100'0000'0000'0000'0000 = 0x40000
  //        └20  └16  └12  └8   └4   └0
  *GPIO_FSEL1 += 0b001 << 18;

  // STEP 2: Continuously set and clear GPIO 16.
  while(1) {
    *GPIO_SET0 = 0b1 << gpio_16;
    spin_sleep_ms(2 * 1000);

    *GPIO_CLR0 = 0b1 << gpio_16;
    spin_sleep_ms(2 * 1000);
  }

  return 0;
}
