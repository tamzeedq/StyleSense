#include <iostream>
#include <vector>
#include <string>

// Test file for StyleSense with intentional style issues
int main() {
    // Variable declaration with spacing issues
    int x=5;  // Missing spaces around '='
    int y = 10;  // Correct spacing
    
    // Conditional statements with spacing issues
    if(x>3){  // Missing spaces around '>' and after 'if'
        std::cout<<"Hello, StyleSense!"<<std::endl;  // Missing spaces around '<<'
    }
    
    // Loop with spacing issues
    for(int i=0; i<10; i++) {
        // Vector initialization with spacing issue
        std::vector<int> vec={1,2,3,4,5};
    }
    
    return 0;
}
