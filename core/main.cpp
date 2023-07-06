#include <cstdlib>

extern "C" int addxx(int a, int b) {
   return (int)malloc(a + b);
}

