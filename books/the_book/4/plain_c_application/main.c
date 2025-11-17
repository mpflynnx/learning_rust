#include <stdio.h>
#include <stdlib.h> // Required for malloc and free
#include <string.h> // Required for strchr or strlen


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
    printf("\n--- User Input ---\n");
    printf("You entered: \"%s\"\n", userInput);
    printf("------------------\n");

    // --- Start problematic code here ---
    // shallow copy userInput
    // Both pointers now hold the exact same memory address.
    char *userInputSC = userInput;

    // Adding safe guards
    // Rust will invalidate userInput automatically 
    // but for C we will do it manually here
    userInput = NULL;

    // Modify userInputSC but also modifies userInput is still valid ! BAD!!!!
    // userInputSC = "efgh"; // This changes the pointer not the data

    // Correct way to change the data (if "efgh" fits in the same allocated space
    // as userInput)
    strcpy(userInputSC, "efgh");

    // --- 4. Print out the user input Shallow Copy ---
    printf("\n--- User Input SC ---\n");
    printf("You entered: \"%s\"\n", userInputSC);
    printf("------------------\n");

        // --- 4. Print out the user input ---
    printf("\n--- User Input ---\n");
    printf("You entered: \"%s\"\n", userInput);
    printf("------------------\n");

    // --- 5. Free the allocated memory ---
    // This returns the space back to the operating system/memory manager.
    // free(userInput);  // also frees userInputSC if userInput still valid
    free(userInputSC); // double free detected if both free lines active SIGABRT points to same heap memory 

        // --- 4. Print out the Invalid user input ---
    printf("\n--- User Input SC ---\n");
    printf("You entered: \"%s\"\n", userInputSC); // junk data as cleared
    printf("------------------\n");

    // Set the pointer to NULL after freeing is a good practice (defensive programming)
    // userInput = NULL; // original
    userInputSC = NULL; // needed if userInput = NULL run earlier

    printf("Memory successfully freed from the heap.\n");

    return 0; // Success
}
