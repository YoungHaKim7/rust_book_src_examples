#include <stdio.h>

void print_table(int start, int stop) {
  int i;

  for (i = start; i <= stop; i++) {
    printf("%d\t", i * i);
  }
  printf("\n");
}

int main(void) {

  print_table(20, 30);

  return 0;
}
