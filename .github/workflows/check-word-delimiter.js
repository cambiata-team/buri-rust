import { realpathSync, readdirSync, lstatSync } from "fs"
import { join } from "path"

/// Return "snake", "kebab", "undecided", or "invalid" depending on the path style.
function computePathStyle(path) {
    let style = "undecided"
    let allowCaps = true
    for (let i = 0; i < path.length; ++i) {
        let code = path.charCodeAt(i)
        if (code >= 65 && code <= 90) {
            // uppercase ASCII
            if (allowCaps) continue
            return "invalid"
        }
        else if (code >= 97 && code <= 122) {
            // lowercase ASCII
            allowCaps = false
            continue;
        }
        else if (code >= 48 && code <= 57) {
            // ASCII digit
            continue;
        }
        else if (code == 46) {
            // ASCII period
            continue;
        }
        else if (code == 45) {
            // ASCII hyphen
            if (style == "snake") return "invalid"
            style = "kebab"
        }
        else if (code == 95) {
            // ASCII underscore
            if (style == "kebab") return "invalid"
            style = "snake"
        }
        else {
            return "invalid"
        }
    }
    return style
}

function checkDirectory(workspaceRoot, directory) {
    let contents = readdirSync(join(workspaceRoot, directory))
    for (let i = 0; i < contents.length; ++i) {
        let style = computePathStyle(contents[i])
        if (style == "invalid") {
            throw new Error(`Invalid path style: ${join(directory, contents[i])}`)
        }
        else if (directory.startsWith("rust")) {
            if (style == "kebab") {
                throw new Error(`Unexpected kebab case path: ${join(directory, contents[i])}`)
            }
        }
        else if (style == "snake") {
            throw new Error(`Unexpected snake case path: ${join(directory, contents[i])}`)
        }
        if (contents[i].startsWith(".")) continue
        if (directory === "" && (contents[i] === "target" || contents[i] === "node_modules")) continue
        let joinedPath = join(directory, contents[i])
        let stats = lstatSync(joinedPath)
        if (stats.isDirectory() && !stats.isSymbolicLink()) {
            checkDirectory(workspaceRoot, joinedPath)
        }
    }
}

checkDirectory(realpathSync(process.argv[2]), "")
