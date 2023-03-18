import { existsSync, mkdirSync } from "fs"
import { getPath } from "./get-path"

type Binary = {
    name: string
    directory: string
}

const binaries: Binary[] = [
    {
        name: "mjolnir",
        directory: "packages/mjolnir",
    },
    {
        name: "thor",
        directory: "packages/thor",
    },
]

const targetsProcess = Bun.spawnSync(["bkg", "--targets"])
// Why is this stderr instead of stdout? No idea. That's just what bkg outputs it.
const targets = targetsProcess.stderr
    .toString()
    .trim()
    .split("\n")
    .map((target) => target.trim())

// ensures the output folder exists
if (!existsSync(getPath(".releases"))) {
    mkdirSync(getPath(".releases"))
}

for (const binary of binaries) {
    for (const target of targets) {
        console.log(`Building ${binary.name} for ${target}...`)
        const buildProcess = Bun.spawnSync([
            // The CLI tool that generates the binaries.
            "bkg",
            // The directory that contains the binary's source code.
            getPath(binary.directory),
            // The output file.
            "-o",
            getPath(`.releases/${binary.name}-${target}`),
            // The target platform.
            "-t",
            target,
        ])
        if (buildProcess.exitCode !== 0) {
            console.error(buildProcess.stderr.toString())
            process.exit(1)
        }
    }
}
