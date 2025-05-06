#include <stdio.h>
#include <sys/socket.h>


int main(void) {

  printf("hello world!\n");
  
  socket(AF_INET, SOCK_STREAM,0);

  return 0;


}
