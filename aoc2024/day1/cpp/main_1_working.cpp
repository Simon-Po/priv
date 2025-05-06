#include <algorithm>
#include <cstdint>
#include <cstdlib>
#include <ios>
#include <iostream>
#include <fstream>
#include <ostream>
#include <string>
#include <vector>

int main(void) {
  std::ifstream file("input.txt");
  std::vector<int> left;
  std::vector<int> right;
    
  if(!file) {
    std::cerr << "Error couldnt read file!" << std::endl;
    return 1; 
  }
  
  std::string line;

  std::string num1;
  std::string num2;
  while(std::getline(file,line)) {

  num1 = "";
    // sort lines into two vectors 
  while(line[0] != ' ') {
      num1 += line[0];
      line.erase(line.begin());
  }
  while(line[0] == ' ') {
      line.erase(0,1);
  }
    num2 = line;
  left.push_back(std::stoi(num1));  
  right.push_back(std::stoi(num2));  
  std::cout << "num1: " << num1 <<  std::endl;
  std::cout << "num2: " << num2 <<  std::endl;

  }
  file.close();
  
  std::sort(left.begin(),left.end());
  std::sort(right.begin(),right.end());
  uint64_t distance = 0; 
  for(int i = 0; i < left.size(); i++) {
     distance += std::abs(left.at(i) - right.at(i));
  }  
  
  std::cout << distance << std::endl;

}
