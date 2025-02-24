#Useful tutorial https://makefiletutorial.com/

CC = gcc
CFLAGS = -g

#Build all but requisite first hworld
all: hworld
	@echo "Build $@: $?"

hworld: hworld.c
	@echo "Build $@" 
	#Build target that is hworld
#	$(CC) $(CPPFLAGS) $(CFLAGS) hworld.c -o hworld #equivalent to below
	$(CC) $(CPPFLAGS) $(CFLAGS) $^ -o $@

clean:
	rm -f hworld
