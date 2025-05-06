#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>

typedef struct {
  uint8_t *start;
  uint8_t *current;
  int32_t size;
  int32_t used;
} Arena;


typedef struct {
  int32_t len;
  int *arr;
} Int32_t_Array;


Arena *initArena(size_t arena_size) {

  Arena *arena = (Arena *)malloc(sizeof(Arena));
  if (!arena){
    fprintf(stderr, "Failure to allocate arena");
    exit(EXIT_FAILURE);
  }

  arena->start = (uint8_t *)malloc(arena_size);
  if (!arena->start) {
    fprintf(stderr, "Failure to allocate arena content");
    exit(EXIT_FAILURE);
  }

  arena->current = arena->start;
  arena->size = arena_size;
  arena->used = 0;

  return arena;
}

uint8_t* arena_alloc(Arena* arena,uint32_t aloc_size) {
    // check if enough space

    uint8_t* r = arena->current;
    arena->current += aloc_size;
    arena->used += aloc_size;
    return r;
}
void arena_free(Arena* arena){
  free(arena);
  (void)arena;
}
int32_t *makeInt32Arr(int32_t n,Arena* arena) {
  int32_t *arr = (int32_t *)arena_alloc(arena,n * sizeof(int32_t));
  if (!arr) {
    fprintf(stderr, "Failure to create int32Array");
    exit(EXIT_FAILURE);
  }
  return arr;
}
int main(void) {

  Arena* arena = initArena(1024);


  printf("Arena initialized with size of %d bytes\n", arena->size);
  printf("Arena position is %p\n",arena->current);
  
  Int32_t_Array arr = {
    .len =  1,
    .arr = makeInt32Arr(1,  arena)

  };
 (void)arr;
  printf("Arena position is %p\n",arena->current);
  printf("Arena used size: %d\n",arena->used);

  arena_free(arena);

  printf("Arena used size: %d\n",arena->used);
  return 0;
}
