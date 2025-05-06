all: build
	./main
build:
	gcc -fsanitize-recover=address -g -Wall -Werror -o main main.c 	
