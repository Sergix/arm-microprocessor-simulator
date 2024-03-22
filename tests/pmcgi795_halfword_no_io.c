// sergix_halfword.c

// expected behavior after running: bytes 0x2 and 0x6 in RAM should be 1

int main()
{
  asm("" : : : "r5"); // force use of stm for push

  // from https://stackoverflow.com/questions/50779537/difference-between-ldrsh-ldrh-strh-and-strsh

  // map every two bytes starting from 0x0 in RAM
  volatile short *a = (short*)0x0;

  volatile short *ptr = a;

  // RAM: 0x2|0x3 = 01 00
  a[1] = 1;

  // RAM: 0x6|0x7 = 05 00
  a[3] = 5;

  // RAM: 0x6|0x7 = 01 00
  *(ptr+3) = a[1];

  asm("swi 0x11");
}