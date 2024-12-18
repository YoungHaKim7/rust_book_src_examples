#include <stdio.h>

int main(void) {
  int num = 42;
  int guess = 0;

  while (guess != num) {
    printf("%d is not the right number\n", guess);
    printf("Enter aother guess : ");
    scanf("%d", &guess);
  }
  printf("Congratulations! Yu guessed the number : %d", num);

  return 0;
}
