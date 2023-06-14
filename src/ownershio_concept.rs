/**
 * Memory is managed through the ownership system (rules checked at compile time)
 * 
 * Two part memory: Stack & Heap
 * 
 * Stack: - Store values in a last in first out format
 *        - Data on the stack must have a defined fixed size
 * 
 * Heap: - A certain amount of space is requested when putting data on heap
 *       - The OS finds the available space and returns an address for that space
 *         called the pointer (reference to that space in memory)
 * 
 * RULES
 * 1. Each value has a variable - called "owner"
 * 2. Only ONE owner can exists for that value at a time
 * 3. When owner goes out-of-scope, the value disappears 
 *    (if compiler determines if we no longer using the value, then it gets deleted and memory is freed)
 *    eg:
 *        let v1 = 69;
 *        let v2 = v1; // owner v1 goes out-of-scope
 *        println!("{}", v1); // hits error on v1, "moved"
 *    to handle this, there are concepts such as ref (&), borrow value/data, or, (.clone), clone makes a copy so TWO instance of 
 *    value in memory exists
 */

// note: & = set reference, * = dereference