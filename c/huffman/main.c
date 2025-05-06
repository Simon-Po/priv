#include "stdio.h"
#include "string.h"
#include "stdlib.h"
#include <string.h>


struct Node {
  char letter;
  int freq;
  struct Node* left;
  struct Node* right;
};

typedef struct Heap {
  struct Node *NodeArr;
  int size;
  int capacity;
} heap;

heap* createHeap(int capacity, struct Node* arr);
void insertHelper(heap* h, int index);
void heapify(heap* h, int index);
int extractMin(heap* h);
void insert(heap* h, struct Node data);

heap* createHeap(int capacity, struct Node* arr) {
  heap* h = (heap*)malloc(sizeof(heap));

  if(h == NULL) {
    printf("Memory error in createHeap memory could not be allocated");
    return NULL;
  }
  h->size = 0;
  h->capacity = capacity;
  h->NodeArr = (struct Node*)malloc(sizeof(struct Node));


  if(h->NodeArr == NULL) {
    printf("Memory error in createHeap memory could not be allocated to NodeArr");
    return NULL; 
  }
  int i; 
  for( i = 0; i < capacity; i ++){
    h->NodeArr[i] = arr[i];
  }
  
  h->size = i;
  while(i >= 0) {
    heapify(h, i);
    i--;
  }
  return h;
}




struct Node makeNode(char ch) {
  struct Node newNode = {
    ch,
    1
  };
  return newNode;
}

struct Node* makeNodeArr(char* input, struct Node nodes[]) {
int lengthOfString = strlen(input);
  int nextpos = 0;
  char found[lengthOfString];  

  for(int i = 0; i < lengthOfString; i++) {
    if(strchr(found,input[i]) == NULL) {
        nodes[nextpos++] = makeNode(input[i]);
        found[strlen(found)] = input[i];
    }else {
        for(int x = 0; x < nextpos; x++) {
          if(nodes[x].letter == input[i]) {
            nodes[x].freq++;
          }
        }
      }
    
  }
  for(int i = 0; i < nextpos; i++) {
  printf("freq: %d \nchar: %c\n\n",nodes[i].freq,nodes[i].letter);
  }
  return nodes;
}



int main (void) {

  char* input = "aaaaabbbbbbwwwwwkkkkwkwk";
  struct Node nodes[strlen(input)];
  makeNodeArr(input,nodes);

}






