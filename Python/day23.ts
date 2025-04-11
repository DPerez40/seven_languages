class FileNotFoundError extends Error {
    constructor(message: string) {
        super(message);
        this.name = "FileNotFoundError";
    }
}

async function readFileFake(path: string): Promise<string> {
    if (path !== "books.txt") {
        throw new FileNotFoundError(`File not found: ${path}`);
    }
    return "Dune/n1984/nThe Fountainhead";
}

async function start() {
    try{
        const contents = await readFileFake("not_real.txt");
        console.log("File contents:\n" + contents);
    } catch (error) {
        if (error instanceof FileNotFoundError) {
            console.error("Custom Caught (Async):", error.message);
        } else {
            console.error("Generic Async Error:", (error as Error).message);
        }
    }
    
}

start();

