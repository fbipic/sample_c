
ifndef CPPUTEST_HOME
	CPPUTEST_HOME = /home/fpic/Perforce/cpputest
endif

CXX = g++
CPPFLAGS = -Wall -Wextra -g
CPPFLAGS += -I$(CPPUTEST_HOME)/include
LD_LIBRARIES = -L$(CPPUTEST_HOME)/cpputest_build/lib -lCppUTest -lCppUTestExt
#LD_LIBRARIES = -L$(CPPUTEST_HOME)/lib -lCppUTest -lCppUTestExt

all: ut_hworld
#	$(CXX) $(CPPFLAGS) -o runtest ut_main.cpp ut_hworld.cpp $(LD_LIBRARIES)

ut_hworld: ut_hworld.cpp ut_main.cpp
	$(CXX) $(CPPFLAGS) -o runtest $^ $(LD_LIBRARIES)

clean:
	rm -f runtest
