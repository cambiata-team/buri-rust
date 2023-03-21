import {
    Bcompile
} from "@buri/mjolnirjs/main.mjs"
import path from "path"
import fs from "fs"

function getSourcePath() {
    let sourcePath = process.argv[2]
    if (sourcePath == undefined) {
        console.log("No source path provided.")
        process.exit(1)
    }
    return sourcePath
}

function canonicalizePath(inputPath) {
    return path.normalize(path.resolve(inputPath))
}

function getOutputPathFromCanonicalSourcePath(inputPath) {
    if (!inputPath.endsWith(".buri")) {
        console.log("Source path must end with \".buri\".")
        process.exit(1)
    }
    let cwd = process.cwd()
    let relativeInputPath = path.relative(cwd, inputPath)
    return path.join(cwd, ".buri", "dist", relativeInputPath).slice(0, -4) + "mjs"
}

function validateCompilerResult(compilerResult) {
    if (!Array.isArray(compilerResult)) {
        console.log("Malformed Compiler Result. Compiler did not return an array.")
        process.exit(1)
    }
    if (compilerResult.length !== 2) {
        console.log("Malformed Compiler Result. Compiler did not return an array with length 2.")
        process.exit(1)
    }
    if (typeof compilerResult[0] !== "string") {
        console.log("Malformed Compiler Result. Compiler did not return a string as the first element of the array.")
        process.exit(1)
    }
    if (typeof compilerResult[1] !== "string") {
        console.log("Malformed Compiler Result. Compiler did not return a string as the second element of the array.")
        process.exit(1)
    }
    if (compilerResult[0] !== "ok" && compilerResult[0] !== "error") {
        console.log("Malformed Compiler Result. Compiler did not return \"ok\" or \"error\" as the first element of the array.")
        process.exit(1)
    }
    return compilerResult
}

function main() {
    let canonicalSourcePath = canonicalizePath(getSourcePath())
    let outputPath = getOutputPathFromCanonicalSourcePath(canonicalSourcePath)
    let source = fs.readFileSync(canonicalSourcePath, "utf8")
    let compilerResult = validateCompilerResult(Bcompile(source))
    fs.mkdirSync(path.dirname(outputPath), { recursive: true })
    fs.writeFileSync(outputPath, compilerResult[1], "utf8")
}

main()
