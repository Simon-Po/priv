#include <stdio.h>
#include <unistd.h>
#include <sys/types.h>

typedef struct
{
    char *buffer;
    size_t buffer_length;
    ssize_t input_length;
} InputBuffer;

InputBuffer* create_input_buffer() {
    InputBuffer* input_buffer = (InputBuffer *)malloc(sizeof(InputBuffer));
    input_buffer->buffer = NULL;
    input_buffer->buffer_length = 0;
    input_buffer->input_length = 0;
}

void print_prompt() {
    printf("db >");
}

int main(int argc, char* argv[])  {
    InputBuffer* input_buffer = create_input_buffer();

    return 0;
}