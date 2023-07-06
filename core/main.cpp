#include <cstdlib>
#include <string>

extern "C" int addxx(int a, int b) {
   std::string s = "some random"; 
   return s.length() + a + b;
}

