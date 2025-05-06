#include <array>
#include <iostream>
#include <stdexcept>
#include <unordered_map>
#include <vector>
  
template <typename T, size_t N>
void printArr(std::array<T,N>* arr)
{
  std::cout << "[";
  for(const auto& el : *arr)
  {
    std::cout << el << ",";
  }
  std::cout << "]";
}



std::array<int, 2> twoSum(const std::vector<int>& arr,int target)
{
  
  std::unordered_map<int, int> map;  
  for(int i = 0; i < arr.size(); i++)
  {
    int diff = target - arr[i];
    if(map.find(diff) != map.end())
    {
      return  {map[diff],i};
    }
    map[arr[i]]= i;

  }
  throw std::invalid_argument("No correct answer can be found with this arg");
}

void testTwoSum() {
      std::vector<std::pair<std::vector<int>, int>> testCases = {
        {{1, 2, 3, 4, 5}, 4},  // Expected: {0, 2}
        {{2, 7, 11, 15}, 9},   // Expected: {0, 1}
        {{3, 2, 4}, 6},        // Expected: {1, 2}
        {{-1, -2, -3, -4}, -6} // Expected: {1, 3}
    };

    for (const auto& [nums, target] : testCases) {
        auto result = twoSum(nums, target);
        for (auto r : result) {
            std::cout << r << " ";
        }
        std::cout << std::endl;
    }
}

int main(void) {
    
  testTwoSum(); 
  return 0;
} 
