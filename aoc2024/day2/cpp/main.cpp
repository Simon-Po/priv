#include <algorithm>
#include <cstdlib>
#include <ios>
#include <iostream>
#include <fstream>
#include <list>
#include <ostream>
#include <sstream>
#include <string>
#include <vector>


bool isSafe(const std::vector<int>& report) {
    if (report.size() < 2) {
        return true; // A single level or empty report is trivially safe
    }

    bool increasing = true, decreasing = true;

    for (size_t i = 1; i < report.size(); ++i) {
        int diff = report[i] - report[i - 1];

        // Check if the difference is out of the acceptable range
        if (std::abs(diff) < 1 || std::abs(diff) > 3) {
            return false;
        }

        // Check monotonicity
        if (diff > 0) {
            decreasing = false;
        } else if (diff < 0) {
            increasing = false;
        }
    }

    return increasing || decreasing;
}

// Main function to count safe reports
int countSafeReports(const std::vector<std::vector<int>>& reports) {
    int safeCount = 0;

    for (const auto& report : reports) {
        if (isSafe(report)) {
            ++safeCount; // Count safe as-is
        } else {
            bool safeWithRemoval = false;
            for (size_t i = 0; i < report.size(); ++i) {
                std::vector<int> modifiedReport = report;
                modifiedReport.erase(modifiedReport.begin() + i);
                if (isSafe(modifiedReport)) {
                    safeWithRemoval = true;
                    break;
                }
            }
            if (safeWithRemoval) {
                ++safeCount;
            }
        }
    }

    return safeCount;
}
int main(void) {
  std::ifstream file("input.txt");
    
  if(!file) {
    std::cerr << "Error couldnt read file!" << std::endl;
    return 1; 
  }
  
  std::string line;
  int saveCount = 0; 
 
  std::vector<std::vector<int>> reports;
  while(std::getline(file,line)) {
    std::string s = "";
    std::istringstream stringStream(line);
    std::vector<int> list = {};
    while(std::getline(stringStream, s, ' '))
    {
      list.push_back(std::stoi(s));
   
    }
    reports.push_back(list) ;

  }
  int result = countSafeReports(reports);

  std::cout << "Number: "<< result <<std::endl;
  return 0;
}

