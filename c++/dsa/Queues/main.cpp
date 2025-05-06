#include <algorithm>
#include <array>
#include <iostream>
#include <ostream>

class Queue {
public:
  Queue();
  ~Queue();
  void enque(int);
  int deque();
  void printQ();
private:
  std::array<int,10> qArr; 
  int qLength = 0;
  void swap(int,int);
};

Queue::Queue() {
  std::cout << "Hello from q" << std::endl;
}

Queue::~Queue() {
  std::cout << "bye from q" << std::endl;
}
void Queue::enque(int x) {
  qArr[qLength] = x;
  qLength++;
   
}
int Queue::deque()
{
  int out = *qArr.begin();
  qArr[0] = 0;
  for(int i = 0; i < qLength; i++) 
  { 
    this->swap(i, i+1);
  }
  return out;
}
void Queue::swap(int x, int y)
{
  int tmp = qArr[x];
  qArr[x] = qArr[y];
  qArr[y] = tmp;
}
void Queue::printQ() {

  std::cout << "[";
  for(int e : qArr)
  {
    std::cout << e << "," ;
  }
  std::cout << "]" << std::endl;
}
int main(void) {

  Queue* q = new Queue();
  q->printQ();
  q->enque(5);
  q->printQ();
  q->enque(6);
  q->printQ();
  std::cout << q->deque() << "\n";
  q->printQ();
}

