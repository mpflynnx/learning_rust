#include <stdio.h>
#include <stdlib.h> // Required for malloc and free
#include <string.h> // Required for strchr or strlen

// Acts like a Rust function when called with a heap allocated
void take_ownership(char *ptr_some_string){
    printf("\n--- Function print ---\n");
    printf("You entered: \"%s\"\n", ptr_some_string);
    printf("------------------\n");

    // equivalent to Rust drop() with heap allocated items
    free(ptr_some_string);
    ptr_some_string = NULL;
}


int main(void) {
    int requested_size = 16;

    // --- 1. mf Hardcoded required size
    // if (requested_size = 1 || requested_size <= 0) {
    //     printf("Invalid size entered. Exiting.\n");
    //     return 1;
    // }

    /*

    // AI
    // --- 1. Determine the required size ---
    printf("Enter the maximum number of characters you plan to type (e.g., 80): ");

    // Read the user's desired size. Handle basic error cases.
    if (scanf("%d", &requested_size) != 1 || requested_size <= 0) {
        printf("Invalid size entered. Exiting.\n");
        return 1;
    }

    // Clear the input buffer after reading the integer
    while (getchar() != '\n');

    */

    // Add 1 to the size to account for the mandatory null terminator ('\0')
    const int actual_size = requested_size + 1;

    // --- 2. Allocate memory on the heap using malloc ---
    // We allocate (actual_size) characters (bytes) on the heap.
    char *userInput = (char *)malloc(actual_size * sizeof(char));

    // --- Check for allocation failure (malloc returns NULL on error) ---
    if (userInput == NULL) {
        // Print a system error message and exit
        perror("Error: Failed to allocate memory on the heap");
        return 1;
    }

    printf("\nMemory allocated successfully: %d bytes.\n", actual_size);

    // --- 3. Read user input into the allocated space ---
    printf("Enter your string (max %d characters):\n", requested_size);

    // Use fgets, which is safer than scanf, to read the string
    if (fgets(userInput, actual_size, stdin) == NULL) {
        printf("Error reading input.\n");
        // Always free memory before returning on failure
        free(userInput);
        return 1;
    }

    // Optional: Remove the trailing newline character added by fgets
    size_t len = strlen(userInput);
    if (len > 0 && userInput[len - 1] == '\n') {
        userInput[len - 1] = '\0';
    }


    // --- 4. Print out the user input ---
    printf("\n--- User Input before call to take_ownership ---\n");
    printf("You entered: \"%s\"\n", userInput);
    printf("------------------\n");

    take_ownership(userInput);

    if (userInput == NULL) {
        // Print a system error message and exit
        perror("Error: Failed to allocate memory on the heap");
        return 1;
    }

    // --- 4. Print out the user input ---
    // userInput heap memory freed, ptr set to NULL
    printf("\n--- User Input after call to take_ownership ---\n");
    printf("You entered: \"%s\"\n", userInput);
    printf("------------------\n");

    return 0; // Success
}
