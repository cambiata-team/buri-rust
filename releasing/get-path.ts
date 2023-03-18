import { join } from "path"

// Exported for testing.
export const getPathBuilder = (argv: string[]) => {
    // Check if the workspacePath argument was provided. If not, that means this
    // is being run locally and not from GitHub actions. In that case, we'll
    // assume the script is run from the workspace root.
    if (argv.length >= 3) {
        return (path: string) => join(argv[2], path)
    }
    return (path: string) => path
}

export const getPath = getPathBuilder(process.argv)
