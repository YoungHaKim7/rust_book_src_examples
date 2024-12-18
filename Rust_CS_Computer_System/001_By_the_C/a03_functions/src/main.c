#include <stdio.h>

int max(int x, int y) {
  int bigger;

  bigger = x;
  if (y > x) {
    bigger = y;
  }
  printf("  in max, before return x: %d y: %d\n", x, y);
  return bigger;
}

int main(void) {

  int big_int = max(10, 3);
  printf("max fn : %d", big_int);

  return 0;
}
