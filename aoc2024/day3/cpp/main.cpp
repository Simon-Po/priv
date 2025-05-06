#include <stdio.h>
#include <cstdlib>
#include <ios>
#include <iostream>
#include <fstream>
#include <ostream>
#include <string>


std::ifstream readIntoString(const std::string &fileName) {
  std::ifstream file(fileName);
    
  if(!file) {
    std::cerr << "Error couldnt read file!" << std::endl;
    throw;
  }
  return std::move(file); 
}


int main(void) {
  
 std::string fileName = "input.txt";
  std::ifstream file = readIntoString(fileName);
  
  std::cout << "File  "<< fileName << " Read" << std::endl;
  std::string line;

  while(std::getline(file,line)) {

  }
  
  file.close();
   

}
