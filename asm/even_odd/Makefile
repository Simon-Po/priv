all: main
	./main	

main: main.o
	ld main.o -o main


main.o: clean
	nasm -f elf64 main.asm -o main.o

clean: 
	-rm main.o main
