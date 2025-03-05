/***************************************
 *
 * Copyright 2025 Fabio Piccolo
 *
 ***************************************/

// printf
#include <stdio.h>
// malloc, memcpy
#include <stdlib.h>
// memset
#include <string.h>

#define HELLO "Hello \"C\" world!"

static char *hello_str(void) {
  char *p_str = NULL;
  p_str = malloc(sizeof(HELLO) + 1);
  if (p_str == NULL) {
    return p_str;
  }
  memset(p_str, 0x00, sizeof(HELLO));
  memcpy(p_str, HELLO, sizeof(HELLO));
  return p_str;
}

int main(void) {
  char *p_hw = NULL;
  printf("Hello world!\n");
  p_hw = hello_str();
  if (p_hw != NULL) {
    printf("%s\n", p_hw);
    free(p_hw);
  }
  return 0;
}
