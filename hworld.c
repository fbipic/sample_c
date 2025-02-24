
#include <stdio.h>
#include <stdlib.h> //malloc, memcpy
#include <string.h> //memset

#define HELLO		"Hello \"C\" world!"

char * hello_str()
{
  char *p_str = NULL;
  p_str=malloc(sizeof(HELLO)+1);
  if (p_str == NULL)
  {
    return p_str;
  }
  memset(p_str, 0x00, sizeof(HELLO));
  memcpy(p_str, HELLO, sizeof(HELLO));
  return p_str;
}


int main()
{
  char *p_hw = NULL; 
  printf("Hello world!\n");
  p_hw = hello_str();
  if (p_hw != NULL)
  {
    printf("%s\n", p_hw);
    free(p_hw);
  }
  return 0;
}
