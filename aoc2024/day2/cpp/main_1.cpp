#include <algorithm>
#include <cstdint>
#include <cstdlib>
#include <ios>
#include <iostream>
#include <fstream>
#include <ostream>
#include <sstream>
#include <string>
#include <vector>

int main(void) {
  std::ifstream file("input.txt");
    
  if(!file) {
    std::cerr << "Error couldnt read file!" << std::endl;
    return 1; 
  }
  
  std::string line;
  int saveCount = 0; 
 

  while(std::getline(file,line)) {
    std::string s = "";
    std::istringstream stringStream(line);
    std::vector<int> list = {};
    while(std::getline(stringStream, s, ' '))
    {
      list.push_back(std::stoi(s));
    }
    bool isUnsafe = false;
    bool increasing;
    if(list[0] > list[1]) {
      increasing = false;
    } else {
      increasing = true;
    }
    for(int i = 1; i<list.size(); i++) {
     std::cout << isUnsafe << " ";
     std::cout << "\t";
      if(std::abs(list[i] - list[i-1]) > 3 ||  std::abs(list[i] - list[i-1]) < 1) isUnsafe = true;
     std::cout << isUnsafe << " ";
     if(increasing && list[i-1] > list[i]) isUnsafe = true;
     std::cout << isUnsafe << " ";
      if(!increasing && list[i-1] < list[i]) isUnsafe = true;
    } 

    if(!isUnsafe) {
      saveCount++;
    }
    std::cout << "Is Unsafe?: " << isUnsafe << std::endl;
  }
  std::cout << saveCount;
}
