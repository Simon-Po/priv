
nasm -f elf64 $1.s
ld -o $1 $1.o
rm $1.o
.\/$1