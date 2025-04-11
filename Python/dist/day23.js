"use strict";
class FileNotFoundError extends Error {
    constructor(message) {
        super(message);
        this.name = "FileNotFoundError";
    }
}
async function readFileFake(path) {
    if (path !== "books.txt") {
        throw new FileNotFoundError(`File not found: ${path}`);
    }
    return "Dune/n1984/nThe Fountainhead";
}
async function start() {
    try {
        const contents = await readFileFake("not_real.txt");
        console.log("File contents:\n" + contents);
    }
    catch (error) {
        if (error instanceof FileNotFoundError) {
            console.error("Custom Caught (Async):", error.message);
        }
        else {
            console.error("Generic Async Error:", error.message);
        }
    }
}
start();
